use serenity::{prelude::Context, all::Ready};

pub async fn ready(_: &Context, data_about_bot: &Ready) {
    log::info!("Logged in as {}", data_about_bot.user.name);
}