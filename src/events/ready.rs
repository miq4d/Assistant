use serenity::{all::Ready, prelude::Context};

use crate::data::FrameworkContext;

pub async fn ready(ctx: &Context, data_about_bot: &Ready, framework: &FrameworkContext<'_>) {
    log::info!("Logged in as {}", data_about_bot.user.name);

    let commands = &framework.options().commands;
    poise::builtins::register_globally(&ctx.http, &commands)
        .await
        .unwrap();

    log::info!("Successfully registered slash commands!");
}
