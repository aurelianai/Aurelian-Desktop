use serde::{Deserialize, Serialize};
use tauri::Window;

#[derive(Serialize, Deserialize)]
pub struct InferenceUpdate {
	pub delta: String,
	pub err: Option<String>,
}

impl InferenceUpdate {
	pub fn send(&self, window: &Window) {
		if let Err(_) = window.emit("inference-update", self) {
			panic!("Error Loading");
		}
	}
}
