mod commands;
mod constants;
mod data;
#[cfg(feature = "db")]
mod db;
mod events;
mod helper;
mod structs;

#[cfg(feature = "db")]
use db::create_pool;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use serenity::{builder::CreateAllowedMentions, Client};
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    commands::{
        adminping, btp, manage, modping, purge, shard, status,
        tags::{en, modify},
        test, translate,
    },
    constants::PREFIX,
    data::{get_intents, SharedData},
};

#[cfg(feature = "admin")]
use crate::commands::admin;
#[cfg(feature = "runjs")]
use crate::commands::runjs;

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

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = get_intents();

    let commands = vec![
        btp::btp(),
        purge::purge(),
        status::status(),
        en::en(),
        modify::tag(),
        #[cfg(feature = "runjs")]
        runjs::runjs(),
        translate::tja(),
        shard::shard(),
        test::test(),
        manage::manage(),
        modping::modping(),
        adminping::adminping(),
        #[cfg(feature = "admin")]
        admin::admin(),
    ];

    let frame = Framework::new(FrameworkOptions {
        commands,
        event_handler: |framework, event| Box::pin(data::event_handler(framework, event)),
        allowed_mentions: Some(
            CreateAllowedMentions::new()
                .replied_user(false)
                .everyone(false),
        ),
        prefix_options: PrefixFrameworkOptions {
            prefix: Some(PREFIX.into()),
            mention_as_prefix: false,
            ..Default::default()
        },
        ..Default::default()
    });

    let mut client = Client::builder(&token, intents)
        .framework(frame)
        .data(Arc::new(SharedData {
            mentioned: Mutex::new(HashMap::new()),
            #[cfg(feature = "db")]
            db: create_pool().await.expect("Failed to create pool"),
        }))
        .await
        .unwrap();

    client.start().await.unwrap();
}
