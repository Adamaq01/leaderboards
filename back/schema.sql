IF NOT EXISTS (
   SELECT name
   FROM sys.databases
   WHERE name = N'leaderboards'
) CREATE DATABASE leaderboards COLLATE Latin1_General_CS_AI
GO

USE leaderboards
GO

CREATE TABLE users (
	id 				int				PRIMARY KEY NOT NULL IDENTITY(1, 1),
	display_name	varchar(32)		NOT NULL,
	username		varchar(32)		UNIQUE NOT NULL,
	salt			varchar(255)	NOT NULL,
	password		varchar(255)	NOT NULL
);

CREATE TABLE followers (
	id 				int				PRIMARY KEY NOT NULL IDENTITY(1, 1),
	follower		int				FOREIGN KEY REFERENCES users NOT NULL,
	following		int				FOREIGN KEY REFERENCES users NOT NULL,
);

CREATE TABLE categories (
	id				int				PRIMARY KEY NOT NULL IDENTITY(1, 1),
	user_id			int				FOREIGN KEY REFERENCES users NOT NULL,
	display_name	varchar(32)		NOT NULL,
	description		varchar(255)
);

CREATE TABLE categories_likes (
	id				int				PRIMARY KEY NOT NULL IDENTITY(1, 1),
	user_id			int				FOREIGN KEY REFERENCES users NOT NULL,
	category_id		int				FOREIGN KEY REFERENCES categories NOT NULL
);

CREATE TABLE leaderboards (
	id				int				PRIMARY KEY NOT NULL IDENTITY(1, 1),
	user_id			int				FOREIGN KEY REFERENCES users NOT NULL,
	display_name	varchar(100)	NOT NULL,
	description		varchar(1000),
	category_id		int				FOREIGN KEY REFERENCES categories NOT NULL,
	ordering		int				NOT NULL
);

CREATE TABLE leaderboards_likes (
	id				int				PRIMARY KEY NOT NULL IDENTITY(1, 1),
	user_id			int				FOREIGN KEY REFERENCES users NOT NULL,
	leaderboard_id	int				FOREIGN KEY REFERENCES leaderboards NOT NULL
);

CREATE TABLE scores (
	id				int				PRIMARY KEY NOT NULL IDENTITY(1, 1),
	user_id			int				FOREIGN KEY REFERENCES users NOT NULL,
	leaderboard_id	int				FOREIGN KEY REFERENCES leaderboards NOT NULL,
	score			int				NOT NULL
);

CREATE TABLE scores_likes (
	id				int				PRIMARY KEY NOT NULL IDENTITY(1, 1),
	user_id			int				FOREIGN KEY REFERENCES users NOT NULL,
	score_id		int				FOREIGN KEY REFERENCES scores NOT NULL
);

CREATE TABLE comments (
	id				int				PRIMARY KEY NOT NULL IDENTITY(1, 1),
	user_id			int				FOREIGN KEY REFERENCES users NOT NULL,
	leaderboard_id	int				FOREIGN KEY REFERENCES leaderboards NOT NULL,
	timestamp		datetime		DEFAULT CURRENT_TIMESTAMP NOT NULL,
	message			varchar(1000)
);

CREATE TABLE comments_likes (
	id				int				PRIMARY KEY NOT NULL IDENTITY(1, 1),
	user_id			int				FOREIGN KEY REFERENCES users NOT NULL,
	comment_id		int				FOREIGN KEY REFERENCES comments NOT NULL
);

GO
