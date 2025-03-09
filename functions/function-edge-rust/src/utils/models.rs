use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct User {
    pub email: String,
    pub id: String,
    pub token: Option<String>,
}

#[derive(Debug)]
pub struct Unauthorized {
    pub message: String,
}
impl warp::reject::Reject for Unauthorized {}


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AppConfig {
    pub sql_db_host: String,
    pub sql_db_name: String,
    pub sql_db_username: String,
    pub sql_db_password: String,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            sql_db_host: "".into(),
            sql_db_name: "".into(),
            sql_db_username: "".into(),
            sql_db_password: "".into(),
        }
    }
}