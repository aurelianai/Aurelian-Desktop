mod events;
mod hardware;
mod prompt_processing;

use crate::inference::{
	events::InferenceUpdate, hardware::autodetect_num_threads, prompt_processing::generate_prompt,
};
use crate::persistence::message::Message;
use crate::ModelState;
use std::convert::Infallible;
use std::{
	io::Write,
	sync::{Arc, Mutex},
};
use tauri::{State, Window};

/// Loads the default model, hardcoded for now
/// Returns if model is already loaded
#[tauri::command(rename_all = "snake_case")]
pub async fn load_default_model(state: State<'_, ModelState>, chat_id: i32) -> Result<(), ()> {
	let mut loaded_model_ptr = state.0.lock().unwrap();

	if (*loaded_model_ptr).is_none() {
		let model = llm::load_dynamic(
			// This will be pulled in from db when better model management is here
			Some(llm::ModelArchitecture::Llama),
			std::path::Path::new("C:\\Users\\Ethan\\Downloads\\llama-2-7b-chat.ggmlv3.q2_K.bin"),
			llm::TokenizerSource::Embedded,
			llm::ModelParameters {
				use_gpu: false,     // TODO Detect Automatically, offload optimal number of layers
				context_size: 2048, // TODO create setting for this
				..Default::default()
			},
			llm::load_progress_callback_stdout,
		)
		.unwrap();

		let session = model.start_session(llm::InferenceSessionConfig {
			n_threads: autodetect_num_threads(),
			n_batch: 512, // Default from llama.cpp
			..Default::default()
		});

		*loaded_model_ptr = Some(ModelManager {
			model,
			session,
			chat_id,
		});
	} else {
		let Some(ref mut model_manager) = *loaded_model_ptr else {
			panic!()
		};

		if model_manager.chat_id != chat_id {
			println!("Dropping session for new chat");
			model_manager.session = model_manager
				.model
				.start_session(llm::InferenceSessionConfig {
					n_threads: autodetect_num_threads(),
					n_batch: 512,
					..Default::default()
				})
		}
	}

	Ok(())
}

/// Inspired By https://github.com/clarkmcc/chitchat/blob/main/src-tauri/src/models.rs#L130
pub struct ModelManager {
	pub model: Box<dyn llm::Model>,
	pub session: llm::InferenceSession,
	pub chat_id: i32,
}

impl ModelManager {
	pub fn complete<F>(&mut self, prompt: &str, callback: F) -> Result<llm::InferenceStats, String>
	where
		F: FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible>,
	{
		self.session
			.infer(
				self.model.as_ref(),
				&mut rand::thread_rng(),
				&llm::InferenceRequest {
					prompt: llm::Prompt::Text(prompt),
					parameters: &llm::InferenceParameters::default(),
					play_back_previous_tokens: false,
					maximum_token_count: None,
				},
				&mut Default::default(),
				callback,
			)
			.map_err(|e| e.to_string())
	}
}

/// Due to my rust skill issues with memory being moved (idk), load_default_model must be called from the frontend BEFORE calling complete
#[tauri::command(rename_all = "snake_case")]
pub async fn complete(
	state: State<'_, ModelState>,
	window: Window,
	msgs: Vec<Message>,
) -> Result<(), ()> {
	let prompt = match generate_prompt(msgs) {
		Err(_) => {
			InferenceUpdate {
				delta: "".into(),
				err: Some("Error formatting prompt".into()),
				done: false,
			}
			.send(&window);
			return Err(());
		}
		Ok(prompt) => prompt,
	};

	let cancelled = Arc::new(Mutex::new(false));
	let callback_ref = Arc::clone(&cancelled);

	let ev_id = window.listen("cancel-generation", move |_| {
		*callback_ref.lock().unwrap() = true;
		println!("Got cancellation event");
	});

	let Some(ref mut manager) = *state.0.lock().unwrap() else {
		panic!()
	};

	let stats = manager
		.complete(&prompt, |r| match r {
			llm::InferenceResponse::InferredToken(t) => {
				print!("{t}");
				std::io::stdout().flush().unwrap();

				if *cancelled.lock().unwrap() {
					println!("Halting Generation");
					return Ok(llm::InferenceFeedback::Halt);
				}

				InferenceUpdate {
					delta: t,
					err: None,
					done: false,
				}
				.send(&window);

				Ok(llm::InferenceFeedback::Continue)
			}
			_ => Ok(llm::InferenceFeedback::Continue),
		})
		.unwrap();

	println!("\nINF STATS: {}", stats.to_string());

	InferenceUpdate {
		delta: String::from(""),
		err: None,
		done: true,
	}
	.send(&window);

	window.unlisten(ev_id);
	Ok(())
}
