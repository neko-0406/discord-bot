use serenity::Client;
use serenity::prelude::GatewayIntents;
use crate::setting::BotSetting;

mod events;
mod setting;

#[tokio::main]
async fn main() {
    // 設定ファイルの読み込み処理
    let setting_path = "E:/develop/discord-bot/setting.json";
    let load_setting_result = BotSetting::read_setting_file(&setting_path);
    
    // 設定ファイルの読み込み成功
    if let Some(setting) = load_setting_result {
        let bot_token = setting.discord_token.as_str();
        let intents = 
            GatewayIntents::GUILD_MESSAGES |
            GatewayIntents::MESSAGE_CONTENT;
        
        let client = Client::builder(bot_token, intents).await;
        
        // クライアントオブジェクトの作成に成功
        if let Ok(mut client) = client {
            println!("clientの作成に成功");
            
            if let Err(e) = &client.start().await {
                println!("スタッフエラー:{}", e.to_string());
            }
        }
        // 失敗
        else {
            println!("clientの作成に失敗");
        }
    }
    // 設定ファイルの読み込み失敗
    else {
        println!("設定ファイルの読み込み処理に失敗しました");
    }
}
