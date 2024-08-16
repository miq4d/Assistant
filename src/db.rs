#![cfg(feature = "db")]

use std::env;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgPoolOptions,
    types::{Json, JsonValue},
};

use crate::data::MiqPool;

pub async fn create_pool() -> sqlx::Result<MiqPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL not specified"))
        .await?;
    Ok(pool)
}

#[derive(Serialize, Deserialize)]
pub struct DiscordGuild {
    pub guild_id: String,
    pub is_banned: bool,
    pub language: String,
    pub cooldown: i32,
    pub redirect_to: Option<String>,
    pub redirect_leave_original: bool,
    pub always_quote_channels: Vec<String>,
    // 注意: always quote には適用されません
    pub role_lists: Vec<String>,
    // 1: blacklist, 2: whitelist
    pub role_lists_mode: i32,
    pub disable_self_quote: bool,
    pub enable_auto_enable_custom_quote: bool,
    pub quiet_generation: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordUser {
    pub user_id: String,
    pub generate_block: String,
    pub allow_custom_quote: bool,
    pub compact: bool,
    pub is_banned: bool,
    pub default_settings: DiscordUserDefaultSettings,
    pub oauth2: Option<Json<DiscordOAuth2Token>>,
    pub guild_cache: Option<Vec<Json<CachedGuild>>>,
    pub cache_updated_at: Option<NaiveDateTime>,
    pub enable_done: bool,
    pub force_disable_custom_quote: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordUserDefaultSettings {
    pub font: String,
    pub light: bool,
    pub color: bool,
    pub bold: bool,
    pub flip: bool,
    pub new: bool,
}

impl From<JsonValue> for DiscordUserDefaultSettings {
    fn from(json: JsonValue) -> Self {
        DiscordUserDefaultSettings {
            font: json["font"].as_str().unwrap_or("mplus").to_string(),
            light: json["light"].as_bool().unwrap_or(false),
            color: json["color"].as_bool().unwrap_or(false),
            bold: json["bold"].as_bool().unwrap_or(false),
            flip: json["flip"].as_bool().unwrap_or(false),
            new: json["new"].as_bool().unwrap_or(false),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DiscordOAuth2Token {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CachedGuild {
    pub guild_id: String,
    pub name: String,
    pub icon_url: String,
    pub is_joined: bool,
}
