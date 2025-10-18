use std::{error::Error, fs::File, io::BufReader};

use crate::structs::dump_data::DumpData;

pub fn display_prods(json_filename: String) -> Result<(), Box<dyn Error>> {
    let file = File::open(json_filename)?;
    let reader = BufReader::new(file);
    let dump: DumpData = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("❌ JSON read error: {}", err);
            return Ok(());
        }
    };
    println!("{}", dump.prods.as_ref().map_or(0, |v| v.len()));
    Ok(())
}
