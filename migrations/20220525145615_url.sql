-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE urls
(
    id        uuid DEFAULT uuid_generate_v4(),
    short_url VARCHAR NOT NULL,
    long_url  VARCHAR NOT NULL,
    PRIMARY KEY (id)
);