CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE restaurant (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    name character varying(50) NOT NULL,
    address text NOT NULL,
    logo text NOT NULL,
    cover text NOT NULL,
    location_url text NOT NULL
);

CREATE TABLE dining_table (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    restaurant_id uuid REFERENCES restaurant(id),
    name character varying(50) NOT NULL
);

CREATE TABLE dish (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    name character varying(50) NOT NULL,
    price bigint NOT NULL,
    description text NOT NULL,
    restaurant_id uuid REFERENCES restaurant(id)
);

CREATE TABLE customer (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    name character varying(50),
    phone character varying(20),
    email character varying(30)
);

CREATE TABLE device_info (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    customer_id uuid REFERENCES customer(id),
    os character varying(20),
    os_version character varying(20),
    user_agent text,
    created_at timestamp without time zone DEFAULT now()
);

CREATE TABLE customer_order (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    restaurant_id uuid REFERENCES restaurant(id),
    dining_table_id uuid REFERENCES dining_table(id),
    customer_id uuid REFERENCES customer(id)
);

CREATE TABLE dish_order (
    id uuid PRIMARY KEY,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    dish_id uuid REFERENCES dish(id),
    customer_order_id uuid REFERENCES customer_order(id)
);

CREATE TABLE partner (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    name character varying(50) NOT NULL,
    username character varying(50) NOT NULL,
    hashed_password text NOT NULL,
    email character varying(50) NOT NULL,
    phone character varying(20) NOT NULL,
    picture character varying(225) NOT NULL,
    restaurant_id uuid REFERENCES restaurant(id)
);

