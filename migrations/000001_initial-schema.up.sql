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

INSERT INTO "public"."restaurant"("id","created_at","name","address","logo","cover","location_url")
VALUES
(E'40ec87c9-d735-4a3f-b8f1-0fabeda6975b',E'2019-02-23 02:17:27.949108',E'The Bros Coffee & Coworking Space',E'Komplek Mayang dan Hompila, Jl. Buring No. 1, Oro-oro Dowo, Klojen, Kota Malang, Jawa Timur 65112',E'http://thebros.co/wp-content/uploads/2018/12/Asset-2-1.png',E'http://thebros.co/wp-content/uploads/2017/01/Bros-Mason-Square.jpg',E'https://goo.gl/maps/tghbVU2s1qR2');

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

INSERT INTO "public"."customer"("id","created_at","name","phone","email")
VALUES
(E'a8b766c6-e73b-42dd-ab98-ae87a7c4eefa',E'2019-02-23 02:26:29.386201',E'Rizki Romadhoni',E'085727204495',NULL);

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
    is_active boolean NOT NULL DEFAULT false,
    email character varying(50) NOT NULL,
    phone character varying(20) NOT NULL,
    picture character varying(225) NOT NULL,
    restaurant_id uuid REFERENCES restaurant(id)
);

INSERT INTO "public"."partner"("id","created_at","name","username","hashed_password","is_active","email","phone","picture","restaurant_id")
VALUES
(E'a442152d-d974-49c7-bb6c-abcbbc8997fe',E'2019-02-23 03:04:52.397895',E'Rizki Romadhoni',E'rrmdn',E'80e6317fe56ba04464c31002ff2d8aa5e9a0fbaa8f3f24646f4ecd33',FALSE,E'donny.staark@gmail.com',E'085727204495',E'https://instagram.fsub8-1.fna.fbcdn.net/vp/fc774036b984b9819ab156036c232504/5D26D207/t51.2885-19/s320x320/44249813_899149023808488_6130789567038488576_n.jpg?_nc_ht=instagram.fsub8-1.fna.fbcdn.net&_nc_cat=100',E'40ec87c9-d735-4a3f-b8f1-0fabeda6975b');

