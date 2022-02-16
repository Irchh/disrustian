use std::future::Future;
use tl;

use serenity::async_trait;

pub async fn test_translate(str: &str) -> String {
    translate_auto(str, "jp").await.unwrap()
}

pub struct Translator{
    pub to: &'static str,
    pub from: &'static str
}

impl Translator {
    pub async fn translate(&self, text: &str) -> Result<String, String> {
        parse_result(fetch_page(text, self.from, self.to).await)
    }
}

pub async fn translate_auto(text: &str, to: &'static str) -> Result<String, String> {
    let translator_struct = Translator{
        to: to,
        from: "auto"
    };

    translator_struct.translate(text).await
}

async fn fetch_page(text: &str, from: &str, to: &str) -> Result<String, String> {
    let formatted_url = format!("https://translate.google.com/m?tl={}&sl={}&q={}", to, from, text);

    match reqwest::get(formatted_url).await {
        Ok(response) => match response.text().await {
            Ok(body) => return Ok(body),
            Err(err) => return Err(err.to_string())
        },
        Err(err) => return Err(err.to_string())
    }
}

fn parse_result(result: Result<String, String>) -> Result<String, String> {
    match result {
        Ok(body) => match tl::parse(&body.to_owned()).get_elements_by_class_name("result-container") {
            Some(element) => return Ok(element[0].inner_text().into_owned()),
            None => return Err(String::from("unexcepted error."))
        },
        Err(err) => return Err(err)
    }
}