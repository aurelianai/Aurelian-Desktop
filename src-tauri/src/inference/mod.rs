mod prompt_processing;
use std::convert::Infallible;
use std::io::Write;

use crate::inference::prompt_processing::generate_prompt;
use crate::persistence::message::Message;
use crate::AppState;
use rand::thread_rng;
use tauri::State;

/// Loads the default model, hardcoded for now
/// Returns if model is already loaded
#[tauri::command]
pub fn load_default_model(state: State<AppState>) -> Result<(), ()> {
	let mut loaded_model_ptr = state.model.lock().unwrap();

	let model_loaded = (*loaded_model_ptr).is_some();

	if !model_loaded {
		let model = llm::load::<llm::models::Llama>(
			std::path::Path::new("C:\\Users\\Ethan\\Downloads\\llama-2-7b-chat.ggmlv3.q4_0.bin"),
			llm::TokenizerSource::Embedded,
			Default::default(),
			llm::load_progress_callback_stdout,
		)
		.unwrap();

		*loaded_model_ptr = Some(Box::new(model));
	}

	Ok(())
}

/// Due to my rust skill issues with memory being moved (idk), load_default_model must be called from the frontend BEFORE calling complete
#[tauri::command(rename_all = "snake_case")]
pub async fn complete(state: State<'_, AppState>, msgs: Vec<Message>) -> Result<(), String> {
	println!("I made it to the complete function!");

	let prompt = generate_prompt(msgs).map_err(|_e| String::from("Error Formatting Prompt"))?;

	let Some(ref model) = *state.model.lock().unwrap() else { return Err(String::from("Model is not loaded")) };

	let mut session = model.start_session(Default::default());

	println!("Starting Inference!");
	session
		.infer::<Infallible>(
			model.as_ref(),
			&mut thread_rng(),
			&llm::InferenceRequest {
				prompt: llm::Prompt::Text(&prompt),
				parameters: &llm::InferenceParameters::default(),
				play_back_previous_tokens: false,
				maximum_token_count: None,
			},
			&mut Default::default(),
			|r| match r {
				llm::InferenceResponse::InferredToken(t) => {
					print!("{t}");
					std::io::stdout().flush().unwrap();
					Ok(llm::InferenceFeedback::Continue)
				}
				_ => Ok(llm::InferenceFeedback::Continue),
			},
		)
		.map_err(|e| e.to_string())?;

	Ok(())
}
