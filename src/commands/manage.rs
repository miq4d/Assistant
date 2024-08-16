use std::{collections::HashMap, time::Duration};

use poise::CreateReply;
use serenity::{
    all::{
        ComponentInteractionDataKind, CreateInteractionResponse, CreateInteractionResponseMessage,
        Member, RoleId,
    },
    builder::{
        CreateActionRow, CreateEmbed, CreateEmbedAuthor, CreateMessage, CreateSelectMenu,
        CreateSelectMenuKind, CreateSelectMenuOption,
    },
};

use crate::{
    constants::MOD_CHANNEL_ID,
    data::{Context, Result},
};

/// Moderate user
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "MODERATE_MEMBERS"
)]
pub async fn manage(ctx: Context<'_>, #[description = "Target"] member: Member) -> Result {
    let roles = ctx.guild_id().unwrap().roles(ctx.http()).await?;
    let roles = roles
        .iter()
        .map(|r| (r.id, r))
        .filter(|r| r.1.name.starts_with("🚨"))
        .collect::<HashMap<_, _>>();
    let roles_id = roles
        .iter()
        .filter(|r| r.1.name.starts_with("🚨"))
        .map(|(id, r)| (id.get(), r))
        .collect::<HashMap<_, _>>();

    // refetch member
    let member = ctx
        .guild_id()
        .unwrap()
        .member(ctx.http(), member.user.id)
        .await?;
    let member_roles = member.roles(ctx.cache()).unwrap_or(vec![]);
    let menu = CreateSelectMenu::new(
        "menu",
        CreateSelectMenuKind::String {
            options: roles
                .iter()
                .map(|(id, r)| {
                    CreateSelectMenuOption::new(r.name.clone(), id.get().to_string())
                        .default_selection(member_roles.contains(r))
                })
                .collect(),
        },
    )
    .placeholder("Select roles")
    .min_values(0)
    .max_values(roles.len() as u8);

    let reply = ctx.send(
        CreateReply::new()
            .components(vec![CreateActionRow::SelectMenu(menu)])
            .embed(
                CreateEmbed::new()
                    .title("Warning role manager")
                    .description("Please choose warning roles to assign/remove to the user.\n**WARNING**: You can only assign roles that marked with 🚨 prefix."),
            )
            .ephemeral(true),
    )
    .await?;

    let m = reply.into_message().await?;

    let interaction = m
        .await_component_interaction(ctx.serenity_context().shard.clone())
        .timeout(Duration::from_secs(300))
        .await;

    if let Some(i) = interaction {
        let roles = {
            let d = i.data.clone();
            if let ComponentInteractionDataKind::StringSelect { values, .. } = d.kind {
                values
                    .iter()
                    .map(|v| v.parse::<RoleId>().unwrap())
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        };
        let mut to_add = vec![];
        let mut to_remove = vec![];
        for role in roles_id {
            if roles.contains(&role.1.id) {
                to_add.push(role.1);
            } else {
                if member_roles.contains(&role.1) {
                    to_remove.push(role.1);
                }
            }
        }

        for role in &to_add {
            log::debug!("Adding role: {}", role.name);
            ctx.http()
                .add_member_role(
                    ctx.guild_id().unwrap(),
                    member.user.id,
                    role.id,
                    Some(&format!("Executed by {}", ctx.author().tag())),
                )
                .await?;
        }
        for role in &to_remove {
            log::debug!("Removing role: {}", role.name);
            ctx.http()
                .remove_member_role(
                    ctx.guild_id().unwrap(),
                    member.user.id,
                    role.id,
                    Some(&format!("Executed by {}", ctx.author().tag())),
                )
                .await?;
        }

        i.create_response(
            ctx.http(),
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embed(
                        CreateEmbed::new()
                            .title("Warning role manager")
                            .description(format!("Roles updated for {}", member.user.tag()))
                            .color(0x00FF00),
                    )
                    .components(vec![]),
            ),
        )
        .await?;

        MOD_CHANNEL_ID
            .send_message(
                ctx.http(),
                CreateMessage::new().embed(
                    CreateEmbed::new()
                        .title("Warning role manager")
                        .description(format!("Roles updated for {}", member.user.tag()))
                        .author(
                            CreateEmbedAuthor::new(format!("Done by {}", ctx.author().tag()))
                                .icon_url(ctx.author().face()),
                        )
                        .fields(vec![
                            (
                                "Roles added",
                                to_add
                                    .iter()
                                    .map(|r| r.name.clone())
                                    .collect::<Vec<_>>()
                                    .join("\n"),
                                true,
                            ),
                            (
                                "Roles removed",
                                to_remove
                                    .iter()
                                    .map(|r| r.name.clone())
                                    .collect::<Vec<_>>()
                                    .join("\n"),
                                true,
                            ),
                        ]),
                ),
            )
            .await?;
    }

    Ok(())
}
