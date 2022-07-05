CREATE TABLE reports
(
    id       SERIAL    NOT NULL,
    author   INTEGER   NOT NULL,
    date     TIMESTAMP NOT NULL,
    user_id  INTEGER   NOT NULL,
    user_msg Text      NOT NULL,
    PRIMARY KEY (id)
)