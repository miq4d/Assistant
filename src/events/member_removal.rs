use serenity::{
    all::{ChannelType, User},
    prelude::Context,
};

use crate::constants::{GUILD_ID, MEMBER_LOG_CHANNEL_ID};

pub async fn member_removal(ctx: &Context, user: &User) {
    log::info!("{} has left the server.", user.name);
    MEMBER_LOG_CHANNEL_ID
        .say(&ctx.http, format!("{} has left.", user.name))
        .await
        .expect("Failed to send message to member log channel.");

    let channels = GUILD_ID
        .channels(&ctx.http)
        .await
        .expect("Failed to get channels.");
    let channels: Vec<_> = channels
        .iter()
        .filter(|f| f.1.kind == ChannelType::PublicThread)
        /*.filter(|f| {
            f.1.parent_id
                .and_then(|c| c.to_channel_cached(&ctx.cache))
                .and_then(|c| c.guild())
                .and_then(|c| Some(c.kind == ChannelType::Forum))
                .unwrap_or(false)
        })*/
        .collect();

    dbg!(&channels);
}
