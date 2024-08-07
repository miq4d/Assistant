#![allow(unused_imports)]
#![allow(unused_variables)]
use poise::CreateReply;
use serenity::all::ChannelFlags;

use crate::{
    constants::{EN_FORUM_CHANNEL_ID, GUILD_ID, JA_FORUM_CHANNEL_ID, TOTAL_SHARDS},
    data::{Context, Result},
};

/// Calculate the shard number of a guild.
#[poise::command(slash_command)]
pub async fn shard(
    ctx: Context<'_>,
    #[description = "The guild ID."]
    #[min_length = 10]
    #[max_length = 21]
    guild_id: String,
    #[description = "Whether the returned response should be ephemeral."] ephemeral: Option<bool>,
    #[description = "Total shard number to calculate shards. Usually you don't have to specify this value."]
    #[min = 1]
    #[max = 100000]
    total_shards: Option<u64>,
) -> Result {
    let guild_id = guild_id.parse::<u64>()?;
    let total_shards = total_shards.unwrap_or(TOTAL_SHARDS);

    let shard = (guild_id >> 22) % total_shards;

    ctx.send(
        CreateReply::new()
            .content(format!("Guild ID {} is on shard {}", guild_id, shard))
            .ephemeral(ephemeral.unwrap_or(false)),
    )
    .await?;

    Ok(())
}
