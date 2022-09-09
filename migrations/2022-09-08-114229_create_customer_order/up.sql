CREATE TABLE customer_orders (
	id SERIAL PRIMARY KEY,
	customer_name VARCHAR NOT NULL,
	item VARCHAR NOT NULL,
	cost DOUBLE PRECISION NOT NULL,
	order_status INT NOT NULL
);