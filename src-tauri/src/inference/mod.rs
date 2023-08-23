use crate::AppState;
use tauri::State;

/// Loads the default model, hardcoded for now
#[tauri::command]
pub fn load_default_model(state: State<AppState>) -> Result<(), ()> {
    let model = llm::load::<llm::models::Llama>(
        std::path::Path::new("C:\\Users\\Ethan\\Downloads\\llama-2-7b-chat.ggmlv3.q4_0.bin"),
        llm::TokenizerSource::Embedded,
        Default::default(),
        llm::load_progress_callback_stdout,
    )
    .unwrap();

    let mut loaded_model_ptr = state.model.lock().unwrap();
    *loaded_model_ptr = Some(Box::new(model));

    Ok(())
}
