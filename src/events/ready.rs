use serenity::all::Ready;

pub async fn ready(data_about_bot: &Ready) {
    log::info!("Logged in as {}", data_about_bot.user.name);
}
