use std::env;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    let chat_id: i64 = env::var("CHAT_ID")
        .expect("需要设置 CHAT_ID 环境变量")
        .parse()
        .expect("CHAT_ID 必须是数字");

    let msg = env::var("REMINDER_MSG").unwrap_or_else(|_| "💊".to_string());

    let bot = Bot::from_env();
    bot.send_message(ChatId(chat_id), msg)
        .await
        .expect("发送失败");
}
