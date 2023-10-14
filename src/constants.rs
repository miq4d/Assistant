use once_cell::sync::Lazy;
use serenity::all::{ChannelId, UserId, RoleId};

pub static STATUS_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1116627817346646058));
pub static MAIN_BOT_ID: Lazy<UserId> = Lazy::new(|| UserId::from(949479338275913799));
pub static DOWN_REPORT_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1162391031321468948));
pub static ADMIN_ROLE_ID: Lazy<RoleId> = Lazy::new(|| RoleId::from(954552825902411795));
pub static PREFIX: &'static str = ".";