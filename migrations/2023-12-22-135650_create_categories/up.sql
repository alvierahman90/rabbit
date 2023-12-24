CREATE TABLE categories (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	user_id SERIAL REFERENCES users(id)
);
