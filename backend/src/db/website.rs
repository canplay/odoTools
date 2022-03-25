use anyhow::Error;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid, Bson},
    Database,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewsInfo {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,
    pub title: String,
    pub date: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub content: String,
    pub delete: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CarouselInfo {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,
    pub img: String,
    pub url: String,
    pub delete: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadsInfo {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,
    pub title: String,
    pub desc: String,
    pub url: String,
    pub delete: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShopInfo {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,
    pub preview: String,
    pub name: String,
    pub cash: f64,
    #[serde(rename = "type")]
    pub typ: String,
    pub desc: String,
    pub delete: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigInfo {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,
    pub name: String,
    pub login_server: String,
    pub delete: bool,
}

pub async fn set_news(news: NewsInfo, db: &Database) -> Result<bool, Error> {
    let typed_collection = db.collection::<NewsInfo>("news");
    let cursor = typed_collection
        .find(doc! { "_id": news.id, "delete": false }, None)
        .await;

    if cursor.is_err() {
        let typed_collection = db.collection::<NewsInfo>("news");
        let result = typed_collection.insert_one(news, None).await?;

        return Ok(if result.inserted_id == Bson::Null {
            false
        } else {
            true
        });
    } else {
        let typed_collection = db.collection::<NewsInfo>("news");
        let result = typed_collection
            .update_one(
                doc! { "_id": news.id },
                doc! {
                    "$set": {
                        "title": news.title,
                        "date": news.date,
                        "type": news.typ,
                        "content": news.content,
                        "delete": news.delete,
                    }
                },
                None,
            )
            .await?;

        return Ok(if result.modified_count == 0 {
            false
        } else {
            true
        });
    }
}

pub async fn get_news(id: &str, db: &Database) -> Result<Vec<NewsInfo>, Error> {
    match id {
        "" => {
            let typed_collection = db.collection::<NewsInfo>("news");
            let mut cursor = typed_collection
                .find(doc! { "delete": false }, None)
                .await?;

            let mut list = vec![];
            while let Some(element) = cursor.try_next().await? {
                list.push(element.clone());
            }

            Ok(list)
        }
        _ => {
            let typed_collection = db.collection::<NewsInfo>("news");
            let cursor = typed_collection
                .find_one(
                    doc! { "_id": oid::ObjectId::parse_str(id).unwrap(), "delete": false },
                    None,
                )
                .await?;
            let list = vec![cursor.unwrap()];

            Ok(list)
        }
    }
}

pub async fn set_carousel(carousel: CarouselInfo, db: &Database) -> Result<bool, Error> {
    let typed_collection = db.collection::<CarouselInfo>("carousel");
    let cursor = typed_collection
        .find(doc! { "_id": carousel.id, "delete": false }, None)
        .await;

    if cursor.is_err() {
        let typed_collection = db.collection::<CarouselInfo>("carousel");
        let result = typed_collection.insert_one(carousel, None).await?;

        return Ok(if result.inserted_id == Bson::Null {
            false
        } else {
            true
        });
    } else {
        let typed_collection = db.collection::<CarouselInfo>("carousel");
        let result = typed_collection
            .update_one(
                doc! { "_id": carousel.id },
                doc! {
                    "$set": {
                        "img": carousel.img,
                        "url": carousel.url,
                        "delete": carousel.delete,
                    }
                },
                None,
            )
            .await?;

        return Ok(if result.modified_count == 0 {
            false
        } else {
            true
        });
    }
}

pub async fn get_carousel(id: &str, db: &Database) -> Result<Vec<CarouselInfo>, Error> {
    match id {
        "" => {
            let typed_collection = db.collection::<CarouselInfo>("carousel");
            let mut cursor = typed_collection
                .find(doc! { "delete": false }, None)
                .await?;

            let mut list = vec![];
            while let Some(element) = cursor.try_next().await? {
                list.push(element.clone());
            }

            Ok(list)
        }
        _ => {
            let typed_collection = db.collection::<CarouselInfo>("carousel");
            let cursor = typed_collection
                .find_one(
                    doc! { "_id": oid::ObjectId::parse_str(id).unwrap(), "delete": false },
                    None,
                )
                .await?;
            let list = vec![cursor.unwrap()];

            Ok(list)
        }
    }
}

pub async fn set_downloads(downloads: DownloadsInfo, db: &Database) -> Result<bool, Error> {
    let typed_collection = db.collection::<DownloadsInfo>("downloads");
    let cursor = typed_collection
        .find(doc! { "_id": downloads.id, "delete": false }, None)
        .await;

    if cursor.is_err() {
        let typed_collection = db.collection::<DownloadsInfo>("downloads");
        let result = typed_collection.insert_one(downloads, None).await?;

        return Ok(if result.inserted_id == Bson::Null {
            false
        } else {
            true
        });
    } else {
        let typed_collection = db.collection::<DownloadsInfo>("downloads");
        let result = typed_collection
            .update_one(
                doc! { "_id": downloads.id },
                doc! {
                    "$set": {
                        "title": downloads.title,
                        "desc": downloads.desc,
                        "url": downloads.url,
                        "delete": downloads.delete,
                    }
                },
                None,
            )
            .await?;

        return Ok(if result.modified_count == 0 {
            false
        } else {
            true
        });
    }
}

pub async fn get_downloads(id: &str, db: &Database) -> Result<Vec<DownloadsInfo>, Error> {
    match id {
        "" => {
            let typed_collection = db.collection::<DownloadsInfo>("downloads");
            let mut cursor = typed_collection
                .find(doc! { "delete": false }, None)
                .await?;

            let mut list = vec![];
            while let Some(element) = cursor.try_next().await? {
                list.push(element.clone());
            }

            Ok(list)
        }
        _ => {
            let typed_collection = db.collection::<DownloadsInfo>("downloads");
            let cursor = typed_collection
                .find_one(
                    doc! { "_id": oid::ObjectId::parse_str(id).unwrap(), "delete": false },
                    None,
                )
                .await?;
            let list = vec![cursor.unwrap()];

            Ok(list)
        }
    }
}

pub async fn set_shop(shop: ShopInfo, db: &Database) -> Result<bool, Error> {
    let typed_collection = db.collection::<ShopInfo>("shop");
    let cursor = typed_collection
        .find(doc! { "_id": shop.id, "delete": false }, None)
        .await;

    if cursor.is_err() {
        let typed_collection = db.collection::<ShopInfo>("shop");
        let result = typed_collection.insert_one(shop, None).await?;

        return Ok(if result.inserted_id == Bson::Null {
            false
        } else {
            true
        });
    } else {
        let typed_collection = db.collection::<ShopInfo>("shop");
        let result = typed_collection
            .update_one(
                doc! { "_id": shop.id },
                doc! {
                    "$set": {
                        "preview": shop.preview,
                        "name": shop.name,
                        "cash": shop.cash,
                        "type": shop.typ,
                        "desc": shop.desc,
                        "delete": shop.delete,
                    }
                },
                None,
            )
            .await?;

        return Ok(if result.modified_count == 0 {
            false
        } else {
            true
        });
    }
}

pub async fn get_shop(id: &str, db: &Database) -> Result<Vec<ShopInfo>, Error> {
    match id {
        "" => {
            let typed_collection = db.collection::<ShopInfo>("shop");
            let mut cursor = typed_collection
                .find(doc! { "delete": false }, None)
                .await?;

            let mut list = vec![];
            while let Some(element) = cursor.try_next().await? {
                list.push(element.clone());
            }

            Ok(list)
        }
        _ => {
            let typed_collection = db.collection::<ShopInfo>("shop");
            let cursor = typed_collection
                .find_one(
                    doc! { "_id": oid::ObjectId::parse_str(id).unwrap(), "delete": false },
                    None,
                )
                .await?;
            let list = vec![cursor.unwrap()];

            Ok(list)
        }
    }
}

pub async fn set_config(config: ConfigInfo, db: &Database) -> Result<bool, Error> {
    let typed_collection = db.collection::<ConfigInfo>("config");
    let cursor = typed_collection
        .find(doc! { "_id": config.id, "delete": false }, None)
        .await;

    if cursor.is_err() {
        let typed_collection = db.collection::<ConfigInfo>("config");
        let result = typed_collection.insert_one(config, None).await?;

        return Ok(if result.inserted_id == Bson::Null {
            false
        } else {
            true
        });
    } else {
        let typed_collection = db.collection::<ConfigInfo>("config");
        let result = typed_collection
            .update_one(
                doc! { "_id": config.id },
                doc! {
                    "$set": {
                        "name": config.name,
                        "login_server": config.login_server,
                        "delete": config.delete,
                    }
                },
                None,
            )
            .await?;

        return Ok(if result.modified_count == 0 {
            false
        } else {
            true
        });
    }
}

pub async fn get_config(id: &str, db: &Database) -> Result<Vec<ConfigInfo>, Error> {
    match id {
        "" => {
            let typed_collection = db.collection::<ConfigInfo>("config");
            let mut cursor = typed_collection
                .find(doc! { "delete": false }, None)
                .await?;

            let mut list = vec![];
            while let Some(element) = cursor.try_next().await? {
                list.push(element.clone());
            }

            Ok(list)
        }
        _ => {
            let typed_collection = db.collection::<ConfigInfo>("config");
            let cursor = typed_collection
                .find_one(
                    doc! { "_id": oid::ObjectId::parse_str(id).unwrap(), "delete": false },
                    None,
                )
                .await?;
            let list = vec![cursor.unwrap()];

            Ok(list)
        }
    }
}
