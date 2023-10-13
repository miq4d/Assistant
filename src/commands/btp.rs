use poise::CreateReply;
use serenity::model::Permissions;

use crate::data::{Context, Result};

/// Get the permission names from a bitfield
#[poise::command(slash_command, guild_only)]
pub async fn btp(ctx: Context<'_>, bits: u64) -> Result {
    let perm = Permissions::from_bits(bits);

    if perm.is_none() {
        ctx.send(CreateReply::new().content("Invalid permission bits").ephemeral(true)).await?;
        return Ok(());
    } else if bits == 0 {
        ctx.send(CreateReply::new().content("No permissions").ephemeral(true)).await?;
        return Ok(());
    }
    let perm = perm.unwrap();

    let arr: Vec<_> = perm.iter_names().collect();
    let arr = arr.iter().map(|s| s.0.to_string()).collect::<Vec<_>>();

    // renaming SCREAMING_SNAKE_CASE to Normal Case
    let arr: Vec<_> = arr.iter().map(|s| {
        let mut s = s.clone();
        s = s.replace('_', " ");
        s = s.to_lowercase();
        s = s.split(' ').map(|s| {
            let mut s = s.to_string();
            s.replace_range(0..1, &s[0..1].to_uppercase());
            s
        }).collect::<Vec<_>>().join(" ");
        s
    }).collect::<Vec<_>>();
    
    ctx.send(CreateReply::new().content(arr.join(", ")).ephemeral(true)).await?;

    Ok(())
}