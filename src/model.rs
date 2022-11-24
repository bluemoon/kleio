pub mod transaction {
  use chrono::Utc;
  use domain::id::TransactionId;
  use sea_orm::entity::prelude::*;

  #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
  #[sea_orm(table_name = "transaction")]
  pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: TransactionId,
    pub user_id: String,
    pub created_at: DateTimeUtc,
  }

  impl ActiveModelBehavior for ActiveModel {}

  #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
  pub enum Relation {}

  impl Default for Model {
    fn default() -> Self {
      Self {
        id: TransactionId::new(),
        user_id: Default::default(),
        created_at: Utc::now(),
      }
    }
  }
}
