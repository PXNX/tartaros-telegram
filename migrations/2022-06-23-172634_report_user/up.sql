CREATE TABLE users
(
    id                 INTEGER   NOT NULL,
    creation_timestamp TIMESTAMP NOT NULL,
    reported_message   TEXT      NOT NULL,
    PRIMARY KEY (id)
)