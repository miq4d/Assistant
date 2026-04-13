use std::collections::HashMap;

use serenity::{
    all::{ConnectionStage, EventHandler, FullEvent, UserId},
    async_trait,
    prelude::GatewayIntents,
};
#[cfg(feature = "db")]
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

use crate::events::{member_removal, message, presence, reaction_add, ready};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type SerenityContext = serenity::all::Context;
pub type Context<'a> = poise::Context<'a, SharedData, Error>;
pub type Result = std::result::Result<(), Error>;
#[cfg(feature = "db")]
pub type MiqPool = Pool<Postgres>;

pub struct RemainingArgs;

#[derive(Debug, Default)]
struct MissingRemainingArgs;

impl std::fmt::Display for MissingRemainingArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("missing remaining arguments")
    }
}

impl std::error::Error for MissingRemainingArgs {}

#[async_trait]
impl<'a> poise::PopArgument<'a> for RemainingArgs {
    async fn pop_from(
        args: &'a str,
        attachment_index: usize,
        _ctx: &SerenityContext,
        _msg: &serenity::all::Message,
    ) -> std::result::Result<
        (&'a str, usize, Self),
        (Box<dyn std::error::Error + Send + Sync>, Option<String>),
    > {
        let args = args.trim_start();
        if args.is_empty() {
            return Err((MissingRemainingArgs.into(), None));
        }

        let _ = args;
        Ok(("", attachment_index, Self))
    }
}

#[derive(Debug)]
pub struct SharedData {
    pub mentioned: Mutex<HashMap<UserId, u64>>,
    #[cfg(feature = "db")]
    pub db: MiqPool,
}

pub struct Handler;

#[inline]
pub fn get_intents() -> GatewayIntents {
    let mut intents = GatewayIntents::empty();
    intents.insert(GatewayIntents::GUILDS);
    intents.insert(GatewayIntents::GUILD_MESSAGES);
    intents.insert(GatewayIntents::GUILD_MEMBERS);
    intents.insert(GatewayIntents::GUILD_PRESENCES);
    intents.insert(GatewayIntents::GUILD_MESSAGE_REACTIONS);
    intents.insert(GatewayIntents::MESSAGE_CONTENT);
    intents
}

#[async_trait]
impl EventHandler for Handler {
    async fn dispatch(&self, ctx: &SerenityContext, event: &FullEvent) {
        match event {
            FullEvent::Resume { event: _, .. } => {
                tracing::info!("Shard: {} has resumed.", ctx.shard_id);
            }
            FullEvent::ShardStageUpdate { event, .. } => {
                if event.new.is_connecting() {
                    tracing::info!("Shard: {} is connecting.", ctx.shard_id);
                } else if event.new == ConnectionStage::Disconnected {
                    tracing::info!("Shard: {} has disconnected.", ctx.shard_id);
                }
            }
            FullEvent::Ready { data_about_bot, .. } => ready::ready(data_about_bot).await,
            FullEvent::Message { new_message, .. } => message::message(ctx, new_message).await,
            FullEvent::PresenceUpdate {
                old_data: _,
                new_data,
                ..
            } => presence::presence(ctx, new_data).await,
            FullEvent::GuildMemberRemoval {
                guild_id: _,
                user,
                member_data_if_available: _,
                ..
            } => member_removal::member_removal(ctx, user).await,
            FullEvent::ReactionAdd { add_reaction, .. } => {
                reaction_add::reaction_add(ctx, add_reaction).await
            }
            _ => {}
        }
    }
}
