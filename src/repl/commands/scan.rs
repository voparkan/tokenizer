use crate::controllers;


use controllers::BlePeripheral;

use comfy_table::Table;

use std::error::Error;

pub fn print_scan_list(list: &Vec<BlePeripheral>, show_all: bool) -> Result<(), Box<dyn Error>> {
    let mut table = Table::new();

    table.add_row(vec!["ID", "Name", "uuid"]);

    let mut empty_list: bool = true;

    for p in list {
        if !show_all && p.name == "unknown" {
            continue;
        }
        table.add_row(vec![
            &p.id.to_string(),
            &p.name,
            &p.address_uuid,
        ]);
        empty_list = false
    }

    if empty_list {
        Err("Error: Empty list.")?;
    }

    println!("{table}");

    Ok(())
}

pub async fn run(
    bt: &mut dyn controllers::BleController,
    timeout: usize,
    print_results: bool,
    show_all: bool,
) -> Result<(), Box<dyn Error>> {
    bt.scan(timeout).await?;

    if print_results {
        print_scan_list(&bt.get_scan_list(), show_all)
    } else {
        Ok(())
    }
}
