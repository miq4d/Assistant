use std::time::Duration;

use serenity::{
    all::{ChannelType, Message},
    prelude::Context,
};

use tokio::time::sleep;

use crate::{
    constants::{HTQ_CHANNEL_ID, MENTION_REGEX},
    data::SharedData,
};

pub async fn message(ctx: &Context, message: &Message) {
    let channel = message.channel(&ctx).await.unwrap().guild().unwrap();

    // Auto-Publish Announce
    if channel.base.kind == ChannelType::News && !message.author.bot() {
        message.crosspost(&ctx.http).await.unwrap();
    }

    if message.author.bot() {
        return;
    }

    let data = ctx.data::<SharedData>();

    // Mentioned
    if MENTION_REGEX.is_match(&message.content) && message.referenced_message.is_none() {
        log::debug!("Mentioned");
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
