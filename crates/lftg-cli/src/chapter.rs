use crate::{config::Config, extract, ollama::{self, Message}, story, Result};
use std::sync::Arc;

pub async fn generate(config: Arc<Config>, outline: &str) -> Result<()> {
    let mut messages = vec![];

    let message = Message {
        content: story::SYSTEM_PROMPT.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    let message = Message {
        content: story::SYSTEM_PROMPT_RMK_STRUCTURE.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    let message = Message {
        content: story::SYSTEM_PROMPT_HEROES_JOURNEY_EVENTS.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    let message = Message {
        content: story::SYSTEM_PROMPT_HEROES_JOURNEY_ARCHETYPES.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    let prompt = format!(r#"You will outline a short fairy tale.
You will use all provided frameworks.
Use Robert McKee's story structure framework for story structure.
Try to use events and archetypes from Christopher Vogler's Hero's Journey framework.
But structure should be based on Robert McKee's framework. So you should return 6 parts.
Short story will be around 2000 words.
Respond in the following JSON format ```json [{{"story_structure_part": string, "outline": string}}]```. Make sure JSON is valid.
{outline}"#);
    let message = Message {
        content: prompt,
        role: "user".to_string(),
    };
    messages.push(message);

    let response = ollama::request(config.clone(), messages.clone()).await?;
    let json = extract::extract_json(&response);
    println!("json = {json}");

    Ok(())
}
