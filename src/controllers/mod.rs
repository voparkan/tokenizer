use async_trait::async_trait;
use std::error::Error;

pub mod btleplug;

#[derive(Debug, Clone)]
pub struct BlePeripheral {
    pub id: usize, // Internal id used by ble implementations
    pub name: String, // peripheral name
    pub address_uuid: String, // Peripheral mac address (or Apple generated UUID on OSX)
}


#[async_trait]
pub trait BleController {
    async fn scan(&mut self, scan_time_s: usize) -> Result<(), Box<dyn Error>>;

    fn get_scan_list(&self) -> Vec<BlePeripheral>;

    async fn get_adapter_infos(&self) -> Result<String, Box<dyn Error>>;

    async fn connect(&mut self, uuid: &str) -> Result<(), Box<dyn Error>>;

    async fn disconnect(&mut self) -> Result<(), Box<dyn Error>>;

    fn is_connected(&self) -> bool;
}
