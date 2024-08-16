use serenity::{
    all::{ChannelFlags, User},
    builder::EditThread,
    prelude::Context,
};

use crate::constants::{EN_FORUM_CHANNEL_ID, GUILD_ID, JA_FORUM_CHANNEL_ID, MEMBER_LOG_CHANNEL_ID};

pub async fn member_removal(ctx: &Context, user: &User) {
    log::info!("{} has left the server.", user.name);
    MEMBER_LOG_CHANNEL_ID
        .say(
            &ctx.http,
            format!("{} (<@{}>) has left.", user.name, user.id.get()),
        )
        .await
        .expect("Failed to send message to member log channel.");

    let threads = &GUILD_ID
        .get_active_threads(&ctx.http)
        .await
        .unwrap()
        .threads;
    let forum_threads = threads
        .iter()
        .filter(|c| {
            c.parent_id.map(|f| f.get() == EN_FORUM_CHANNEL_ID.get()
                            || f.get() == JA_FORUM_CHANNEL_ID.get())
                .unwrap_or(false)
        })
        .filter(|c| {
            !c.flags.contains(ChannelFlags::PINNED)
                && !c.thread_metadata.unwrap().locked()
                && !c.thread_metadata.unwrap().archived()
        })
        .collect::<Vec<_>>();

    let own_threads = forum_threads
        .iter()
        .filter(|c| c.owner_id.unwrap().get() == user.id.get())
        .map(|f| f.to_owned())
        .collect::<Vec<_>>();

    for thread in own_threads {
        let mut thread = thread.to_owned();
        thread
            .say(&ctx.http, {
                if thread.parent_id.unwrap().get() == EN_FORUM_CHANNEL_ID.get() {
                    "The thread owner has left the server. This thread will be locked immediately."
                } else {
                    "スレッドのオーナーがサーバーを退出したため、このスレッドはロックされます。"
                }
            })
            .await
            .ok();
        thread
            .edit_thread(
                &ctx.http,
                EditThread::new()
                    .locked(true)
                    .archived(true)
                    .audit_log_reason("Thread owner left the server."),
            )
            .await
            .unwrap();
    }
}
