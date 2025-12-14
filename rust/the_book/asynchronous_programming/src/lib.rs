/*
  | `tokio::sync::mpsc` | `std::sync::mpsc` |
  | ------------------- | ----------------- |
  | `channel`           | `sync_channel`    |
  | `unbounded_channel` | `channel`         |
*/

use std::pin::pin;

use futures::future::{self, Either};

// pub async fn get_page_title(url: &str) -> Option<String> {
//     let res = reqwest::get(url).await.unwrap();
//     let response_text = res.text().await.unwrap();
//     scraper::Html::parse_document(&response_text)
//         .select(&scraper::Selector::parse("title").unwrap())
//         .nth(0)
//         .map(|el| el.inner_html())
// }

pub async fn get_page_title(url: &str) -> (&str, Option<String>) {
    let res = reqwest::get(url).await.unwrap();
    let response_text = res.text().await.unwrap();
    let maybe_title_text = scraper::Html::parse_document(&response_text)
        .select(&scraper::Selector::parse("title").unwrap())
        .nth(0)
        .map(|el| el.inner_html());

    (url, maybe_title_text)
}

pub async fn select<F1, F2, A, B>(f1: F1, f2: F2) -> Either<A, B>
where
    F1: Future<Output = A>,
    F2: Future<Output = B>,
{
    let f1 = pin!(f1);
    let f2 = pin!(f2);

    match future::select(f1, f2).await {
        Either::Left((a, _)) => Either::Left(a),
        Either::Right((b, _)) => Either::Right(b),
    }
}

pub async fn race<F1, F2, A, B>(f1: F1, f2: F2) -> Either<A, B>
where
    F1: Future<Output = A>,
    F2: Future<Output = B>,
{
    select(f1, f2).await
}
