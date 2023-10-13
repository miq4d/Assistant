mod data;
mod events;
mod commands;
mod constants;

use std::env;
use poise::{Framework, FrameworkOptions};
use serenity::{Client, builder::CreateAllowedMentions, all::Command};

use crate::{
    data::get_intents,
    commands::{btp, purge, status},
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    env_logger::builder()
        .filter_module("assistant", {
            if cfg!(debug_assertions) {
                log::LevelFilter::Trace
            } else {
                log::LevelFilter::Info
            }
        })
        .init();

    log::info!("Starting...");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let intents = get_intents();

    let commands = vec![
        btp::btp(),
        purge::purge(),
        status::status()
    ];

    let frame = Framework::new(
        FrameworkOptions {
            commands,
            event_handler: |event, framework, user_data| {
                Box::pin(data::event_handler(event, framework, user_data))
            },
            allowed_mentions: Some(
                CreateAllowedMentions::new()
                    .replied_user(false)
                    .everyone(false)
            ),
            ..Default::default()
        },
        |ctx, _ready, framework| {
            Box::pin(async move {
                let cmd = poise::builtins::create_application_commands(
                    framework.options().commands.as_slice(),
                );
                Command::set_global_commands(&ctx.http, cmd).await?;
                Ok(())
            })
        },
    );

    let mut client = Client::builder(token, intents)
        .framework(frame)
        .await
        .unwrap();

    client.start().await.unwrap();
}