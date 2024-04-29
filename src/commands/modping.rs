use poise::CreateReply;
use serenity::all::CreateAllowedMentions;

use crate::{
    constants::MOD_ROLE_ID,
    data::{Context, Result},
};

#[poise::command(prefix_command, guild_only, global_cooldown = 300)]
pub async fn modping(ctx: Context<'_>, #[rest] _comments: Option<String>) -> Result {
    ctx.send(
        CreateReply::new()
            .allowed_mentions(CreateAllowedMentions::new().all_roles(true))
            .content(format!(
                "<@&{}> (Pinged by <@{}>)",
                MOD_ROLE_ID.get(),
                ctx.author().id.get()
            )),
    )
    .await
    .unwrap();
    Ok(())
}
