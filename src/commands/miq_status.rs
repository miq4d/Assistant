use std::{collections::HashMap, env};

use poise::CreateReply;
use serenity::{builder::CreateEmbed, model::Color};
use regex::Regex;
use once_cell::sync::Lazy;

use crate::data::{Context, Result};

static CPU_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Current CPU Usage</div><div.*?>(\d\d?\.?\d?\d?)%").unwrap());
static RAM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?s)Current RAM Usage</div>.*?<div.*?("|')>\s?\((\d\d?\.?\d?\d?)%\)"#).unwrap());

/// Get the status of the MiQ VPS
#[poise::command(slash_command, guild_only, rename = "miq-status", global_cooldown = 20)]
pub async fn miq_status(ctx: Context<'_>) -> Result {
    ctx.defer().await?;

    let client = reqwest::Client::new();

    let mut body = HashMap::new();
    body.insert("m", "69");
    body.insert("tx", "aff71d023cc9141e2df4aab475b7cc50");
    body.insert("u", "10080");
    body.insert("v", "");

    let html = client
        .post(format!(
            "{}/auth.php",
            env::var("HETRIX_STATUS_URL").unwrap()
        ))
        .form(&body)
        .send()
        .await?
        .text()
        .await?;

    let cpu_capture = CPU_REGEX.captures(&html).unwrap();
    let cpu_usage = cpu_capture.get(1).unwrap().as_str();
    
    let ram_capture = RAM_REGEX.captures(&html).unwrap();
    let ram_usage = ram_capture.get(2).unwrap().as_str();

    let embed = CreateEmbed::new()
        .title("MiQ VPS Status")
        .description(format!(">>> _CPU Usage:_ {}%\n_RAM Usage:_ {}%", cpu_usage, ram_usage))
        .color(Color::DARK_GREEN);

    ctx.send(CreateReply::new().embed(embed)).await?;

    Ok(())
}
