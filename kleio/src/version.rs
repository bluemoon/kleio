use chrono::Utc;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, EntityTrait,
    IntoActiveModel, ModelTrait, Set,
};

use crate::model;

#[async_trait::async_trait]
pub trait Version {
    type VersionEntity;

    async fn create_version<Active>(&self, conn: &DatabaseConnection, user_id: String)
    where
        Self: ModelTrait + Clone,
        // Entity needs have [`sea_orm::EntityTrait`]
        Self::VersionEntity: EntityTrait,
        // Entity Model needs to be [`From`] and [`sea_orm::ActiveModelTrait`]
        // and [`sea_orm::ActiveModelBehavior`]
        <Self::VersionEntity as EntityTrait>::Model: From<Self> + IntoActiveModel<Active>,
        // We turn the Model into an ActiveModel which is denoted as `Active` here
        Active: ActiveModelBehavior + ActiveModelTrait + Send + Sync,
        // VersionEntity::Model -> ActiveModelTrait::Entity
        // -> ActiveModelTrait::Entity::Model
        //
        // Needs to be [`sea_orm::IntoActiveModel`]
        <<Active as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model:
            IntoActiveModel<Active>,
    {
        let txn = model::transaction::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            created_at: Set(Utc::now()),
        };
        txn.into_active_model().insert(conn).await.unwrap();

        let versioned = <Self::VersionEntity as EntityTrait>::Model::from(self.clone());
        versioned.into_active_model().insert(conn).await.unwrap();
    }
}
