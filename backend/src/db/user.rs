use anyhow::Error;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
    Database,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameOption {
    #[serde(rename = "gameOptionData")]
    pub game_option_data: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UiInfo {
    #[serde(rename = "uiData")]
    pub ui_data: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Macroses {
    pub index: i32,
    #[serde(rename = "type")]
    pub typee: i32,
    #[serde(rename = "macrosData")]
    pub macros_data: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountInfo {
    #[serde(rename = "_id")]
    pub id: i64,
    #[serde(rename = "accountName")]
    pub account_name: String,
    pub email: String,
    pub password: String,
    pub pin: String,
    pub family: String,
    #[serde(rename = "accessLvl")]
    pub access_lvl: i32,
    #[serde(rename = "characterSlots")]
    pub character_slots: i32,
    pub cash: i64,
    #[serde(rename = "confirmationHash")]
    pub confirmation_hash: String,
    #[serde(rename = "changePasswordHash")]
    pub change_password_hash: String,
    #[serde(rename = "registrationDate")]
    pub registration_date: i64,
    #[serde(rename = "waitCheck")]
    pub wait_check: bool,
    #[serde(rename = "pcCode")]
    pub pc_code: Option<String>,
    pub host: String,
    #[serde(rename = "gameOption")]
    pub game_option: GameOption,
    #[serde(rename = "uiInfo")]
    pub ui_info: UiInfo,
    pub macroses: Vec<Macroses>,
}

pub async fn add_user(account: AccountInfo, db: &Database) -> Result<bool, Error> {
    let typed_collection = db.collection::<AccountInfo>("accounts");
    let result = typed_collection.insert_one(account, None).await?;

    Ok(if result.inserted_id == Bson::Null {
        false
    } else {
        true
    })
}

pub async fn get_user(username: &str, db: &Database) -> Result<Vec<AccountInfo>, Error> {
    let typed_collection = db.collection::<AccountInfo>("accounts");
    let mut cursor = typed_collection
        .find(doc! { "accountName": username }, None)
        .await?;

    let mut accounts = vec![];
    while let Some(account) = cursor.try_next().await? {
        accounts.push(account)
    }

    Ok(accounts)
}
