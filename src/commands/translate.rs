use std::env;

use serenity::all::Message;
use serde::{Serialize, Deserialize};

use crate::data::{Result, Context};

#[derive(Serialize, Debug)]
struct DeepLRequest {
    text: Vec<String>,
    target_lang: String,
}

#[derive(Deserialize, Debug)]
struct DeepLResponse {
    translations: Vec<Translation>,
}

#[derive(Deserialize, Debug)]
struct Translation {
    text: String,
    #[allow(dead_code)]
    detected_source_language: Option<String>,
}

#[poise::command(context_menu_command = "Translate to Japanese", ephemeral, guild_only)]
pub async fn tja(ctx: Context<'_>, message: Message) -> Result {
    let content = message.content_safe(ctx.cache());

    if content.is_empty() {
        ctx.reply("This message has no text").await?;
    }

    ctx.defer_ephemeral().await?;

    let client = reqwest::Client::new();

    let res = client
        .post("https://api-free.deepl.com/v2/translate")
        .header("Authorization", format!("DeepL-Auth-Key {}", env::var("DEEPL_KEY").expect("Set DeepL Key!")))
        .json(&DeepLRequest {
            text: vec![content.to_string()],
            target_lang: "JA".to_string(),
        })
        .send()
        .await?
        .json::<DeepLResponse>()
        .await?;

    ctx.reply(&res.translations[0].text).await?;
    
    Ok(())
}