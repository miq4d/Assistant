use std::time::Duration;

use serenity::{
    all::{
        ChannelType, CreateAllowedMentions, CreateMessage, Message, MessageReference,
        MessageReferenceKind,
    },
    prelude::Context,
};

use tokio::time::sleep;

use crate::{
    constants::{HTQ_CHANNEL_ID, MENTION_REGEX, MOD_CHANNEL_ID, TRAP_CHANNEL_ID},
    data::SharedData,
};

pub async fn message(ctx: &Context, message: &Message) {
    if message.author.bot() {
        return;
    }

    if message.channel_id == TRAP_CHANNEL_ID.widen() {
        handle_trap_message(ctx, message).await;
        return;
    }

    let channel = message.channel(&ctx).await.unwrap();

    if let Some(guild) = channel.guild() {
        // Auto-Publish Announce
        if guild.base.kind == ChannelType::News {
            message.crosspost(&ctx.http).await.unwrap();
        }
    }

    let data = ctx.data::<SharedData>();

    // Mentioned
    if MENTION_REGEX.is_match(&message.content) && message.referenced_message.is_none() {
        tracing::debug!("Mentioned");
        let mut mentioned = data.mentioned.lock().await;
        let time = mentioned.get(&message.author.id);
        if let Some(time) = time {
            if time + 60000 > message.timestamp.timestamp_millis() as u64 {
                sleep(Duration::from_secs(1)).await;
                message
                    .reply(
                        &ctx.http,
                        format!(
                            "<@{}>, How to Quote: <#{}>",
                            message.author.id.get(),
                            HTQ_CHANNEL_ID.get()
                        ),
                    )
                    .await
                    .unwrap();
            } else {
                mentioned.insert(
                    message.author.id,
                    message.timestamp.timestamp_millis() as u64,
                );
            }
        } else {
            mentioned.insert(
                message.author.id,
                message.timestamp.timestamp_millis() as u64,
            );
        }
    }
}

async fn handle_trap_message(ctx: &Context, message: &Message) {
    let Some(guild_id) = message.guild_id else {
        tracing::warn!(
            user_id = message.author.id.get(),
            message_id = message.id.get(),
            "skipping trap automation outside of a guild"
        );
        return;
    };

    let mut forward = MessageReference::new(MessageReferenceKind::Forward, message.channel_id)
        .message_id(message.id);
    forward = forward.guild_id(guild_id);

    let builder = CreateMessage::new()
        .allowed_mentions(
            CreateAllowedMentions::new()
                .all_users(false)
                .all_roles(false)
                .everyone(false)
                .replied_user(false),
        )
        .reference_message(forward);

    // Keep evidence in the moderation channel before banning the sender.
    if let Err(error) = MOD_CHANNEL_ID
        .widen()
        .send_message(&ctx.http, builder)
        .await
    {
        tracing::error!(
            user_id = message.author.id.get(),
            message_id = message.id.get(),
            ?error,
            "failed to forward trap message"
        );
        return;
    }

    if let Err(error) = guild_id
        .ban(
            &ctx.http,
            message.author.id,
            86400 * 3,
            Some("Automatically banned after posting in the trap channel"),
        )
        .await
    {
        tracing::error!(
            user_id = message.author.id.get(),
            message_id = message.id.get(),
            ?error,
            "failed to ban trap message author"
        );
        return;
    }

    tracing::info!(
        user_id = message.author.id.get(),
        message_id = message.id.get(),
        "forwarded trap message and banned author"
    );
}
