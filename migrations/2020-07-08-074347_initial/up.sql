CREATE TABLE rooms (
    id TEXT NOT NULL PRIMARY KEY,
    max_occupancy INTEGER NOT NULL
);

CREATE TABLE users (
    id TEXT NOT NULL PRIMARY KEY,
    -- can be NULL if LDAP should be used
    password_hash TEXT
);

CREATE TABLE presencies (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "from" TIMESTAMP NOT NULL,
    "to" TIMESTAMP NOT NULL,
    user TEXT NOT NULL REFERENCES user(id),
    room TEXT NOT NULL REFERENCES room(id)
);