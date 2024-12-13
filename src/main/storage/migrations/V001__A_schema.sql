CREATE TABLE event
(
    id        VARCHAR(64) NOT NULL PRIMARY KEY,
    timestamp INT         NOT NULL,
    kind      INT         NOT NULL,
    tags      jsonb       NOT NULL,
);