use surrealdb::{engine::local::Db, Surreal};

pub mod model_dal;
pub mod table_dal;
pub mod table_row_dal;

pub type SurrealDb = Surreal<Db>;
// pub type SurrealDb = Surreal<Client>;
