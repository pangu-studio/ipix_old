
ALTER TABLE m_repo ADD COLUMN addition TEXT DEFAULT '{}';

-- --------------------------------------
-- Table structure for m_storage_account
-- --------------------------------------
CREATE TABLE IF NOT EXISTS m_storage_account (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    app_key TEXT NOT NULL,
    secret TEXT NOT NULL,
    create_time TIMESTAMP NOT NULL,
    provider INTEGER DEFAULT 0,
    addition TEXT NOT NULL DEFAULT '{}',
    deleted BOOLEAN NOT NULL DEFAULT 0
);

-- ---------------------------------
-- Table structure for m_tag_upload
-- ---------------------------------
CREATE TABLE IF NOT EXISTS m_tag_upload (
    thid INTEGER PRIMARY KEY,
    tid INTEGER NOT NULL,
    hid INTEGER NOT NULL
);

-- ---------------------------------
-- Table structure for m_tags
-- ---------------------------------
CREATE TABLE IF NOT EXISTS m_tag (
    tid INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

-- ------------------------------------
-- Table structure for m_upload_history
-- ------------------------------------
CREATE TABLE IF NOT EXISTS m_upload_history (
    hid INTEGER PRIMARY KEY,
    key TEXT NOT NULL,
    url TEXT,
    description TEXT,
    repo_id TEXT NOT NULL,
    status INTEGER NOT NULL DEFAULT 0,
    addition TEXT NOT NULL DEFAULT '{}',
    deleted BOOLEAN NOT NULL DEFAULT 0
);

-- -----------------------------------
-- Indexes structure for table m_repo
-- -----------------------------------
CREATE INDEX "idx_repo_deleted"
ON "m_repo" (
  "deleted" ASC
);

-- ----------------------------------------
-- Indexes structure for table m_tag_upload
-- ----------------------------------------
CREATE INDEX "idx_tag_id"
ON "m_tag_upload" (
  "tid" ASC
);
CREATE INDEX "idx_upload_id"
ON "m_tag_upload" (
  "hid" DESC
);

-- -------------------------------------
-- Indexes structure for table m_tag
-- -------------------------------------
CREATE INDEX "uqx_tag_name"
ON "m_tag" (
  "name" ASC
);

-- ----------------------------------------------
-- Indexes structure for table m_upload_history
-- ----------------------------------------------
CREATE INDEX "idx_history_deleted"
ON "m_upload_history" (
  "deleted" ASC
);
CREATE INDEX "idx_history_repo_id"
ON "m_upload_history" (
  "repo_id" ASC
);
