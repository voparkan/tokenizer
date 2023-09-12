use crate::controllers;
use std::error::Error;
pub async fn by_index(
    bt: &mut dyn controllers::BleController,
    id: usize,
) -> Result<(), Box<dyn Error>> {
    match bt.get_scan_list().iter().find(|e| e.id == id) {
        Some(p) => {
            println!("Connecting with id: {}", id);
            bt.connect(&p.address_uuid).await?;
            println!("Connected!");
        }
        None => Err("Id not found")?,
    }
    Ok(())
}

