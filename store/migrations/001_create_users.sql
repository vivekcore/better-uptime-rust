CREATE TYPE status AS ENUM ('Up', 'Down', 'Unknown');

CREATE TABLE users (
    id        UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    username   TEXT NOT NULL UNIQUE,
    password   TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);


CREATE TABLE websites (
    id                      UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    url                     TEXT NOT NULL,
    is_active               BOOLEAN DEFAULT TRUE,
    check_interval_seconds  INT DEFAULT 60,
    time_added              TIMESTAMPTZ NOT NULL DEFAULT now(),
    user_id                 UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE regions (
    id      UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    name    TEXT UNIQUE NOT NULL
);

CREATE TABLE website_ticks (
    id               UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    response_time_ms INT ,
    website_status   status NOT NULL DEFAULT 'Unknown',
    status_code      INT,
    website_id        UUID NOT NULL REFERENCES websites(id) ON DELETE CASCADE,
    region_id         UUID NOT NULL REFERENCES regions(id) ON DELETE CASCADE,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);



CREATE INDEX idx_website_user_id ON websites(user_id);
CREATE INDEX idx_ticks_website_created ON website_ticks(website_id, created_at DESC);