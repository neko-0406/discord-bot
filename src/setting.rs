use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BotSetting {
    pub discord_token: String
}

impl BotSetting {
    pub fn read_setting_file(abs_file_path: &str) -> Option<BotSetting> {
        let mut bot_setting: Option<BotSetting> = None;

        // ファイルパス文字列からPathオブジェクトを作成・オープン
        let target_file_path = Path::new(abs_file_path);
        let target_file_obj = File::open(target_file_path);

        // ファイルのオープンに成功
        if let Ok(target_file) = target_file_obj {
            let buf_reader = BufReader::new(target_file);
            let read_result = serde_json::from_reader::<_, BotSetting>(buf_reader);
            
            // ファイルの中身の読み取りに成功
            if let Ok(result) = read_result {
                bot_setting = Some(result);
            }
            else {
                println!("Failed to read Setting File：{}", abs_file_path);
            }
        }
        else {
            println!("Failed to open Setting File：{}", abs_file_path);
        }

        bot_setting
    }
}