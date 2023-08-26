use rocket::{
	get,
	response::stream::{Event, EventStream},
	routes,
	serde::{json::Json, Deserialize, Serialize},
	tokio::sync::mpsc,
	State,
};
use std::{
	convert::Infallible,
	sync::{Arc, Mutex},
	thread,
};

pub struct ModelManager {
	pub model: Box<dyn llm::Model>,
	pub session: llm::InferenceSession,
}

impl ModelManager {
	pub fn infer<F>(&mut self, prompt: &str, callback: F) -> Result<llm::InferenceStats, String>
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

#[tauri::command]
pub async fn start_server() {
   tauri::async_runtime::spawn( async move {
	let model = llm::load_dynamic(
		Some(llm::ModelArchitecture::Llama),
		std::path::Path::new("C:\\Users\\Ethan\\Downloads\\llama-2-7b-chat.ggmlv3.q2_K.bin"),
		llm::TokenizerSource::Embedded,
		Default::default(),
		llm::load_progress_callback_stdout,
	)
	.expect("failed to load model");

	let session = model.start_session(Default::default());

	let manager = ModelManager { model, session };

	let _ = rocket::build()
		.manage(Arc::new(Mutex::new(manager)))
		.mount("/", routes![health, complete])
		.launch().await;
}
   );
}

#[get("/health")]
async fn health() -> &'static str {
	"Service Ready to Accept Traffic!"
}

#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct CompletionRequest<'a> {
	prompt: &'a str,
}

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct CompletionUpdate {
	delta: String,
	err: Option<String>,
}

#[get("/complete", data = "<prompt>")]
async fn complete<'r>(
	s: &'r State<Arc<Mutex<ModelManager>>>,
	prompt: Json<CompletionRequest<'r>>,
) -> EventStream![Event + 'r] {
	// TODO look into buffer sizes. This may be overkill or potentially not enough ?
	let (tx, mut rx) = mpsc::channel::<CompletionUpdate>(500);

	let worker_ref = Arc::clone(s);
	let prompt = String::from(prompt.prompt);

	thread::spawn(move || {
		let ref mut manager = *worker_ref
			.lock()
			.expect("Mutex poisoned launching thread for generation");

		manager
			.infer(&prompt, move |r| match r {
				llm::InferenceResponse::InferredToken(t) => {
					if let Err(_) = tx.blocking_send(CompletionUpdate {
						delta: t,
						err: None,
					}) {
						Ok(llm::InferenceFeedback::Halt)
					} else {
						Ok(llm::InferenceFeedback::Continue)
					}
				}
				_ => Ok(llm::InferenceFeedback::Continue),
			})
			.unwrap();
	});

	let ev_stream = EventStream! {
		while let Some(up) = rx.recv().await {
			yield Event::json(&up);
		}
	};

	EventStream::from(ev_stream).heartbeat(None)
}
