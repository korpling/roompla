CREATE TABLE rooms (
    id TEXT NOT NULL PRIMARY KEY,
    max_occupancy INTEGER NOT NULL,
    timezone TEXT
);

CREATE TABLE users (
    id TEXT NOT NULL PRIMARY KEY,
    display_name TEXT NOT NULL,
    contact_info TEXT NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE occupancies (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "start" TIMESTAMP NOT NULL,
    "end" TIMESTAMP NOT NULL,
    user_id TEXT NOT NULL,
    user_name TEXT NOT NULL,
    user_contact TEXT NOT NULL,
    room TEXT NOT NULL REFERENCES rooms(id)
);

CREATE INDEX idx_occ_start  ON occupancies("start");
CREATE INDEX idx_occ_end  ON occupancies("end");
CREATE INDEX idx_occ_room ON occupancies(room);
CREATE INDEX idx_occ_user_id ON occupancies(user_id);
CREATE INDEX idx_occ_user_name ON occupancies(user_name);