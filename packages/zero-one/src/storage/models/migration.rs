use toasty::Model;

#[derive(Debug, Model)]
pub struct Migration {
    #[key]
    #[auto]
    pub id: i32,

    current_migration_hash: String,

    #[default(jiff::Timestamp::now())]
    executed_at: jiff::Timestamp,
}
