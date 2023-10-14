use serenity::{all::{Message, ChannelType}, prelude::Context};

pub async fn message(ctx: &Context, message: &Message) {
    let channel = message.channel(&ctx).await.unwrap().guild().unwrap();

    // Auto-Publish Announce
    if channel.kind == ChannelType::News && !message.author.bot {
        message.crosspost(&ctx).await.unwrap();
    }

    if message.author.bot {
        return;
    }
}
