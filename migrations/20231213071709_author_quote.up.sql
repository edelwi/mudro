-- Add down migration script here
CREATE TABLE author (
    id SERIAL PRIMARY KEY,
    author_name varchar(100) NOT NULL,
    CONSTRAINT auth_name_unq UNIQUE(author_name)
);

CREATE TABLE quote (
    id SERIAL PRIMARY KEY,
    text varchar NOT NULL,
    author_id int NOT NULL,
    FOREIGN KEY(author_id) REFERENCES author(id) ON DELETE CASCADE,
    CHECK (text <> ''),
    CONSTRAINT quote_unq UNIQUE(author_id, text)
);

