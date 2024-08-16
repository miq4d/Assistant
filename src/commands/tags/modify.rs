use crate::{
    data::{Context, Result},
    helper::{add_tag, remove_tag},
    structs::tags::Tag,
};

#[allow(non_camel_case_types)]
#[derive(poise::ChoiceParameter)]
pub enum OptionChoices {
    #[name = "English"]
    en,
}

#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "MANAGE_MESSAGES",
    subcommands("create", "delete", "list")
)]
pub async fn tag(_: Context<'_>) -> Result {
    Ok(())
}

/// Create a tag
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "MANAGE_MESSAGES",
    ephemeral
)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Lang"] lang: OptionChoices,
    #[description = "Command key"] key: String,
    #[description = "Tag value"] value: String,
) -> Result {
    let lang = match lang {
        OptionChoices::en => "en",
    };
    let key = key.to_lowercase();
    add_tag(
        Tag {
            key,
            value: value.replace("\\n", "\n"),
        },
        lang,
    )
    .await;
    ctx.say("Tag created").await?;
    Ok(())
}

/// Delete a tag
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "MANAGE_MESSAGES",
    ephemeral
)]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "Lang"] lang: OptionChoices,
    #[description = "Command key"] key: String,
) -> Result {
    let lang = match lang {
        OptionChoices::en => "en",
    };
    let key = key.to_lowercase();
    remove_tag(key, lang).await?;
    ctx.say("Tag deleted").await?;
    Ok(())
}

/// List tag
#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn list(ctx: Context<'_>, #[description = "Lang"] lang: OptionChoices) -> Result {
    let lang = match lang {
        OptionChoices::en => "en",
    };
    let tags = crate::helper::get_tags(lang).await;
    let tags = tags
        .into_iter()
        .map(|tag| format!("- {}", tag.key))
        .collect::<Vec<_>>()
        .join("\n");
    ctx.say(tags).await?;
    Ok(())
}
