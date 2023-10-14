use std::time::Duration;

use serenity::{
    all::{ChannelType, Message},
    prelude::Context,
};

use tokio::time::sleep;

use crate::{data::SharedData, constants::HTQ_CHANNEL_ID};

pub async fn message(ctx: &Context, message: &Message, data: &SharedData) {
    let channel = message.channel(&ctx).await.unwrap().guild().unwrap();

    // Auto-Publish Announce
    if channel.kind == ChannelType::News && !message.author.bot {
        message.crosspost(&ctx).await.unwrap();
    }

    if message.author.bot {
        return;
    }

    // Mentioned
    if message.mentions_me(&ctx).await.unwrap() {
        if message.referenced_message.is_none() {
            let mut mentioned = data.mentioned.lock().await;
            let time = mentioned.get(&message.author.id);
            if let Some(time) = time {
                if time + 60000 > message.timestamp.timestamp_millis() as u64 {
                    sleep(Duration::from_secs(1)).await;
                    message.reply(&ctx, format!("{}, How to Quote: <#{}>", message.author.id.get(), HTQ_CHANNEL_ID.get())).await.unwrap();
                } else {
                    mentioned.insert(
                        message.author.id,
                        message.timestamp.timestamp_millis() as u64,
                    );
                }
            }
        }
    }
}
