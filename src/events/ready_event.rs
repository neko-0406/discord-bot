use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use serenity::all::{Context, GuildId, Ready};
use serenity::async_trait;
use serenity::client::EventHandler;
use crate::modules::system_info::load_system_info;

pub struct ReadyEvent {
    pub issued_task: AtomicBool
}

#[async_trait]
impl EventHandler for ReadyEvent {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built Successfully!");

        // タスクが未発行の場合
        if !self.issued_task.load(Ordering::Relaxed) {
            // タスク分割時に渡すためのオリジナルコンテクスト
            let origin_context = Arc::new(ctx);
            let device_info_ctx = Arc::clone(&origin_context);      // CPU・Memory関数に渡すコンテクスト

            // タスクの発行
            /* CPUとメモリの使用率を監視するタスク */
            tokio::spawn(async move {
                loop {
                    load_system_info(&device_info_ctx).await;
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            });
            
            // タスクを発行したらフラグを変更
            self.issued_task.swap(true, Ordering::Relaxed);
        }
    }

    async fn ready(&self, _: Context, data_about_bot: Ready) {
        println!("{} is connected!", data_about_bot.user.name);
    }
}