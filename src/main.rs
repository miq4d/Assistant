mod data;
mod events;
mod commands;
mod constants;
mod structs;
mod helper;

use std::{env, sync::Arc, collections::HashMap};
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use serenity::{Client, builder::CreateAllowedMentions};
use tokio::sync::Mutex;

use crate::{
    data::{get_intents, SharedData},
    commands::{btp, purge, status, tags::{en, modify}, runjs, translate, miq_status, test, manage},
    constants::PREFIX,
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
        status::status(),
        en::en(),
        modify::tag(),
        runjs::runjs(),
        translate::tja(),
        miq_status::miq_status(),
        test::test(),
        manage::manage()
    ];

    let frame = Framework::new(
        FrameworkOptions {
            commands,
            event_handler: |framework, event| {
                Box::pin(data::event_handler(framework, event))
            },
            allowed_mentions: Some(
                CreateAllowedMentions::new()
                    .replied_user(false)
                    .everyone(false)
            ),
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(PREFIX.into()),
                mention_as_prefix: false,
                ..Default::default()
            },
            ..Default::default()
        }/*,
        |ctx, _ready, framework| {
            Box::pin(async move {
                let cmd = poise::builtins::create_application_commands(
                    framework.options().commands.as_slice(),
                );
                Command::set_global_commands(&ctx.http, cmd).await?;
                Ok()
            })
        },*/
    );

    let mut client = Client::builder(&token, intents)
        .framework(frame)
        .data(Arc::new(SharedData {
            mentioned: Mutex::new(HashMap::new())
        }))
        .await
        .unwrap();

    client.start().await.unwrap();
}