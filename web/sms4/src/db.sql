PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS users (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    username     TEXT UNIQUE NOT NULL,
    display_name TEXT NOT NULL,
    bio          TEXT,
    avatar       TEXT,
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS posts (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id      INTEGER NOT NULL,
    content      TEXT NOT NULL,
    parent_id    INTEGER,
    likes_count  INTEGER NOT NULL DEFAULT 0,
    replies_count INTEGER NOT NULL DEFAULT 0,
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at   TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (parent_id) REFERENCES posts(id)
);

CREATE TABLE IF NOT EXISTS follows (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    follower_id  INTEGER NOT NULL,
    followee_id  INTEGER NOT NULL,
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (follower_id) REFERENCES users(id),
    FOREIGN KEY (followee_id) REFERENCES users(id),
    UNIQUE(follower_id, followee_id)
);

CREATE TABLE IF NOT EXISTS likes (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id      INTEGER NOT NULL,
    post_id      INTEGER NOT NULL,
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (post_id) REFERENCES posts(id),
    UNIQUE(user_id, post_id)
);

CREATE TABLE IF NOT EXISTS messages (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    sender_id    INTEGER NOT NULL,
    receiver_id  INTEGER NOT NULL,
    content      TEXT NOT NULL,
    read         INTEGER NOT NULL DEFAULT 0,
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (sender_id) REFERENCES users(id),
    FOREIGN KEY (receiver_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS profiles (
    user_id      INTEGER PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    birthday     TEXT,
    gender       TEXT,
    city         TEXT,
    occupation   TEXT,
    education    TEXT,
    height       INTEGER,
    looking_for  TEXT DEFAULT 'friend',
    about_me     TEXT,
    updated_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS interests (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id      INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tag          TEXT NOT NULL COLLATE NOCASE,
    UNIQUE(user_id, tag)
);

CREATE INDEX IF NOT EXISTS idx_interests_tag ON interests(tag);
CREATE INDEX IF NOT EXISTS idx_interests_user ON interests(user_id);
