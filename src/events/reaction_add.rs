use serenity::all::{
    CacheHttp, Context, CreateEmbed, CreateMessage, Mentionable, Reaction, ReactionType,
};

use crate::{
    constants::{
        ALLOWED_REACTIONS, ALLOWED_REACTION_IDS, ANNOUNCE_CHANNEL_IDS, BANNED_REACTIONS,
        MOD_CHANNEL_ID, NOT_REACTABLE_ROLE_ID, REACTION_LIMIT_BYPASS_ROLE_IDS,
    },
    helper::remove_skin_tone,
};

pub async fn reaction_add(ctx: &Context, add_reaction: &Reaction) {
    if add_reaction.user(&ctx).await.unwrap().bot() {
        return;
    }

    if !ANNOUNCE_CHANNEL_IDS.contains(&add_reaction.channel_id.expect_channel()) {
        return;
    }

    for reaction in BANNED_REACTIONS.iter() {
        if let ReactionType::Unicode(s) = &add_reaction.emoji {
            if remove_skin_tone(s) == *reaction {
                add_reaction.delete(ctx.http()).await.unwrap();
                add_reaction
                    .member
                    .as_ref()
                    .unwrap()
                    .add_role(
                        ctx.http(),
                        *NOT_REACTABLE_ROLE_ID,
                        Some(&format!(
                            "Automatically added role for reacting with a banned reaction: {}",
                            reaction
                        )),
                    )
                    .await
                    .unwrap();
                MOD_CHANNEL_ID
                    .widen()
                    .send_message(
                        ctx.http(),
                        CreateMessage::default().embeds(vec![CreateEmbed::default().description(
                            format!(
                                "{} reacted with a banned reaction: {}",
                                add_reaction.user_id.unwrap().mention(),
                                reaction
                            ),
                        )]),
                    )
                    .await
                    .unwrap();
                return;
            }
        }
    }

    for role in REACTION_LIMIT_BYPASS_ROLE_IDS.iter() {
        if add_reaction.member.as_ref().unwrap().roles.contains(role) {
            return;
        }
    }

    for reaction in ALLOWED_REACTIONS.iter() {
        match &add_reaction.emoji {
            ReactionType::Unicode(s) => {
                if remove_skin_tone(s) == *reaction {
                    return;
                }
            }
            ReactionType::Custom {
                animated: _,
                id,
                name: _,
            } => {
                if ALLOWED_REACTION_IDS.contains(id) {
                    return;
                }
            }
            _ => {}
        }
    }

    add_reaction.delete(ctx.http()).await.unwrap();
}
