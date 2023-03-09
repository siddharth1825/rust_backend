-- Your SQL goes heCRE

CREATE TABLE "song" (
    id VARCHAR(255) PRIMARY KEY,
    link VARCHAR NOT NULL UNIQUE,
    user_email VARCHAR(255) NOT NULL REFERENCES "user"(email)
);

