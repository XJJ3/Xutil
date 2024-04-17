use reqwest::Response;
use serde_json::Value;

use super::common::AsyncCommandTrait;

pub struct TranslateText;
impl AsyncCommandTrait for TranslateText {
    async fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String> {
        println!("翻译路径: {}", args);
        let params: Value = serde_json::from_str(&args.to_string()).unwrap();

        let url = match params.get("url") {
            Some(str) => match str.as_str() {
                Some(s) => s,
                None => "",
            },
            None => "",
        };

        if url.is_empty() {
            return Err("url 参数不能为空".to_string());
        }

        let response = match translate_req(url).await {
            Ok(res) => res,
            Err(err) => {
                println!("报错了: {:?}", err);
                return Err(err.to_string());
            }
        };

        let status = response.status().to_string();
        // let headers = response.headers();
        // let body = response.json::<TranslateResponseBody>().await.unwrap();
        let body = response.text().await.unwrap();
        Ok(serde_json::json!({ "status": status,  "body": body }))
    }
}

async fn translate_req(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    Ok(reqwest::get(url).await?)
}
