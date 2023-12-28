use super::{
    _entities::posts::{self, ActiveModel},
    users,
};
use loco_rs::{model::ModelResult, prelude::*};
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub title: String,
    pub md_content: String,
}

impl Params {
    pub fn update(&self, item: &mut ActiveModel) {
        item.title = Set(self.title.clone());
        item.md_content = Set(Some(self.md_content.clone()));
    }
}

async fn load_item(db: &DatabaseConnection, id: i32) -> Result<posts::Model> {
    let item = posts::Entity::find_by_id(id).one(db).await?;
    item.ok_or_else(|| Error::NotFound)
}

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl super::_entities::posts::Model {
    pub async fn add(
        db: &DatabaseConnection,
        params: &Params,
        user_pid: &str,
    ) -> ModelResult<Self> {
        let user_id = users::Model::find_by_pid(db, user_pid).await.unwrap().id;

        let post = posts::ActiveModel {
            title: ActiveValue::set(params.title.to_string()),
            md_content: ActiveValue::set(Some(params.md_content.to_string())),
            user_id: ActiveValue::set(user_id),
            ..Default::default()
        };

        let post = post.insert(db).await?;

        Ok(post)
    }

    pub async fn remove(
        db: &DatabaseConnection,
        id: i32,
        user_pid: &str,
    ) -> ModelResult<(), Error> {
        let user_id = users::Model::find_by_pid(db, user_pid).await?.id;
        let post = load_item(db, id).await?;

        if user_id != post.user_id {
            Err(Error::Unauthorized(
                "You do not have authorization to delete this post".to_string(),
            ))
        } else {
            post.delete(db).await?;
            Ok(())
        }
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        user_pid: &str,
        params: &Params,
    ) -> ModelResult<Self, Error> {
        let user_id = users::Model::find_by_pid(db, user_pid).await?.id;
        let post = load_item(db, id).await?;

        if user_id != post.user_id {
            Err(Error::Unauthorized(
                "You do not have authorization to delete this post".to_string(),
            ))
        } else {
            let mut post = post.into_active_model();
            params.update(&mut post);

            let post = post.update(db).await?;

            Ok(post)
        }
    }
}
