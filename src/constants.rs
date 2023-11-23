use once_cell::sync::Lazy;
use regex::Regex;
use serenity::all::{ChannelId, UserId, RoleId, GuildId};

pub static PREFIX: &'static str = ".";

pub static MENTION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(r"<@!?{}>", MAIN_BOT_ID.get()).as_str()).unwrap());

pub static STATUS_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1116627817346646058));
pub static DOWN_REPORT_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1162391031321468948));
pub static HTQ_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1101830185743028224));
pub static MOD_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1046639404292251718));
pub static MEMBER_LOG_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1099137450758123540));
pub static EN_FORUM_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1086220818423500810));
pub static JA_FORUM_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1086217844108304455));

pub static MAIN_BOT_ID: Lazy<UserId> = Lazy::new(|| UserId::from(949479338275913799));

pub static ADMIN_ROLE_ID: Lazy<RoleId> = Lazy::new(|| RoleId::from(954552825902411795));
pub static NOT_REACTABLE_ROLE_ID: Lazy<RoleId> = Lazy::new(|| RoleId::from(1163413580629155900));

pub static GUILD_ID: Lazy<GuildId> = Lazy::new(|| GuildId::from(954552825902411786));