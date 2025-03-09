use jsonwebtokens as jwts;
use jwts::raw::{self, TokenSlices};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub jti: String,
    pub sub: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "creator_id")]
    pub creator_id: String,
    pub email: String,
    #[serde(rename = "first_name")]
    pub first_name: String,
    pub id: String,
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(rename = "last_name")]
    pub last_name: String,
    #[serde(rename = "organisation_id")]
    pub organisation_id: String,
    #[serde(rename = "organisation_name")]
    pub organisation_name: String,
    #[serde(rename = "organisation_path")]
    pub organisation_path: String,
    pub password: String,
    pub scope: String,
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
    pub user_role: String,
    #[serde(rename = "user_permissions")]
    pub user_permissions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClaimsEdge {
    pub aud: String,
    pub iss: String,
    pub iat: i64,
    pub exp: u64,
    pub email: String,
    pub idtyp: String,
    pub name: String,
    pub oid: String,
    pub tid: String,
    #[serde(rename = "unique_name")]
    pub unique_name: String,
}

#[derive(Debug)]
pub struct TokenError {
    message: String,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub async fn decode(token: String)-> Result<(String, String), TokenError> {
    let current_time = SystemTime::now();
    // Calculate the duration since the Unix epoch
    let duration_since_epoch = match current_time.duration_since(UNIX_EPOCH) {
        Ok(v) => v,
        Err(e) => return Err(TokenError{message: e.to_string()}),
    };
    
    // Extract the number of seconds as the Unix timestamp
    let unix_timestamp:u64 = duration_since_epoch.as_secs();

    let TokenSlices{claims,..} = match raw::split_token(&token) {
        Ok(v) => v,
        Err(e) => return Err(TokenError{message: e.to_string()}),
    };
    let raw_claim = match raw::decode_json_token_slice(claims) {
        Ok(v) => v,
        Err(e) => return Err(TokenError{message: e.to_string()}),
    };
    let claim:Claims = serde_json::from_value(raw_claim).unwrap();
    if claim.exp <= unix_timestamp as i64{
        return Err(TokenError{message: "invalid token provided".to_string()})
    }
    return Ok((claim.user.email, claim.user.id))
}


// http://jwtbuilder.jamiekurtz.com/