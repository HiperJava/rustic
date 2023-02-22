CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "sensors"
(
    id        UUID    NOT NULL DEFAULT uuid_generate_v4(),
    dimension VARCHAR NOT NULL
)
