use once_cell::sync::Lazy;
use serenity::all::ChannelId;

pub static STATUS_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1116627817346646058));