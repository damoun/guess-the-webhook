use chrono::prelude::*;
use serenity::http::Http;
use serenity::model::webhook::Webhook;
use serenity::builder::ExecuteWebhook;
use serenity::builder::CreateAttachment;
use serenity::model::channel::MessageFlags;

#[tokio::main]
async fn main() {
    let http = Http::new("");
    let game_id = (Utc::now() - Utc.with_ymd_and_hms(2022, 5, 14, 0, 0, 0).unwrap()).num_days();
    let game_image = CreateAttachment::url(&http, &format!("https://guessthe.game/games/{game_id}/1.webp")).await.expect("Cannot get game image");
    let webhook_url = std::env::var("DISCORD_WEBHOOK").expect("Please set env: DISCORD_WEBHOOK");
    let webhook = Webhook::from_url(&http, &webhook_url).await.expect("Cannot create Webhook");
    let builder = ExecuteWebhook::new()
        .thread_name(format!("guess-the-game-{game_id}"))
        .flags(MessageFlags::SUPPRESS_EMBEDS)
        .content(format!("https://guessthe.game/?fpg={game_id}"))
        .add_files([game_image]);

    webhook.execute(&http, false, builder).await.expect("Cannot execute Webhook");
}
