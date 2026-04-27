use super::project::Project;
use toasty::Model;

#[derive(Debug, Model)]
pub struct Session {
    #[key]
    #[auto]
    pub id: i32,

    #[index]
    pub project_id: i32,

    pub title: String,

    #[default(false)]
    pub is_archived: bool,

    #[default(jiff::Timestamp::now())]
    pub created_at: jiff::Timestamp,

    #[has_many]
    messages: toasty::HasMany<SessionMessage>,
    #[belongs_to(key = project_id, references = id)]
    project: toasty::BelongsTo<Project>,
}

impl Session {
    pub async fn get_project(&mut self, mut db: toasty::Db) -> toasty::Result<Project> {
        Project::get_by_id(&mut db, self.project_id).await
    }

    pub async fn get_messages(&self, mut db: toasty::Db) -> toasty::Result<Vec<SessionMessage>> {
        self.messages().exec(&mut db).await
    }
}

#[derive(Debug, Model)]
pub struct SessionMessage {
    #[key]
    #[auto]
    pub id: i32,

    #[index]
    pub session_id: i32,

    pub role: String,    // e.g., "user", "assistant", "system", etc.
    pub content: String, // the actual message content, e.g., user input or model response
    pub reasoning_content: Option<String>, // optional field for reasoning or thought process
    pub finish_reason: String, // e.g., "stop", "length", "error", etc.
    pub extra_metadata: String, // JSON string for any additional metadata

    #[default(jiff::Timestamp::now())]
    pub sent_at: jiff::Timestamp,

    #[belongs_to(key = session_id, references = id)]
    session: toasty::BelongsTo<Session>,
}

impl SessionMessage {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "role": self.role,
            "content": self.content,
            "reasoning_content": self.reasoning_content,
            "finish_reason": self.finish_reason,
            "extra_metadata": self.extra_metadata,
            "sent_at": self.sent_at.to_string(),
        })
    }
}
