use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{hyper::StatusCode, Filter, reject, Rejection, Reply};
use utils::api;
use dotenv::dotenv;

mod utils;


#[derive(Debug)]
struct MethodError;
impl reject::Reject for MethodError {}


#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT", "OPTIONS"]);

    let storage_handler = warp::path!("api" / "storageAccTrigger"  )
        .and(warp::post())
        .then(|| async move {
            api::process_storage_event().await
        });

    let health = warp::path!("api" /  "health")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .then(|p: HashMap<String, String>| {
            utils::api::index(p)
        });
    
    let options_filter = warp::any().and(warp::options()).map(warp::reply);

    let routes = options_filter
        .or(storage_handler)
        .or(health)
        .with(cors)
        .or(options_filter)
        .recover(handle_not_found);

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };
    
    warp::serve(routes.recover(handle_custom)).run((Ipv4Addr::LOCALHOST, port)).await
}


pub async fn handle_custom(reject: Rejection) -> Result<impl Reply, Rejection> {
    if reject.find::<MethodError>().is_some() {
        Ok(StatusCode::METHOD_NOT_ALLOWED)
    } else {
        Err(reject)
    }
}

pub async fn handle_not_found(reject: Rejection) -> Result<impl Reply, Rejection> {
    if reject.is_not_found() {
        Ok(StatusCode::NOT_FOUND)
    } else {
        Err(reject)
    }
}

// use azure_core::error::{ErrorKind, ResultExt};
// use azure_storage::prelude::*;
// use azure_storage_blobs::prelude::*;
// use futures::stream::StreamExt;

// #[tokio::main]
// async fn main() -> azure_core::Result<()> {
//     let file_name = "azure_sdk_for_rust_stream_test.txt";

//     // First we retrieve the account name and access key from environment variables.
//     let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
//     let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");
//     let container = std::env::var("STORAGE_CONTAINER").expect("missing STORAGE_CONTAINER");
//     let blob_name = std::env::var("STORAGE_BLOB_NAME").expect("missing STORAGE_BLOB_NAME");

//     let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
//     let blob_client = ClientBuilder::new(account, storage_credentials).blob_client(&container, blob_name);

//     blob_client.put_block_blob("hello world").content_type("text/plain").await?;

//     let mut result: Vec<u8> = vec![];

//     // The stream is composed of individual calls to the get blob endpoint
//     let mut stream = blob_client.get().into_stream();
//     while let Some(value) = stream.next().await {
//         let mut body = value?.data;
//         // For each response, we stream the body instead of collecting it all
//         // into one large allocation.
//         while let Some(value) = body.next().await {
//             let value = value?;
//             result.extend(&value);
//         }
//     }

//     println!("result: {:?}", result);
//     let fin = String::from_utf8(result);
//     println!("final: {:?}", fin);

//     Ok(())
// }