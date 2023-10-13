use serenity::{
    all::{OnlineStatus, Presence},
    builder::CreateMessage,
    prelude::Context,
};

use crate::constants::{ADMIN_ROLE_ID, DOWN_REPORT_CHANNEL_ID, MAIN_BOT_ID};

pub async fn presence(ctx: &Context, new_data: &Presence) {
    if new_data.user.id.get() != MAIN_BOT_ID.get() {
        return;
    }
    if new_data.client_status.is_none() {
        return;
    }
    let statuses = new_data.client_status.as_ref().unwrap();
    if statuses.desktop.unwrap_or(OnlineStatus::Offline) == OnlineStatus::Offline
        && statuses.mobile.unwrap_or(OnlineStatus::Offline) == OnlineStatus::Offline
        && statuses.web.unwrap_or(OnlineStatus::Offline) == OnlineStatus::Offline
    {
        DOWN_REPORT_CHANNEL_ID.send_message(
            ctx,
            CreateMessage::new().content(format!("{} Bot is offline!", ADMIN_ROLE_ID.get())),
        )
        .await
        .unwrap();
    }
}
