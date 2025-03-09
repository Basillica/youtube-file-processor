use crate::utils::models;
use std::collections::HashMap;
use warp::{http::Response, http::Error};
use std::convert::Infallible;


pub async fn index(p: HashMap<String, String>) -> Result<Response<String>, Error>{
    let var_name = match p.get("name") {
        Some(name) => Response::builder().body(format!("Hello, {}. This HTTP triggered function executed successfully.", name)),
        None => Response::builder().body(format!("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.")),
    };
    var_name
}


pub async fn process_storage_event() -> Result<impl warp::Reply, Infallible> {
    println!("received request from user");
    Ok(format!("sensor has been successfully added"))
}