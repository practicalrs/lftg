use crate::{Result, config::Config, ollama::{self, Message}};
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "You are a writer who specializes in writing books for children.
The story should align with Christopher Vogler's Hero's Journey framework.
The story should have the following events:
- ordinary world
- call to adventure
- refusal of the call
- meeting with the mentor
- crossing the first threshold
- tests, allies, and enemies
- approach to the inmost cave
- the ordeal
- reward
- the road back
- the resurrection
- return with the elixir
Story needs to follow Robert McKee's story structure framework.
It should have 6 parts that can be regarded as:
- inciting incident
- progressive complications
- turning point progressive complications
- crysis question
- climax
- resolution
Dialog writing should follow Robert McKee's framework for dialogs.
Dialogs should have 5 parts that can be regarded as:
- desire
- sense of antagonism
- choice of action
- action/reaction
- expression
";

pub async fn generate(config: Arc<Config>) -> Result<()> {
    println!("config = {:?}", config);

    let mut messages = vec![];

    let system_prompt = SYSTEM_PROMPT.to_string();
    let message = Message {
        content: system_prompt,
        role: "system".to_string(),
    };
    messages.push(message);

    let prompt = format!(r#"You will outline a short fairy tale. Write a story about Carrie and her pink dragon. Robert McKee's story structure framework. Respond in the following JSON format ```json [{{"story_structure_part": string, "outline": string}}]```. Make sure JSON is valid."#);
    let message = Message {
        content: prompt,
        role: "user".to_string(),
    };
    messages.push(message);

    let response = ollama::request(config.clone(), messages.clone()).await?;

    println!("response = {response}");

    Ok(())
}
