use serenity::all::{Context, Ready};
use serenity::async_trait;
use serenity::client::EventHandler;

pub struct ReadyEvent;

#[async_trait]
impl EventHandler for ReadyEvent {
    async fn ready(&self, _: Context, data_about_bot: Ready) {
        println!("{} is connected!", data_about_bot.user.name);
    }
}