use std:: collections::HashMap;

use serenity::{prelude::GatewayIntents, client::FullEvent, gateway::ConnectionStage, all::UserId};
use tokio::sync::Mutex;

use crate::events::{ready, message, presence, member_removal};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, SharedData, Error>;
pub type Result = std::result::Result<(), Error>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, SharedData, Error>;

#[derive(Debug)]
pub struct SharedData {
    pub mentioned: Mutex<HashMap<UserId, u64>>
}

#[inline]
pub fn get_intents() -> GatewayIntents {
    let mut intents = GatewayIntents::empty();
    intents.insert(GatewayIntents::GUILDS);
    intents.insert(GatewayIntents::GUILD_MESSAGES);
    intents.insert(GatewayIntents::GUILD_MEMBERS);
    intents.insert(GatewayIntents::GUILD_PRESENCES);
    intents.insert(GatewayIntents::MESSAGE_CONTENT);
    intents
}

pub async fn event_handler(
    framework: FrameworkContext<'_>,
    event: &FullEvent,
) -> Result {
    let data = framework.user_data();
    let ctx = framework.serenity_context;
    match event {
        FullEvent::Resume { event: _ } => {
            log::info!("Shard: {} has resumed.", ctx.shard_id);
        },
        FullEvent::ShardStageUpdate { event } => {
            if event.new.is_connecting() {
                log::info!("Shard: {} is connecting.", ctx.shard_id);
            } else if event.new == ConnectionStage::Disconnected {
                log::info!("Shard: {} has disconnected.", ctx.shard_id);
            }
        }
        FullEvent::Ready {
            data_about_bot,
        } => ready::ready(ctx, data_about_bot, &framework).await,
        FullEvent::Message { new_message } => message::message(ctx, new_message, &data).await,
        FullEvent::PresenceUpdate { new_data } => {
            presence::presence(ctx, new_data).await
        }
        FullEvent::GuildMemberRemoval { guild_id: _, user, member_data_if_available: _ } => {
            member_removal::member_removal(ctx, user).await
        }
        _ => {}
    }
    Ok(())
}