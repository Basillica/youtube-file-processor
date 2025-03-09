use warp::Filter;
use crate::utils::{jwt, models};

// A function to simulate user authentication
async fn authenticate(token: &str) -> Option<models::User> {
    let parts: Vec<&str> = token.split(' ').collect();
    if parts.len() == 2 {
        let (email, id) = match jwt::decode(parts[1].to_string()).await {
            Ok(v) => v,
            Err(e) => {
                println!("some error occurred decoding user token: {e:?}");
                return None
            },
        };
        Some(models::User {
            email, id, token: Some(token.to_string()),
        })
    } else {
        None
    }
}

pub fn authenticate_filter() -> impl Filter<Extract = (models::User,), Error = warp::reject::Rejection> + Clone {
    warp::header::optional("Authorization")
        .and_then(move |authorization: Option<String>| async move {
            match authorization {
                Some(token) => {
                    if let Some(user) = authenticate(&token).await {
                        Ok(user)
                    } else {
                        Err(warp::reject::custom(models::Unauthorized{
                            message: "invalid authentication token provided".to_string(),
                        }))
                    }
                }
                None => Err(warp::reject::custom(models::Unauthorized{
                    message: "Authorization header missing in request header".to_string(),
                })),
            }
        })
}