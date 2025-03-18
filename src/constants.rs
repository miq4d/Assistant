use once_cell::sync::Lazy;
use regex::Regex;
use serenity::all::{ChannelId, EmojiId, GuildId, RoleId, UserId};

pub static PREFIX: &str = ".";

pub static MENTION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(format!(r"<@!?{}>", MAIN_BOT_ID.get()).as_str()).unwrap());

pub static STATUS_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1116627817346646058));
pub static DOWN_REPORT_CHANNEL_ID: Lazy<ChannelId> =
    Lazy::new(|| ChannelId::from(1162391031321468948));
pub static HTQ_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1101830185743028224));
pub static MOD_CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| ChannelId::from(1185539468975943740));
pub static MEMBER_LOG_CHANNEL_ID: Lazy<ChannelId> =
    Lazy::new(|| ChannelId::from(1099137450758123540));
pub static EN_FORUM_CHANNEL_ID: Lazy<ChannelId> =
    Lazy::new(|| ChannelId::from(1086220818423500810));
pub static JA_FORUM_CHANNEL_ID: Lazy<ChannelId> =
    Lazy::new(|| ChannelId::from(1086217844108304455));
pub static ANNOUNCE_CHANNEL_IDS: Lazy<Vec<ChannelId>> = Lazy::new(|| {
    vec![
        ChannelId::from(1010956316849741935),
        ChannelId::from(954688294879387678),
    ]
});

pub static MAIN_BOT_ID: Lazy<UserId> = Lazy::new(|| UserId::from(949479338275913799));

pub static ADMIN_ROLE_ID: Lazy<RoleId> = Lazy::new(|| RoleId::from(954552825902411795));
pub static MOD_ROLE_ID: Lazy<RoleId> = Lazy::new(|| RoleId::from(1048375469050970183));
pub static NOT_REACTABLE_ROLE_ID: Lazy<RoleId> = Lazy::new(|| RoleId::from(1163413580629155900));
pub static REACTION_LIMIT_BYPASS_ROLE_IDS: Lazy<Vec<RoleId>> = Lazy::new(|| {
    vec![
        RoleId::from(1115915406041952318),
        RoleId::from(1200396797324054528),
        RoleId::from(954552825944350770),
        RoleId::from(1048375469050970183),
        RoleId::from(954552825902411795),
    ]
});

pub static GUILD_ID: Lazy<GuildId> = Lazy::new(|| GuildId::from(954552825902411786));
pub static TOTAL_SHARDS: u64 = 500;

pub static BANNED_REACTIONS: Lazy<Vec<&str>> = Lazy::new(|| vec!["🖕"]);
pub static ALLOWED_REACTIONS: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "👍",
        "👍🏻",
        "😄",
        "😀",
        "😆",
        "😂",
        "☺️",
        "😊",
        "😇",
        "😍",
        "🙂",
        "😉",
        "🥰",
        "😙",
        "😋",
        "😳",
        "🫣",
        "🤗",
        "🤔",
        "🫠",
        "🫡",
        "😮",
        "🥴",
        "😎",
        "🥳",
        "😱",
        "😈",
        "😸",
        "😹",
        "😻",
        "😽",
        "🫶",
        "🤝",
        "🙌",
        "👏",
        "✌️",
        "👋",
        "💪",
        "👀",
        "✨",
        "⭐",
        "🌟",
        "💫",
        "⚡",
        "🔥",
        "🏆",
        "🥇",
        "🗿",
        "🎁",
        "🪄",
        "🎊",
        "🎉",
        "‼️",
        "⁉️",
        "❓",
        "❔",
        "❗",
        "❕",
        "♥️",
        "🩷",
        "❤️",
        "🧡",
        "💛",
        "💚",
        "🩵",
        "💙",
        "💜",
        "🖤",
        "🩶",
        "🤍",
        "🤎",
        "❣️",
        "💕",
        "💞",
        "💓",
        "💗",
        "💖",
        "💝",
        "❤️‍🔥",
        "💯",
        "🔟",
        "✅",
        "✔️",
        "☑️",
        "🆗",
        "🆒",
        "🆕"
    ]
});
pub static ALLOWED_REACTION_IDS: Lazy<Vec<EmojiId>> = Lazy::new(|| {
    vec![
        EmojiId::from(1276805815365537868),
        EmojiId::from(1276805203491819550),
        EmojiId::from(1276805726727180360),
        EmojiId::from(1276805471550046261),
        EmojiId::from(1276805645823246407),
        EmojiId::from(1276805399290450034),
        EmojiId::from(1276806225022943323),
        EmojiId::from(1276805621622116487),
        EmojiId::from(1276805953550942238),
        EmojiId::from(1276806950830477363),
        EmojiId::from(1277871073983205418),
        EmojiId::from(1275469043650007154),
    ]
});
