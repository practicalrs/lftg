use crate::{chapter, config::Config, Result};
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "You are a writer who specializes in writing books for children.
You will get specific instructions regarding frameworks that you should use for your writing.";
pub const SYSTEM_PROMPT_HEROES_JOURNEY_ARCHETYPES: &str = "The story should align with Christopher Vogler's Hero's Journey framework.
The story may use the following archetypes:
- hero
- mentor
- threshold guardian
- herald
- shapeshifter
- shadow
- ally
- trickster";
pub const SYSTEM_PROMPT_HEROES_JOURNEY_EVENTS: &str = "The story should align with Christopher Vogler's Hero's Journey framework.
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
- return with the elixir";
pub const SYSTEM_PROMPT_RMK_DIALOG: &str = "Dialog writing should follow Robert McKee's framework for dialogs.
Dialogs should have 5 parts:
- desire
- a sense of antagonism
- choice of action
- action/reaction
- expression";
pub const SYSTEM_PROMPT_RMK_STRUCTURE: &str = "The story needs to follow Robert McKee's story structure framework.
It should have 6 parts that can be regarded as:
- inciting incident
- progressive complications
- turning point progressive complications
- crysis question
- climax
- resolution";

pub async fn generate(config: Arc<Config>) -> Result<()> {
    chapter::generate(config, "Write a story about Carrie and her pink dragon.").await?;

    Ok(())
}
