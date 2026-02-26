use worker::*;
use serde::Serialize;

#[derive(Serialize)]
struct TelegramMessage {
    chat_id: i64,
    text: String,
}

async fn send_message(env: &Env) -> Result<()> {
    let token = env.secret("TELOXIDE_TOKEN")?.to_string();
    let chat_id_str = env.secret("CHAT_ID")?.to_string();
    let msg_text = env.var("REMINDER_MSG").map(|v| v.to_string()).unwrap_or_else(|_| "💊".to_string());

    let chat_id: i64 = chat_id_str.parse().map_err(|_| Error::RustError("CHAT_ID must be a number".to_string()))?;

    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    
    let payload = TelegramMessage {
        chat_id,
        text: msg_text,
    };

    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;

    let req = Request::new_with_init(
        &url,
        &RequestInit {
            method: Method::Post,
            headers,
            body: Some(serde_json::to_string(&payload)?.into()),
            ..Default::default()
        },
    )?;

    let mut resp: Response = Fetch::Request(req).send().await?;

    if resp.status_code() != 200 {
        let error_text = resp.text().await?;
        return Err(Error::RustError(format!("Telegram API Error: {}", error_text)));
    }

    Ok(())
}

#[event(fetch)]
pub async fn main(_req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    match send_message(&env).await {
        Ok(_) => Response::ok("Message sent"),
        Err(e) => Response::error(format!("Failed: {:?}", e), 500),
    }
}

// 触发方式 2: 定时任务 (Cron)
#[event(scheduled)]
pub async fn scheduled(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_error_panic_hook::set_once();

    if let Err(e) = send_message(&env).await {
        console_log!("Cron Failed: {:?}", e);
    } else {
        console_log!("Cron Succeeded");
    }
}