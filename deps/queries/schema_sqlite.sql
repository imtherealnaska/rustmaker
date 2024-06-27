-- entries
DROP TABLE IF EXISTS entries;

CREATE TABLE entries (
    -- Internal unique ID.
    id              INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Publicly visible unique ID (used in public APIs such as submissions and corrections).
    guid            TEXT NOT NULL UNIQUE DEFAULT (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-' || '4' || substr(lower(hex(randomblob(2))),2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))),2) || '-' || lower(hex(randomblob(6)))),

    -- Actual language content. Dictionary word or definition entries
    content         TEXT NOT NULL CHECK (content <> ''),

    -- The first “alphabet” of the content. For English, for the word Apple, the initial is A
    initial         TEXT NOT NULL DEFAULT '',

    -- An optional numeric value to order search results
    weight          DECIMAL NOT NULL DEFAULT 0,

    -- Fulltext search tokens. (SQLite doesn't support full-text search tokens like PostgreSQL)
    tokens          TEXT NOT NULL DEFAULT '',

    -- String representing the language of content. Eg: en, english
    lang            TEXT NOT NULL CHECK (lang <> ''),

    -- Optional tags
    tags            TEXT NOT NULL DEFAULT '',

    -- Phonetic (pronunciation) descriptions of the content. Eg: {ap(ə)l, aapl} for Apple
    phones          TEXT NOT NULL DEFAULT '',

    -- Optional text notes
    notes           TEXT NOT NULL DEFAULT '',

    -- Optional arbitrary metadata
    meta            TEXT NOT NULL DEFAULT '{}',

    status          TEXT NOT NULL DEFAULT 'enabled',
    created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP
);

DROP INDEX IF EXISTS idx_entries_content;
CREATE INDEX idx_entries_content ON entries((LOWER(SUBSTR(content, 1, 50))));
DROP INDEX IF EXISTS idx_entries_initial;
CREATE INDEX idx_entries_initial ON entries(initial);
DROP INDEX IF EXISTS idx_entries_lang;
CREATE INDEX idx_entries_lang ON entries(lang);
DROP INDEX IF EXISTS idx_entries_tokens;
-- No direct equivalent for GIN indexes in SQLite3, using a normal index instead
CREATE INDEX idx_entries_tokens ON entries(tokens);
DROP INDEX IF EXISTS idx_entries_tags;
CREATE INDEX idx_entries_tags ON entries(tags);

-- relations
DROP TABLE IF EXISTS relations;

CREATE TABLE relations (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    from_id         INTEGER REFERENCES entries(id) ON DELETE CASCADE ON UPDATE CASCADE,
    to_id           INTEGER REFERENCES entries(id) ON DELETE CASCADE ON UPDATE CASCADE,

    types           TEXT NOT NULL DEFAULT '',
    tags            TEXT NOT NULL DEFAULT '',
    notes           TEXT NOT NULL DEFAULT '',
    weight          DECIMAL DEFAULT 0,

    status          TEXT NOT NULL DEFAULT 'enabled',
    created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP
);

DROP INDEX IF EXISTS idx_relations;
CREATE UNIQUE INDEX idx_relations ON relations(from_id, to_id);

-- comments
-- This table holds change suggestions submitted by the public. It can either be on an entry
-- or on a relation.
DROP TABLE IF EXISTS comments;

CREATE TABLE comments (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    from_id        INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE ON UPDATE CASCADE,
    to_id          INTEGER NULL REFERENCES entries(id) ON DELETE CASCADE ON UPDATE CASCADE,
    comments       TEXT NOT NULL DEFAULT '',

    created_at      DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- settings
DROP TABLE IF EXISTS settings;

CREATE TABLE settings (
    key             TEXT NOT NULL UNIQUE,
    value           TEXT NOT NULL DEFAULT '{}',
    updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP
);

DROP INDEX IF EXISTS idx_settings_key;
CREATE INDEX idx_settings_key ON settings(key);
