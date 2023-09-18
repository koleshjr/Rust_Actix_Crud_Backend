-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; --provides functions for generating UUIDS

CREATE TABLE 
    IF NOT EXISTS notes (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()), --generate a UUID value as the primary key
        title VARCHAR(255) NOT NULL UNIQUE,
        content TEXT NOT NULL,
        category VARCHAR(100),
        published BOOLEAN DEFAULT FALSE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP 
        WITH
            TIME ZONE DEFAULT NOW()
    ); 