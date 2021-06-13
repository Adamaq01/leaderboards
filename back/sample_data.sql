USE leaderboards
GO

-- Insert some users
INSERT INTO users (display_name, username, salt, password)
VALUES ('Admin', 'admin', '4ocGSVtC8uK892B9dFPVGKHXDTaaT0Rxss6ea4BQ5laPFN0gWZhiRwEYtHvRn0g60Od3ICvdNZ0lyQyDj6qE4cIwJC21lurUJTEHvhd504f5uW3jWhPyBhCjk1p8GgKz', '$argon2id$v=19$m=4096,t=3,p=1$NG9jR1NWdEM4dUs4OTJCOWRGUFZHS0hYRFRhYVQwUnhzczZlYTRCUTVsYVBGTjBnV1poaVJ3RVl0SHZSbjBnNjBPZDNJQ3ZkTlowbHlReURqNnFFNGNJd0pDMjFsdXJVSlRFSHZoZDUwNGY1dVczaldoUHlCaENqazFwOEdnS3o$440XjIVOe/KFSrO/ZdvJcvlxkFEeE5mL1CLdOBsa2XY');
INSERT INTO users (display_name, username, salt, password)
VALUES ('Adam Thibert', 'adam.thibert', '8cfddJPOXxZX4jzsDF4ZcNeUY8blNKBwHppyStSWYPBVC6RSaZIYijRJ4GnhEGG2FPqdYx6P4IdRNAZpxjE7ckblxv6aYWHvwBnwijn2upBcghnRlQ5ZqJHXpYsAtGRh', '$argon2id$v=19$m=4096,t=3,p=1$OGNmZGRKUE9YeFpYNGp6c0RGNFpjTmVVWThibE5LQndIcHB5U3RTV1lQQlZDNlJTYVpJWWlqUko0R25oRUdHMkZQcWRZeDZQNElkUk5BWnB4akU3Y2tibHh2NmFZV0h2d0Jud2lqbjJ1cEJjZ2huUmxRNVpxSkhYcFlzQXRHUmg$UZrAmkPDLINo/7RQ9pgZVN0BujHJHXYFVn2a4/2dsGw');
GO

-- Make the first user follow the second one
INSERT INTO followers (follower, following)
VALUES (1, 2);
GO

-- Insert a sample category
INSERT INTO categories (user_id, display_name, description)
VALUES (1, 'Video Games', 'Video Games related leaderboards');
GO

-- Insert a sample leaderboard
INSERT INTO leaderboards (user_id, display_name, description, category_id, ordering)
VALUES (1, 'Pac-Man Scores', 'Upload your original Pac-Man scores here :)', 1, 0);
GO
INSERT INTO leaderboards (user_id, display_name, description, category_id, ordering)
VALUES (1, 'Pac-Man Scores', 'Upload your original Pac-Man scores here :)', 1, 0);
GO

-- Insert a like from the second user on the sample leaderboard
INSERT INTO leaderboards_likes (user_id, leaderboard_id)
VALUES (2, 1);
GO

-- Insert a comment from the second user on the sample leaderboard
INSERT INTO comments (user_id, leaderboard_id, message)
VALUES (2, 1, 'I like Pac-Man so much ! Look at my score');
GO

-- Insert a score from the second user on the sample leaderboard
INSERT INTO scores (score, user_id, leaderboard_id)
VALUES (23248, 2, 1);
GO

-- Insert another comment from the second user on the sample leaderboard
INSERT INTO comments (user_id, leaderboard_id, message)
VALUES (2, 1, 'Oh I forgot to upload it, now it should be there :)');
GO
