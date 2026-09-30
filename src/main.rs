use serenity::{Client, all::GatewayIntents};

#[tokio::main]
async fn main() {
    // .envファイル読み込み
    dotenv::dotenv().ok();
    let discord_token =
        dotenv::var("DISCORD_BOT_TOKEN").expect("discord tokenの取得に失敗しました");

    // Intentsの設定
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // クライアントの作成
    let mut client = Client::builder(&discord_token, intents)
        .await
        .expect("Clientの作成に失敗しました");

    // クライアント実行
    if let Err(why) = client.start().await {
        println!("クライアントエラー: {:?}", why);
    }
}
