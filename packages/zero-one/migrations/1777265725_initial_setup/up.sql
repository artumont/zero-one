BEGIN;

CREATE TABLE IF NOT EXISTS "project" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "name" TEXT NOT NULL,
  "worktree" TEXT NOT NULL,
  "vcs" TEXT NOT NULL,
  "root_path" TEXT NOT NULL,
  "is_archived" INTEGER NOT NULL DEFAULT 0,
  "created_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "session" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "project_id" INTEGER NOT NULL,
  "title" TEXT NOT NULL,
  "is_archived" INTEGER NOT NULL DEFAULT 0,
  "created_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY("project_id") REFERENCES "project"("id") ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS "idx_session_project_id" ON "session" ("project_id");

CREATE TABLE IF NOT EXISTS "session_message" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "session_id" INTEGER NOT NULL,
  "role" TEXT NOT NULL,
  "content" TEXT NOT NULL,
  "reasoning_content" TEXT,
  "finish_reason" TEXT NOT NULL,
  "extra_metadata" TEXT NOT NULL,
  "sent_at" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY("session_id") REFERENCES "session"("id") ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS "idx_session_message_session_id" ON "session_message" ("session_id");

COMMIT;