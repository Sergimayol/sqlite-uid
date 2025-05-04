mod object_id;
mod ulid;

use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, define_scalar_function, Result};
use uuid::Uuid;

fn ulid(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    let ulid = ulid::Ulid::new();
    api::result_text(context, ulid.to_string())?;
    Ok(())
}

fn object_id(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    let object_id = object_id::ObjectId::new();
    api::result_text(context, object_id.to_hex_string())?;
    Ok(())
}

fn uuid(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    let uuid = Uuid::new_v4();
    api::result_text(context, uuid.to_string())?;
    Ok(())
}

fn snowflake_id(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let _name = api::value_text(values.get(0).expect("1st argument as name"))?;
    api::result_text(context, "todo: snowflake_id")?;
    Ok(())
}

#[sqlite_entrypoint]
pub fn sqlite3_uid_init(db: *mut sqlite3) -> Result<()> {
    let flags = FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC;
    define_scalar_function(db, "uuid", 0, uuid, flags)?;
    define_scalar_function(db, "ulid", 0, ulid, flags)?;
    define_scalar_function(db, "object_id", 0, object_id, flags)?;
    define_scalar_function(db, "snowflake_id", 0, snowflake_id, flags)?;
    Ok(())
}
