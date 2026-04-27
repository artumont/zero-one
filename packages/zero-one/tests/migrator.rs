use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::time::{SystemTime, UNIX_EPOCH};

use zero_one::storage::migrator::Migrator;

fn temp_db_path(suffix: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "z1_migrator_integration_test_{}_{}_{}.db",
        std::process::id(),
        now,
        suffix
    ))
}

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut cx = Context::from_waker(&waker);
    let mut future = Box::pin(future);
    loop {
        match Pin::new(&mut future).poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn expected_embedded_migration_count() -> usize {
    let migrations_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    if !migrations_dir.exists() {
        return 0;
    }

    migrations_dir
        .read_dir()
        .expect("failed to read migrations dir")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_dir()
                && path.join("metadata.json").exists()
                && path.join("up.sql").exists()
                && path.join("down.sql").exists()
        })
        .count()
}

#[test]
fn new_creates_migration_tracking_table() {
    let db_path = temp_db_path("new_creates_table");
    let _migrator = Migrator::new(&db_path).expect("failed to construct migrator");

    let conn = rusqlite::Connection::open(&db_path).expect("failed to re-open db");
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='__z1_migrations'")
        .expect("failed to prepare table existence query");
    let found = stmt
        .query_row([], |row| row.get::<_, String>(0))
        .expect("failed to query migrations table")
        == "__z1_migrations";

    assert!(found);
    std::fs::remove_file(db_path).ok();
}

#[test]
fn run_applies_embedded_migrations_and_is_idempotent() {
    let db_path = temp_db_path("run_idempotent");
    let migrator = Migrator::new(&db_path).expect("failed to construct migrator");

    block_on(migrator.run()).expect("first migration run should succeed");
    block_on(migrator.run()).expect("second migration run should be idempotent");

    let conn = rusqlite::Connection::open(&db_path).expect("failed to open db for verification");
    let applied_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM __z1_migrations", [], |row| row.get(0))
        .expect("failed to count applied migrations");

    let expected = expected_embedded_migration_count() as i64;
    assert_eq!(applied_count, expected);

    if expected > 0 {
        let project_table_exists: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='project'",
                [],
                |row| row.get(0),
            )
            .expect("failed to query project table existence");
        assert_eq!(project_table_exists, 1);
    }

    std::fs::remove_file(db_path).ok();
}
