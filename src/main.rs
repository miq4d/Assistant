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
use rustls::crypto::aws_lc_rs;
use serenity::{Client, all::Token, builder::CreateAllowedMentions};
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;
use tracing_subscriber::EnvFilter;

use crate::{
    commands::{
        adminping, btp, manage, modping, purge, shard, status,
        tags::{en, modify},
        test, translate,
    },
    constants::PREFIX,
    data::{Handler, SharedData, get_intents},
};

#[cfg(feature = "admin")]
use crate::commands::admin;
#[cfg(feature = "runjs")]
use crate::commands::runjs;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let default_filter = if cfg!(debug_assertions) {
        "assistant=trace"
    } else {
        "assistant=info"
    };
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));
    let log_format = env::var("LOG_FORMAT").unwrap_or_default();
    let subscriber = tracing_subscriber::fmt().with_env_filter(env_filter);
    if log_format.eq_ignore_ascii_case("json") {
        subscriber
            .json()
            .flatten_event(true)
            .with_current_span(false)
            .with_span_list(false)
            .init();
    } else {
        subscriber.init();
    }

    aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    tracing::info!("Starting...");

    let token = Token::from_env("DISCORD_TOKEN").expect("Expected a token in the environment");

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
        allowed_mentions: Some(
            CreateAllowedMentions::new()
                .replied_user(false)
                .everyone(false),
        ),
        prefix_options: PrefixFrameworkOptions {
            prefix: Some(PREFIX.into()),
            // Forum post bodies are dispatched as thread creation messages.
            // Keep prefix tag commands available there.
            ignore_thread_creation: false,
            mention_as_prefix: false,
            ..Default::default()
        },
        ..Default::default()
    });

    let mut client = Client::builder(token, intents)
        .framework(Box::new(frame))
        .event_handler(Arc::new(Handler))
        .data(Arc::new(SharedData {
            mentioned: Mutex::new(HashMap::new()),
            #[cfg(feature = "db")]
            db: create_pool().await.expect("Failed to create pool"),
        }))
        .await
        .unwrap();

    client.start().await.unwrap();
}
