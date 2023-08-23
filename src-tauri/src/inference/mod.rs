use llm::Model;

fn load_model(path: string) -> llm::KnownModel {
    llm::load::<llm::models::Llama>(
        std::path::Path::new(""),
        llm::TokenizerSource::Embedded,
        Default::default(),
        llm::load_progress_callback_stdout,
    )
    .unwrap();
}
