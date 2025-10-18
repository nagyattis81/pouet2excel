use crate::structs::dump_data::DumpData;
use rust_xlsxwriter::Workbook;
use std::{error::Error, fs::File, io::BufReader};

pub fn export_prods(json_filename: String) -> Result<(), Box<dyn Error>> {
    let file = File::open(&json_filename)?;
    let reader = BufReader::new(file);
    let dump: DumpData = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("❌ JSON read error: {}", err);
            return Ok(());
        }
    };

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write_string(0, 0, "Id")?;
    worksheet.write_string(0, 1, "Name")?;
    worksheet.write_string(0, 2, "Groups")?;
    worksheet.write_string(0, 3, "Type")?;
    worksheet.write_string(0, 4, "Date")?;
    worksheet.write_string(0, 5, "Down")?;
    worksheet.write_string(0, 6, "Pig")?;
    worksheet.write_string(0, 7, "Up")?;
    worksheet.write_string(0, 8, "Avg")?;
    worksheet.write_string(0, 9, "Download")?;

    if let Some(prods) = &dump.prods {
        for (row, prod) in prods.iter().enumerate() {
            let excel_row = (row + 1) as u32;

            if let Some(value) = prod.id {
                worksheet.write_string(excel_row, 0, format!("{}", value))?;
            }

            if let Some(value) = &prod.name {
                worksheet.write_string(excel_row, 1, value)?;
            }

            if let Some(value) = &prod.groups {
                let joined = value
                    .iter()
                    .filter_map(|t| t.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                worksheet.write_string(excel_row, 2, &joined)?;
            }

            if let Some(value) = &prod.prod_type {
                let joined = value
                    .iter()
                    .map(|t| format!("{:?}", t))
                    .collect::<Vec<_>>()
                    .join(", ");
                worksheet.write_string(excel_row, 3, &joined)?;
            }

            if let Some(value) = &prod.added_date {
                worksheet.write_string(excel_row, 4, value)?;
            }

            if let Some(value) = &prod.votedown {
                worksheet.write_number(excel_row, 5, *value)?;
            }

            if let Some(value) = &prod.votepig {
                worksheet.write_number(excel_row, 6, *value)?;
            }

            if let Some(value) = &prod.voteup {
                worksheet.write_number(excel_row, 7, *value)?;
            }

            if let Some(value) = &prod.voteavg {
                worksheet.write_number(excel_row, 8, *value)?;
            }

            if let Some(value) = &prod.download {
                worksheet.write_string(excel_row, 9, value)?;
            }
        }
    }

    let excel_file_name = format!("{}.xlsx", json_filename);
    workbook.save(&excel_file_name)?;
    println!("✅ Write excel {}", excel_file_name);

    Ok(())
}
