use serenity::{prelude::GatewayIntents, client::FullEvent, gateway::ConnectionStage};

use crate::events::{ready, message};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, SharedData, Error>;
pub type Result = std::result::Result<(), Error>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, SharedData, Error>;

pub type SharedData = ();

#[inline]
pub fn get_intents() -> GatewayIntents {
    let mut intents = GatewayIntents::empty();
    intents.insert(GatewayIntents::GUILDS);
    intents.insert(GatewayIntents::GUILD_MESSAGES);
    intents
}

pub async fn event_handler(
    event: &FullEvent,
    _framework: FrameworkContext<'_>,
    _data: &SharedData,
) -> Result {
    match event {
        FullEvent::Resume { ctx, event: _ } => {
            log::info!("Shard: {} has resumed.", ctx.shard_id);
        },
        FullEvent::ShardStageUpdate { ctx, event } => {
            if event.new.is_connecting() {
                log::info!("Shard: {} is connecting.", ctx.shard_id);
            } else if event.new == ConnectionStage::Disconnected {
                log::info!("Shard: {} has disconnected.", ctx.shard_id);
            }
        }
        FullEvent::Ready {
            ctx,
            data_about_bot,
        } => ready::ready(ctx, data_about_bot).await,
        FullEvent::Message { ctx, new_message } => message::message(ctx, new_message).await,
        _ => {}
    }
    Ok(())
}