use crate::structs::{dump_data::DumpData, prod::Prod};
use anyhow::Result;
use rust_xlsxwriter::{worksheet::Worksheet, Workbook};
use std::{error::Error, fs::File, io::BufReader};

struct Column<'a> {
    header: &'a str,
    write: Box<dyn Fn(&mut Worksheet, u32, u16, &Prod) -> Result<()> + 'a>,
}

impl<'a> Column<'a> {
    fn new<H, F>(header: H, write: F) -> Self
    where
        H: Into<&'a str>,
        F: Fn(&mut Worksheet, u32, u16, &Prod) -> Result<()> + 'a,
    {
        Self {
            header: header.into(),
            write: Box::new(write),
        }
    }
}

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

    let columns: Vec<Column> = vec![
        Column::new("Name", |ws, row, col, p| {
            if let Some(v) = &p.name {
                ws.write_string(row, col, v)?;
            }
            Ok(())
        }),
        Column::new("Groups", |ws, row, col, p| {
            if let Some(v) = &p.groups {
                let joined = v
                    .iter()
                    .filter_map(|t| t.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                if !joined.is_empty() {
                    ws.write_string(row, col, &joined)?;
                }
            }
            Ok(())
        }),
        Column::new("Type", |ws, row, col, p| {
            if let Some(v) = &p.prod_type {
                let joined = v
                    .iter()
                    .map(|t| format!("{:?}", t))
                    .collect::<Vec<_>>()
                    .join(", ");
                if !joined.is_empty() {
                    ws.write_string(row, col, &joined)?;
                }
            }
            Ok(())
        }),
        Column::new("Platform", |ws, row, col, p| {
            if let Some(v) = &p.platforms {
                let joined = v
                    .values()
                    .filter_map(|pl| pl.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                if !joined.is_empty() {
                    ws.write_string(row, col, &joined)?;
                }
            }
            Ok(())
        }),
        Column::new("Party", |ws, row, col, p| {
            if let Some(party) = &p.party {
                if let Some(name) = party.name.as_deref() {
                    ws.write_string(row, col, name)?;
                }
            }
            Ok(())
        }),
        Column::new("Place", |ws, row, col, p| {
            if let Some(v) = &p.party_place {
                ws.write_number(row, col, *v)?;
            }
            Ok(())
        }),
        Column::new("Year", |ws, row, col, p| {
            if let Some(v) = &p.party_year {
                ws.write_number(row, col, *v)?;
            }
            Ok(())
        }),
        Column::new("Added", |ws, row, col, p| {
            if let Some(v) = &p.added_date {
                ws.write_string(row, col, v)?;
            }
            Ok(())
        }),
        Column::new("Release", |ws, row, col, p| {
            if let Some(v) = &p.release_date {
                ws.write_string(row, col, v)?;
            }
            Ok(())
        }),
        Column::new("Down", |ws, row, col, p| {
            if let Some(v) = p.votedown {
                ws.write_number(row, col, v)?;
            }
            Ok(())
        }),
        Column::new("Pig", |ws, row, col, p| {
            if let Some(v) = p.votepig {
                ws.write_number(row, col, v)?;
            }
            Ok(())
        }),
        Column::new("Up", |ws, row, col, p| {
            if let Some(v) = p.voteup {
                ws.write_number(row, col, v)?;
            }
            Ok(())
        }),
        Column::new("Avg", |ws, row, col, p| {
            if let Some(v) = p.voteavg {
                ws.write_number(row, col, v)?;
            }
            Ok(())
        }),
        Column::new("Download", |ws, row, col, p| {
            if let Some(v) = &p.download {
                ws.write_string(row, col, v)?;
            }
            Ok(())
        }),
        Column::new("Link", |ws, row, col, p| {
            if let Some(v) = p.id {
                ws.write_string(
                    row,
                    col,
                    format!("https://www.pouet.net/prod.php?which={}", v),
                )?;
            }
            Ok(())
        }),
    ];

    for (col_idx, col) in columns.iter().enumerate() {
        worksheet.write_string(0, col_idx as u16, col.header)?;
    }

    if let Some(prods) = &dump.prods {
        for (r, prod) in prods.iter().enumerate() {
            let row = (r as u32) + 1;
            for (c, col) in columns.iter().enumerate() {
                (col.write)(worksheet, row, c as u16, prod)?;
            }
        }
    }

    let excel_file_name = format!("{}.xlsx", json_filename);
    workbook.save(&excel_file_name)?;
    println!("✅ Write excel {}", excel_file_name);

    Ok(())
}
