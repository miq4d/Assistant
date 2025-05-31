use poise::{ChoiceParameter, CreateReply};
use serenity::all::CreateAttachment;
use sqlx::types::Json;

use crate::{
    data::{Context, Result},
    db::{CachedGuild, DiscordGuild, DiscordOAuth2Token, DiscordUser},
};

/// Admin command
#[poise::command(
    slash_command,
    owners_only,
    default_member_permissions = "ADMINISTRATOR",
    subcommands("status")
)]
pub async fn admin(_ctx: Context<'_>) -> Result {
    Ok(())
}

#[derive(Clone, ChoiceParameter, Debug, PartialEq, Eq)]
enum StatusType {
    Guild = 0,
    User = 1,
}

/// Show status of a guild or user
#[poise::command(slash_command, owners_only)]
pub async fn status(
    ctx: Context<'_>,
    #[rename = "type"]
    #[description = "Guild or User"]
    kind: StatusType,
    #[description = "ID of the guild or user"] id: String,
    #[description = "Whether the result is ephemeral"] ephemeral: Option<bool>,
) -> Result {
    if kind == StatusType::Guild {
        let result = sqlx::query_as!(
            DiscordGuild,
            "SELECT * FROM discord_guilds WHERE guild_id = $1",
            id
        )
        .fetch_optional(&ctx.data().db)
        .await?;

        if let Some(result) = result {
            let serialized = serde_json::to_string_pretty(&result)?;
            ctx.send(
                CreateReply::new()
                    .attachment(CreateAttachment::bytes(
                        serialized.into_bytes(),
                        "guild.json",
                    ))
                    .ephemeral(ephemeral.unwrap_or(false)),
            )
            .await?;
        } else {
            ctx.send(
                CreateReply::new()
                    .content("Guild not found")
                    .ephemeral(ephemeral.unwrap_or(false)),
            )
            .await?;
        }
    } else {
        let result = sqlx::query_as!(
            DiscordUser,
            r#"
            SELECT user_id, is_banned, generate_block, allow_custom_quote, cache_updated_at, compact,
                default_settings, guild_cache as "guild_cache: Vec<Json<CachedGuild>>" , oauth2 as "oauth2: Json<DiscordOAuth2Token>", 
                enable_done, force_disable_custom_quote
            FROM "discord_users"
            WHERE "user_id" = $1"#,
            id
        )
            .fetch_optional(&ctx.data().db)
            .await?;

        if let Some(result) = result {
            let serialized = serde_json::to_string_pretty(&result)?;
            ctx.send(
                CreateReply::new()
                    .attachment(CreateAttachment::bytes(
                        serialized.into_bytes(),
                        "user.json",
                    ))
                    .ephemeral(ephemeral.unwrap_or(false)),
            )
            .await?;
        } else {
            ctx.send(
                CreateReply::new()
                    .content("User not found")
                    .ephemeral(ephemeral.unwrap_or(false)),
            )
            .await?;
        }
    }

    Ok(())
}
