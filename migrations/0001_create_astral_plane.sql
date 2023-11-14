CREATE TABLE IF NOT EXISTS astral_plane (
	short_name VARCHAR PRIMARY KEY,
	full_name VARCHAR NOT NULL,
	settlement_uri VARCHAR NOT NULL,
	landing_uri VARCHAR NOT NULL,
	landing_count SMALLINT NOT NULL DEFAULT 0 CHECK (landing_count >= 0)
);
