CREATE TABLE posts (
  id BIGSERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  body TEXT NOT NULL
);

INSERT INTO posts (title, body) VALUES
  ('First Post', 'This is my first post'),
  ('Second Post', 'This is my second post');
