pub mod project;
pub mod session;

pub use project::Project;
pub use session::{Session, SessionMessage};

/// Helper macro to include all storage models in the database connection setup.
#[macro_export]
macro_rules! storage_models {
    () => {
        toasty::models!(
            $crate::storage::models::Project,
            $crate::storage::models::Session,
            $crate::storage::models::SessionMessage,
        )
    };
}
