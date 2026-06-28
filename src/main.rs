use serenity::Client;
use serenity::prelude::GatewayIntents;
use crate::events::ping_event::PingEvent;
use crate::events::ready_event::ReadyEvent;
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
        
        let client = Client::builder(bot_token, intents)
            .event_handler(ReadyEvent)
            .event_handler(PingEvent)
            .await;
        
        // クライアントオブジェクトの作成に成功
        if let Ok(mut client) = client {
            println!("Success to create Client");
            
            if let Err(e) = &client.start().await {
                println!("Failed to start this bot:{}", e.to_string());
            }
        }
        // 失敗
        else {
            println!("Failed to create Client");
        }
    }
    // 設定ファイルの読み込み失敗
    else {
        println!("Failed to read setting file");
    }
}
