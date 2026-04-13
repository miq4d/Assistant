use serenity::all::Ready;

pub async fn ready(data_about_bot: &Ready) {
    tracing::info!("Logged in as {}", data_about_bot.user.name);
}
