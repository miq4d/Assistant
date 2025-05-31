use std::collections::HashMap;

use serenity::{
    all::{EventHandler, FullEvent, UserId},
    async_trait,
    gateway::ConnectionStage,
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
                log::info!("Shard: {} has resumed.", ctx.shard_id);
            }
            FullEvent::ShardStageUpdate { event, .. } => {
                if event.new.is_connecting() {
                    log::info!("Shard: {} is connecting.", ctx.shard_id);
                } else if event.new == ConnectionStage::Disconnected {
                    log::info!("Shard: {} has disconnected.", ctx.shard_id);
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
