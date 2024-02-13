use poise::CreateReply;
use serenity::{all::User, builder::{GetMessages, CreateEmbed}, model::Colour};

use crate::data::{Context, Result};

/// Purge messages from a channel
#[poise::command(slash_command, guild_only, default_member_permissions = "MANAGE_MESSAGES")]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "The number of messages to purge"]
    #[min = 2]
    #[max = 100]
    amount: u64,
    #[description = "The user to purge messages from"]
    user: Option<User>,
    #[description = "Hide completed messages"]
    hide: Option<bool>,
) -> Result {
    if hide.unwrap_or(false) {
        ctx.defer_ephemeral().await?;
    } else {
        ctx.defer().await?;
    }

    let user = user.map(|u| u.id);

    let messages = ctx
        .channel_id()
        .messages(ctx.http(), GetMessages::new().limit(amount.try_into().unwrap_or(100)))
        .await?;

    let messages = messages
        .iter()
        .filter(|m| {
            if let Some(user) = user {
                m.author.id == user
            } else {
                true
            }
        })
        .map(|m| m.id)
        .collect::<Vec<_>>();

    ctx.channel_id().delete_messages(ctx.http(), &messages).await?;

    ctx.send(CreateReply::new().embed(
        CreateEmbed::new()
            .color(Colour::DARK_GREEN)
            .description(format!("Purged {} messages", &messages.len()))
    )).await?;

    Ok(())
}