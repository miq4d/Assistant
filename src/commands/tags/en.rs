use crate::{
    data::{Context, Result},
    helper::get_tags,
};

#[poise::command(prefix_command, guild_only, user_cooldown = 2)]
pub async fn en(ctx: Context<'_>, tag_name: String, #[rest] _comments: Option<String>) -> Result {
    let tag_name = tag_name.to_lowercase();
    let tag = get_tags("en")
        .await
        .into_iter()
        .find(|tag| tag.key == tag_name);
    if let Some(tag) = tag {
        ctx.say(tag.value).await?;
    } else {
        ctx.say("Tag not found").await?;
    }
    Ok(())
}
