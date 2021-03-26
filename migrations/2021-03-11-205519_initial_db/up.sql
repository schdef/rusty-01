CREATE TABLE vaultstore
(
    id SERIAL PRIMARY KEY,
    created_timestamp timestamp
    without time zone,
    context character varying
    (255),
    entity_key character varying
    (255),
    entity_value character varying
    (255)
);