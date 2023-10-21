CREATE TABLE IF NOT EXISTS files (
    id UUID PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    note VARCHAR(64) DEFAULT NULL,
    password CHAR(64) DEFAULT NULL,
    downloads INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expired_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP DEFAULT NULL
)