//! Bridge to provide a client implementation for the `reqwest` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`KitsuRequester`].
//!
//! [`KitsuRequester`]: trait.KitsuRequester.html

use crate::builder::Search;
use crate::model::{Anime, Manga, Response, User};
use crate::{Error, Result, API_URL};
use async_trait::async_trait;
pub use reqwest::Client as KitsuClient;
use reqwest::{RequestBuilder, StatusCode, Url};
use serde::de::DeserializeOwned;

/// Trait which defines the methods necessary to interact with the service.
///
/// # Examples
///
/// To bring in the implemenation for the `reqwest` Client, simply use the
/// trait:
///
/// ```rust,no_run
/// use kitsu_async::reqwest_kitsu::KitsuRequester;
/// ```
///
/// At this point, the methods will be on your Reqwest Client.
#[async_trait]
pub trait KitsuRequester {
    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu_async;
    /// extern crate reqwest;
    ///
    /// use kitsu_async::reqwest_kitsu::KitsuRequester;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let anime_id = 1;
    ///
    ///     // Get the anime.
    ///     let anime = client.get_anime(anime_id).await
    ///         .expect("Error getting anime");
    ///
    ///     // Do something with anime
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    async fn get_anime(&self, id: u64) -> Result<Response<Anime>>;

    /// Gets a manga using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu_async;
    /// extern crate reqwest;
    ///
    /// use kitsu_async::reqwest_kitsu::KitsuRequester;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let manga_id = 1;
    ///
    ///     // Get the manga.
    ///     let manga = client.get_anime(manga_id).await
    ///         .expect("Error getting manga");
    ///
    ///     // Do something with manga
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    async fn get_manga(&self, id: u64) -> Result<Response<Manga>>;

    /// Gets a user using their id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu_async;
    /// extern crate reqwest;
    ///
    /// use kitsu_async::reqwest_kitsu::KitsuRequester;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let user_id = 1;
    ///
    ///     // Get the user.
    ///     let user = client.get_anime(user_id).await
    ///         .expect("Error getting user");
    ///
    ///     // Do something with user
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    async fn get_user(&self, id: u64) -> Result<Response<User>>;

    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu_async;
    /// extern crate reqwest;
    ///
    /// use kitsu_async::reqwest_kitsu::KitsuRequester;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let anime_name = "Your Lie in April";
    ///
    ///     // Search for the anime.
    ///     let anime = client.search_anime(|f| f.filter("text", anime_name))
    ///         .await
    ///         .expect("Error searching for anime");
    ///
    ///     // Do something with anime
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    async fn search_anime<F>(&self, f: F) -> Result<Response<Vec<Anime>>>
    where
        F: FnOnce(Search) -> Search + Send;

    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu_async;
    /// extern crate reqwest;
    ///
    /// use kitsu_async::reqwest_kitsu::KitsuRequester;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let manga_name = "Say I Love You";
    ///
    ///     // Search for the manga.
    ///     let manga = client.search_manga(|f| f.filter("text", manga_name))
    ///         .await
    ///         .expect("Error getting manga");
    ///
    ///     // Do something with manga
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    async fn search_manga<F: FnOnce(Search) -> Search>(&self, f: F) -> Result<Response<Vec<Manga>>>
    where
        F: FnOnce(Search) -> Search + Send;

    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu_async;
    /// extern crate reqwest;
    ///
    /// use kitsu_async::reqwest_kitsu::KitsuRequester;
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let user_name = "Billy";
    ///
    ///     // Search for the user.
    ///     let user = client.search_users(|f| f.filter("name", user_name))
    ///         .await
    ///         .expect("Error searching for user");
    ///
    ///     // Do something with users
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    async fn search_users<F>(&self, f: F) -> Result<Response<Vec<User>>>
    where
        F: FnOnce(Search) -> Search + Send;
}

#[async_trait]
impl KitsuRequester for KitsuClient {
    async fn get_anime(&self, id: u64) -> Result<Response<Anime>> {
        let uri = Url::parse(&format!("{}/anime/{}", API_URL, id.to_string()))?;

        handle_request::<Response<Anime>>(self.get(uri)).await
    }

    async fn get_manga(&self, id: u64) -> Result<Response<Manga>> {
        let uri = Url::parse(&format!("{}/manga/{}", API_URL, id.to_string()))?;

        handle_request::<Response<Manga>>(self.get(uri)).await
    }

    async fn get_user(&self, id: u64) -> Result<Response<User>> {
        let uri = Url::parse(&format!("{}/users/{}", API_URL, id.to_string()))?;

        handle_request::<Response<User>>(self.get(uri)).await
    }

    async fn search_anime<F>(&self, f: F) -> Result<Response<Vec<Anime>>>
    where
        F: FnOnce(Search) -> Search + Send,
    {
        let params = f(Search::default()).0;
        let uri = Url::parse(&format!("{}/anime?{}", API_URL, params))?;

        handle_request::<Response<Vec<Anime>>>(self.get(uri)).await
    }

    async fn search_manga<F>(&self, f: F) -> Result<Response<Vec<Manga>>>
    where
        F: FnOnce(Search) -> Search + Send,
    {
        let search = Search::default();
        let params = f(search).0;
        let uri = Url::parse(&format!("{}/manga?{}", API_URL, params))?;
        println!("Reqwesting uri: {}", uri);
        handle_request::<Response<Vec<Manga>>>(self.get(uri)).await
    }

    async fn search_users<F>(&self, f: F) -> Result<Response<Vec<User>>>
    where
        F: FnOnce(Search) -> Search + Send,
    {
        let params = &f(Search::default()).0;
        let uri = Url::parse(&format!("{}/users?{}", API_URL, params))?;
        println!("Reqwesting uri: {}", uri);
        handle_request::<Response<Vec<User>>>(self.get(uri)).await
    }
}

async fn handle_request<T: DeserializeOwned>(request: RequestBuilder) -> Result<T> {
    let response = request.send().await?;

    match response.status() {
        StatusCode::OK => {}
        StatusCode::BAD_REQUEST => {
            return Err(Error::ReqwestBad(Box::new(response)));
        }
        StatusCode::UNAUTHORIZED => {
            return Err(Error::ReqwestUnauthorized(Box::new(response)));
        }
        _ => return Err(Error::ReqwestInvalid(Box::new(response))),
    }

    response.json::<T>().await.map_err(From::from)
}
