/*
  | `tokio::sync::mpsc` | `std::sync::mpsc` |
  | ------------------- | ----------------- |
  | `channel`           | `sync_channel`    |
  | `unbounded_channel` | `channel`         |
*/

pub async fn get_page_title(url: &str) -> Option<String> {
    let res = reqwest::get(url).await.unwrap();
    let response_text = res.text().await.unwrap();
    scraper::Html::parse_document(&response_text)
        .select(&scraper::Selector::parse("title").unwrap())
        .nth(0)
        .map(|el| el.inner_html())
}
