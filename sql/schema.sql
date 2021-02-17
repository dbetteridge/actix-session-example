DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE testing.users (
	email VARCHAR(200) NOT NULL PRIMARY KEY,
	name VARCHAR(200) NOT NULL,
	username    VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(50),
	UNIQUE (username)
);

CREATE TABLE testing.sessions (
	"user" VARCHAR(200) PRIMARY KEY,
	"token" VARCHAR(200) NOT NULL,
	created TIMESTAMP DEFAULT NOW(),
	CONSTRAINT fk_user_sessions
	FOREIGN KEY("user")
	REFERENCES testing.users(email)
	ON DELETE CASCADE
);

CREATE TABLE testing.cards (
	id BIGSERIAL PRIMARY KEY,
	"owner" VARCHAR(200),
	created TIMESTAMP DEFAULT NOW(),
  	"name" text,
  	description text,
	CONSTRAINT fk_owner_user
	FOREIGN KEY("owner")
	REFERENCES testing.users(email)
	ON DELETE CASCADE
);

CREATE TABLE testing.trades (
	id BIGSERIAL PRIMARY KEY,
	sender VARCHAR(200),
  	recipient VARCHAR(200),
	created DATE DEFAULT NOW(),
  	card BIGINT,
	CONSTRAINT fk_sender_user
	FOREIGN KEY(sender)
	REFERENCES testing.users(email)
	ON DELETE CASCADE,
	CONSTRAINT fk_recipient_user
	FOREIGN KEY(recipient)
	REFERENCES testing.users(email)
	ON DELETE CASCADE,
	CONSTRAINT fk_card
	FOREIGN KEY(card)
	REFERENCES testing.cards(id)
	ON DELETE CASCADE
);