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