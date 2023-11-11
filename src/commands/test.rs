use serenity::all::ChannelType;

use crate::{data::{Context, Result}, constants::GUILD_ID};

#[poise::command(prefix_command, guild_only, owners_only)]
pub async fn test(ctx: Context<'_>) -> Result {
    let channels = GUILD_ID
        .channels(ctx.http())
        .await
        .expect("Failed to get channels.");
    let forum_channels: Vec<_> = channels
        .iter()
        .filter(|f| f.1.kind == ChannelType::Forum)
        .collect();
    
    let forum_threads = forum_channels
        .iter()
        .filter(|f| {
            f.1.posts
        })
        .collect::<Vec<_>>();

    dbg!(&forum_channels);

    Ok(())
}