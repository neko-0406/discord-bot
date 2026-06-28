use serenity::all::{Context, EventHandler, Message};
use serenity::async_trait;

pub struct PingEvent;

#[async_trait]
impl EventHandler for PingEvent {
    async fn message(&self, ctx: Context, new_message: Message) {
        if new_message.content == "!ping" {
            let send_result = new_message.channel_id.say(&ctx.http, "pong!").await;
            if let Err(error) = send_result {
                println!("メッセージの送信に失敗しました：{}", error.to_string());
            }
        }
    }
}