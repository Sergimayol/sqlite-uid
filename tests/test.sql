SELECT load_extension('./target/release/libsqlite_uid', 'sqlite3_uid_init');

.bail on
.header on
.mode box

.timer on

-- ULID
SELECT ulid() as ulid;
SELECT ULID() as ulid;

WITH ulid_table AS (
    SELECT ulid() AS id
    UNION ALL
    SELECT ulid()
    UNION ALL
    SELECT ulid()
    UNION ALL
    SELECT ulid()
)
SELECT * FROM ulid_table ORDER BY id;

-- ObjectId
SELECT object_id() as object_id;

WITH object_id_table AS (
    SELECT object_id() AS id
    UNION ALL
    SELECT object_id()
    UNION ALL
    SELECT object_id()
    UNION ALL
    SELECT object_id()
)
SELECT * FROM object_id_table ORDER BY id;

-- UUID (v4)
SELECT uuid() as uuid;

WITH uuid_table AS (
    SELECT uuid() AS id
    UNION ALL
    SELECT uuid()
    UNION ALL
    SELECT uuid()
    UNION ALL
    SELECT uuid()
)
SELECT * FROM uuid_table;

-- Snowflake Id
SELECT snowflake_id(1, 1700000000000) as snowflake_id;

WITH snowflake_id_table AS (
    SELECT snowflake_id(1, 1700000000000) AS id
    UNION ALL
    SELECT snowflake_id(1, 1700000000000)
    UNION ALL
    SELECT snowflake_id(1, 1700000000000)
    UNION ALL
    SELECT snowflake_id(1, 1700000000000)
)
SELECT * FROM snowflake_id_table ORDER BY id;

-- https://stackoverflow.com/questions/76342920/unsafe-use-of-virtual-table-when-creating-triggers-for-sqlite-full-text-search#:~:text=This%20is%20a,%3D1%3B
PRAGMA trusted_schema=1;

CREATE TABLE TEST (
    uuid TEXT UNIQUE default(uuid()),
    ulid TEXT UNIQUE default(ulid()),
    object_id TEXT UNIQUE default(object_id()),
    snowflake_id TEXT UNIQUE default(snowflake_id(1, 1700000000000)),
    data TEXT
);

INSERT INTO TEST (data) VALUES
    ('Hello'),
    ('Hola'),
    ('random data'),
    ('test for object_id');

SELECT * FROM TEST;
