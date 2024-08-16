#![allow(unused_imports)]
#![allow(unused_variables)]
use serenity::all::ChannelFlags;

use crate::{
    constants::{EN_FORUM_CHANNEL_ID, GUILD_ID, JA_FORUM_CHANNEL_ID},
    data::{Context, Result},
};

#[poise::command(prefix_command, guild_only, owners_only)]
pub async fn test(ctx: Context<'_>) -> Result {
    Ok(())
}
