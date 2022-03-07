use anyhow::Error;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Database};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    id: i64,
    accountName: String,
    email: String,
    password: String,
    pin: String,
}

pub async fn get_user(username: &str, db: &Database) -> Result<UserInfo, Error> {
    let filter = doc! { "accountName": username };
    let find_options = FindOptions::builder().sort(doc! { "title": 1i32 }).build();
    let typed_collection = db.collection::<UserInfo>("accounts");
    let mut cursor = typed_collection.find(filter, find_options).await?;

    while let Some(user) = cursor.try_next().await? {
        return Ok(UserInfo {
            id: user.id,
            accountName: user.accountName,
            email: user.email,
            password: user.password,
            pin: user.pin,
        });
    }

    Ok(UserInfo {
        id: -1,
        accountName: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
        pin: "".to_string(),
    })
}
