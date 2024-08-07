use poise::CreateReply;
use serenity::builder::EditChannel;

use crate::{
    constants::STATUS_CHANNEL_ID,
    data::{Context, Result},
};

#[poise::command(
    slash_command,
    guild_only,
    subcommands("stabilize", "error"),
    default_member_permissions = "MANAGE_CHANNELS"
)]
pub async fn status(_: Context<'_>) -> Result {
    Ok(())
}

/// Set the status to `Operational`
#[poise::command(slash_command, guild_only)]
pub async fn stabilize(ctx: Context<'_>) -> Result {
    ctx.defer().await?;
    let edited = STATUS_CHANNEL_ID
        .edit(&ctx.http(), EditChannel::new().name("🟢｜Status: Operational"))
        .await;

    if let Err(e) = edited {
        ctx.send(CreateReply::new().content(format!("Failed to edit channel: {}", e)))
            .await?;
    } else {
        ctx.send(CreateReply::new().content("Status set to `Operational`"))
            .await?;
    }

    Ok(())
}

/// Set the status to error
#[poise::command(slash_command, guild_only)]
pub async fn error(ctx: Context<'_>, #[description = "status name (example: Error)"] name: String) -> Result {
    ctx.defer().await?;
    let edited = STATUS_CHANNEL_ID
        .edit(&ctx.http(), EditChannel::new().name(format!("🔴｜Status: {}", &name)))
        .await;

    if let Err(e) = edited {
        ctx.send(CreateReply::new().content(format!("Failed to edit channel: {}", e)))
            .await?;
    } else {
        ctx.send(CreateReply::new().content(format!("Status set to `{}`", &name)))
            .await?;
    }

    Ok(())
}
