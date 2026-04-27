use super::session::Session;
use std::path::PathBuf;

use toasty::Model;

#[derive(Debug, Model)]
pub struct Project {
    #[key]
    #[auto]
    pub id: i32,

    pub name: String,
    pub worktree: String, // also known as a "branch" in some VCS
    pub vcs: String,      // e.g., "git", "svn", etc.
    root_path: String,    // the root directory of the project

    #[default(false)]
    pub is_archived: bool,

    #[default(jiff::Timestamp::now())]
    pub created_at: jiff::Timestamp,

    #[has_many]
    sessions: toasty::HasMany<Session>,
}

impl Project {
    pub async fn get_root_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let root_path = PathBuf::from(&self.root_path);
        if !root_path.exists() {
            log::error!("Project root path does not exist: {:?}", root_path);
            return Err("Project root path does not exist.".into());
        }
        Ok(root_path)
    }

    pub async fn get_sessions(&self, mut db: toasty::Db) -> toasty::Result<Vec<Session>> {
        self.sessions()
            .query(Session::fields().is_archived().eq(false))
            .exec(&mut db)
            .await
    }
}
