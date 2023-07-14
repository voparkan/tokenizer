use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;

pub struct AdapterReal {
    id: u128,
    active: bool,
    mac_addr: String
}

pub async fn find_bluetooth_devices() {
    let manager = Manager::new().await.unwrap();
    let adapter = manager.adapters().await.unwrap().pop().unwrap();

    adapter.start_scan(ScanFilter::default()).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let devices = adapter.peripherals().await.unwrap();
    for device in devices {
        println!("Found device: {:#?}", device.properties().await.unwrap());
    }

    adapter.stop_scan().await.unwrap();
}