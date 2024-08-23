use serenity::all::{CacheHttp, Context, CreateEmbed, CreateMessage, Mentionable, Reaction};

use crate::constants::{
    BANNED_REACTIONS, FORBIDDEN_REACTIONS, MOD_CHANNEL_ID, NOT_REACTABLE_ROLE_ID,
};

pub async fn reaction_add(ctx: &Context, add_reaction: &Reaction) {
    if add_reaction.user(&ctx).await.unwrap().bot() {
        return;
    }

    for reaction in BANNED_REACTIONS.iter() {
        if add_reaction.emoji.unicode_eq(reaction) {
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

    for reaction in FORBIDDEN_REACTIONS.iter() {
        if add_reaction.emoji.unicode_eq(reaction) {
            add_reaction.delete(ctx.http()).await.unwrap();
            return;
        }
    }
}
