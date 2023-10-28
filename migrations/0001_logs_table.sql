-- Add migration script here
CREATE TABLE logs
(
	id uuid NOT NULL PRIMARY KEY,
	channel_id int8 NOT NULL,
	count int NOT NULL,
    time int8 NOT NULL
);
