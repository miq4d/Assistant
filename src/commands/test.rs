#![allow(unused_imports)]
#![allow(unused_variables)]
use serenity::all::ChannelFlags;

use crate::{data::{Context, Result}, constants::{GUILD_ID, EN_FORUM_CHANNEL_ID, JA_FORUM_CHANNEL_ID}};

#[poise::command(prefix_command, guild_only, owners_only)]
pub async fn test(ctx: Context<'_>) -> Result {
    
    Ok(())
}