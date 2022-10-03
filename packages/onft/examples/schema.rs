use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use std::env::current_dir;
use std::fs::create_dir_all;

use onft::{msg::ONFTExecuteMsg, query::ONFTQuery, types::*};


fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(ONFTExecuteMsg), &out_dir);
    export_schema(&schema_for!(ONFTQuery), &out_dir);
    export_schema(&schema_for!(Denom), &out_dir);
    export_schema(&schema_for!(ONFT), &out_dir);
    export_schema(&schema_for!(Metadata), &out_dir);
    export_schema(&schema_for!(Owner), &out_dir);
    export_schema(&schema_for!(IDCollection), &out_dir);
    export_schema(&schema_for!(Collection), &out_dir);
}