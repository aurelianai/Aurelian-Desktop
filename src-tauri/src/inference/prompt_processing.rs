use crate::persistence::message::Message;
struct PromptConfig {
	pub user_prefix: String,
	pub user_suffix: String,
	pub model_prefix: String,
	pub model_suffix: String,
}

/// Generates prompt from messages.
/// TODO cut off messages if they go out of context
pub fn generate_prompt(msgs: Vec<Message>) -> Result<String, ()> {
	// Hardcoded for llama wo finetune
	let prompt_config = PromptConfig {
		user_prefix: String::from("<s>[INST]\n"),
		user_suffix: String::from(" [/INST]"),
		model_prefix: String::from(" "),
		model_suffix: String::from("</s>"),
	};

	let mut prompt = String::from("");

	for msg in msgs {
		if msg.role == "USER" {
			prompt.push_str(&prompt_config.user_prefix);
		} else {
			prompt.push_str(&prompt_config.model_prefix);
		}

		prompt.push_str(&msg.content);

		if msg.role == "USER" {
			prompt.push_str(&prompt_config.user_suffix);
		} else {
			prompt.push_str(&prompt_config.model_suffix);
		}
	}

	Ok(prompt)
}
