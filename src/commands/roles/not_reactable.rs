use poise::{ApplicationContext, Modal};
use serenity::{
    all::User,
    builder::{CreateEmbed, CreateMessage, CreateEmbedFooter}, model::Color,
};

use crate::{
    constants::{MOD_CHANNEL_ID, NOT_REACTABLE_ROLE_ID},
    data::{Error, Result, SharedData},
};

#[derive(Debug, poise::Modal)]
#[name = "Reason"]
struct ReasonModal {
    #[name = "Reason"]
    #[paragraph]
    #[placeholder = "Why make this user Not Reactable?"]
    #[min_length = 3]
    reason: String,
}

#[poise::command(context_menu_command = "Not Reactable User", guild_only, ephemeral)]
pub async fn not_reactable(ctx: ApplicationContext<'_, SharedData, Error>, user: User) -> Result {
    let reason = ReasonModal::execute(ctx).await?.unwrap().reason;
    let guild = ctx.guild_id().unwrap();

    let member = guild.member(&ctx.http(), user.id).await;

    if member.is_err() {
        ctx.reply("This user is not in this server").await?;
        return Ok(());
    }

    let mut member = member.unwrap();

    if member.roles.contains(&NOT_REACTABLE_ROLE_ID) {
        ctx.reply("This user is already not reactable").await?;
        return Ok(());
    }

    member
        .add_role(&ctx.http(), NOT_REACTABLE_ROLE_ID.get())
        .await?;

    ctx.reply(format!(
        "<@!{}> is now not reactable because: {}",
        user.id.get(),
        reason
    ))
    .await?;

    MOD_CHANNEL_ID
        .send_message(
            &ctx.http(),
            CreateMessage::new().embed(
                CreateEmbed::new()
                    .title("Not Reactable")
                    .description(format!(">>> **User:** @{} (<@!{}>)\n**Reason:** {}", user.name, user.id.get(), reason))
                    .thumbnail(user.face())
                    .timestamp(chrono::Utc::now())
                    .color(Color::RED)
                    .footer(CreateEmbedFooter::new(format!("@{}", ctx.author().name)).icon_url(ctx.author().face())),
            ),
        )
        .await?;

    Ok(())
}
