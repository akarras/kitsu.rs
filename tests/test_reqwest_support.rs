#![cfg(feature = "reqwest-support")]

extern crate kitsu_async;
extern crate reqwest;

use kitsu_async::reqwest_kitsu::KitsuRequester;
use reqwest::Client;

#[ignore]
#[tokio::test]
async fn test_get_anime() {
    let client = Client::new();
    let res = client.get_anime(1).await.unwrap();

    assert_eq!(res.data.id, "1");
}


#[tokio::test]
async fn test_get_manga() {
    let client = Client::new();
    let res = client.get_manga(1).await.unwrap();

    assert_eq!(res.data.id, "1");
}

#[ignore]
#[tokio::test]
async fn test_get_user() {
    let client = Client::new();
    let res = client.get_user(1).await.unwrap();

    assert_eq!(res.data.id, "1");
}

#[tokio::test]
async fn test_search_anime() {
    let client = Client::new();
    let res = client.search_anime(|f| f.filter("text", "non non biyori")).await.unwrap();

    assert!(res.data.len() > 0);
}

#[tokio::test]
async fn test_search_manga() {
    let client = Client::new();
    let res = client.search_manga(|f| f.filter("text", "orange")).await.unwrap();

   assert!(res.data.len() > 0);
}

#[ignore]
#[tokio::test]
async fn test_search_users() {
    let client = Client::new();
    let res = client.search_users(|f| f.filter("name", "vikhyat")).await.unwrap();
    assert!(res.data.len() > 0);
}
