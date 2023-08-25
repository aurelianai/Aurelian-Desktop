mod events;
mod prompt_processing;

use crate::inference::events::InferenceUpdate;
use crate::inference::prompt_processing::generate_prompt;
use crate::persistence::message::Message;
use crate::ModelState;
use std::convert::Infallible;
use std::io::Write;
use tauri::{State, Window};

/// Loads the default model, hardcoded for now
/// Returns if model is already loaded
#[tauri::command]
pub fn load_default_model(state: State<ModelState>) -> Result<(), ()> {
	let loaded_model_ptr = state.0.lock().unwrap();

	let model_loaded = (*loaded_model_ptr).is_some();

	if !model_loaded {
		let model = llm::load_dynamic(
			// This will be pulled in from db when better model management is here
			Some(llm::ModelArchitecture::Llama),
			std::path::Path::new("C:\\Users\\Ethan\\Downloads\\llama-2-7b-chat.ggmlv3.q2_K.bin"),
			llm::TokenizerSource::Embedded,
			Default::default(),
			llm::load_progress_callback_stdout,
		)
		.unwrap();

		let session = model.start_session(Default::default());

		*state.0.lock().unwrap() = Some(ModelManager { model, session })
	}

	Ok(())
}

/// Inspired By https://github.com/clarkmcc/chitchat/blob/main/src-tauri/src/models.rs#L130
pub struct ModelManager {
	pub model: Box<dyn llm::Model>,
	pub session: llm::InferenceSession,
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
pub fn complete(
	state: State<'_, ModelState>,
	window: Window,
	msgs: Vec<Message>,
) -> Result<(), ()> {
	let prompt = match generate_prompt(msgs) {
		Err(_) => {
			InferenceUpdate {
				delta: "".into(),
				err: Some("Error formatting prompt".into()),
			}
			.send(&window);
			return Err(());
		}
		Ok(prompt) => prompt,
	};

	let mut manager_ptr = state.0.lock().unwrap();
	let manager: &mut ModelManager = (*manager_ptr).as_mut().unwrap();

	match manager.complete(&prompt, |r| match r {
		llm::InferenceResponse::InferredToken(t) => {
			print!("{t}");
			std::io::stdout().flush().unwrap();

			InferenceUpdate {
				delta: t,
				err: None,
			}
			.send(&window);

			Ok(llm::InferenceFeedback::Continue)
		}
		_ => Ok(llm::InferenceFeedback::Continue),
	}) {
		Err(e) => {
			InferenceUpdate {
				delta: "".into(),
				err: Some(format!("{}", e)),
			}
			.send(&window);
			return Err(());
		}
		Ok(_) => return Ok(()),
	}
}
