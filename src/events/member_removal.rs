use serenity::{all::User, builder::EditThread, prelude::Context};

use crate::constants::{EN_FORUM_CHANNEL_ID, GUILD_ID, JA_FORUM_CHANNEL_ID, MEMBER_LOG_CHANNEL_ID};

pub async fn member_removal(ctx: &Context, user: &User) {
    log::info!("{} has left the server.", user.name);
    MEMBER_LOG_CHANNEL_ID
        .widen()
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
            c.parent_id.get() == EN_FORUM_CHANNEL_ID.get()
                || c.parent_id.get() == JA_FORUM_CHANNEL_ID.get()
        })
        .filter(|c| !c.thread_metadata.locked() && !c.thread_metadata.archived())
        .collect::<Vec<_>>();

    let own_threads = forum_threads
        .iter()
        .filter(|c| c.owner_id.get() == user.id.get())
        .map(|f| f.to_owned())
        .collect::<Vec<_>>();

    for thread in own_threads {
        let mut thread = thread.to_owned();
        thread
            .id
            .widen()
            .say(&ctx.http, {
                if thread.parent_id.get() == EN_FORUM_CHANNEL_ID.get() {
                    "The thread owner has left the server. This thread will be locked immediately."
                } else {
                    "スレッドのオーナーがサーバーを退出したため、このスレッドはロックされます。"
                }
            })
            .await
            .ok();
        thread
            .edit(
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
