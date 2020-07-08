CREATE TABLE room (
    id TEXT NOT NULL PRIMARY KEY,
    max_occupancy INTEGER NOT NULL
);

CREATE TABLE user (
    id TEXT NOT NULL PRIMARY KEY,
    -- can be NULL if LDAP should be used
    password_hash TEXT
);

CREATE TABLE presency (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    "datetime" TEXT NOT NULL,
    user TEXT REFERENCES user(id)
);