// use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

use reqwest::{Client, header};
use serde_json::{Value, json};
use tokio::runtime::Runtime;

pub async fn hoi_dap_ngan_gon(
    // tro_chuyen: &str,
    dau_vao: &str,
) -> Result<String, reqwest::Error> {
    // let mut tro_chuyen_moi = String::new();
    // tro_chuyen_moi.push_str("Toi: ");
    // tro_chuyen_moi.push_str(dau_vao);
    // tro_chuyen_moi.push_str("\nAI: ");

    let json = json!({
        "model": "alibaba/tongyi-deepresearch-30b-a3b:free",
        "messages": [
            {
                "role": "user",
                "content": dau_vao
            }
        ],
        "temperature": 0.3,
        // "max_tokens": 150,
        "top_p": 0.9,
        "frequency_penalty": 0.1,
        "presence_penalty": 0.1,
        // "stop": ["Me:", "AI:"]
    });
    let client: Client = Client::new();
    let api_key: &'static str =
        "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";
    let url: &'static str = "https://openrouter.ai/api/v1/chat/completions";

    let body: Value = call_api(&client, api_key, url, json).await?;
    let cau_tra_loi: String = lay_cau_tra_loi(&body).to_string();
    // tro_chuyen_moi.push_str(&cau_tra_loi);
    // tro_chuyen_moi.push('\n');
    Ok(cau_tra_loi)
}

pub async fn call_api(
    client: &Client,
    api_key: &str,
    url: &str,
    json: serde_json::Value,
) -> Result<Value, reqwest::Error> {
    let tra_loi: reqwest::Response = client
        .post(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&json)
        .send()
        .await?;
    let body: Value = tra_loi.json().await?;
    Ok(body)
}

pub fn lay_cau_tra_loi(body: &Value) -> &str {
    body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap()
        .trim()
}

pub fn hoi_dap(dau_vao: &str) -> String {
    // Hàm này vẫn đồng bộ, có thể gây block UI nếu gọi trực tiếp từ main thread Godot
    let rt: Runtime = Runtime::new().unwrap();
    rt.block_on(async {
        match hoi_dap_ngan_gon(dau_vao).await {
            Ok(cau_tra_loi) => cau_tra_loi,
            Err(e) => format!("Đã xảy ra lỗi: {}", e),
        }
    })
}

// Hàm async an toàn cho UI Godot: chạy hoi_dap ở thread riêng, trả kết quả qua callback

pub fn hoi_dap_async(dau_vao: String, result_holder: Arc<Mutex<Option<String>>>) {
    thread::spawn(move || {
        let result = hoi_dap(&dau_vao);
        let mut holder = result_holder.lock().unwrap();
        *holder = Some(result);
    });
}

// pub fn doc_ban_phim() -> String{
//     let mut nhap = String::new();
//     io::stdin().read_line(&mut nhap).unwrap();
//     nhap.trim().to_string()
// }
