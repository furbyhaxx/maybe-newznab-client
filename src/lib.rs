#[cfg(all(feature = "async", feature = "sync"))]
compile_error!(
    "`async` and `sync` features cannot both be enabled at \
    the same time, if you want to use `blocking` you need to set \
    `default-features = false`"
);

#[cfg(not(any(feature = "async", feature = "sync")))]
compile_error!(
    "You have to enable at least one of the \
    `async` or `sync` features."
);

pub mod common;

mod error;
pub use error::Error;

mod client;


pub use client::*;

pub type NewznabClientBuilder = ClientBuilder;
pub type NewznabClient = Client;


#[cfg(test)]
mod tests {
    use crate::client::{Client, ClientBuilder};
    use crate::common::Function;
    use crate::common::models::SearchParameters;

    #[maybe_async::test(
        feature="sync",
        async(all(not(feature="sync"), feature="async"), async_std::test),
    )]
    async fn test_nn_client() {
        let mut client = ClientBuilder::new()
            .url("http://search.arr.reinet.xyz:80/")
            .endpoint("/api")
            .api_token("6PMLKMJMJTML0URBRSTD9PFOHA")
            .to_client();

        println!("api_url: {}", client.get_api_url());
        println!("api_key: {:?}", client.get_api_key());
        println!("default_payload: {:?}", client.get_default_payload());

        // let res = client.function(Function::Caps, OutputFormat::Xml);
        // println!("response: {:?}", client.get_default_payload());

        let caps = client.get_caps().await.unwrap();

        println!("");
    }

    #[maybe_async::test(
        feature="sync",
        async(all(not(feature="sync"), feature="async"), async_std::test),
    )]
    async fn test_nn_search() {
        let mut client = ClientBuilder::new()
            .url("http://search.arr.reinet.xyz:80/")
            .endpoint("/api")
            .api_token("6PMLKMJMJTML0URBRSTD9PFOHA")
            .to_client();

        println!("api_url: {}", client.get_api_url());
        println!("api_key: {:?}", client.get_api_key());
        println!("default_payload: {:?}", client.get_default_payload());

        // let res = client.function(Function::Caps, OutputFormat::Xml);
        // println!("response: {:?}", client.get_default_payload());

        let caps = client.get_caps().await.unwrap();

        println!("");
    }

    #[maybe_async::test(
        feature="sync",
        async(all(not(feature="sync"), feature="async"), async_std::test),
    )]
    async fn test_active_search() {
        let mut client = ClientBuilder::new()
            .url("http://search.arr.reinet.xyz:80/")
            .endpoint("/api")
            .api_token("6PMLKMJMJTML0URBRSTD9PFOHA")
            .to_client();

        println!("api_url: {}", client.get_api_url());
        println!("api_key: {:?}", client.get_api_key());


        let mut sr = client.search(Function::Search (SearchParameters {
            q: "One Piece german".to_string(),
            limit: Some(100),
            offset: None,
            params: None
        })).await;

        if let Ok(mut ar) = sr {
            println!("ar_offset: {} of {}", ar.search_offset.offset, ar.search_offset.total);
            ar.more(&client).await;
            println!("ar_offset: {} of {}", ar.search_offset.offset, ar.search_offset.total);
            ar.more(&client).await;
            println!("ar_offset: {} of {}", ar.search_offset.offset, ar.search_offset.total);
            ar.more(&client).await;
            println!("ar_offset: {} of {}", ar.search_offset.offset, ar.search_offset.total);

            ar.get_more(&client, 1000).await;
            println!("ar_offset: {} of {}", ar.search_offset.offset, ar.search_offset.total);

            // ar.all(&client).await;
            // println!("ar_offset: {} of {}", ar.search_offset.offset, ar.search_offset.total);
        }

        println!();

    }
}