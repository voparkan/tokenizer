use super::{
    BleController, BlePeripheral,
};

use async_trait::async_trait;
use futures::executor::block_on;
use futures::stream::StreamExt;
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;

use crate::utils;

use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager};

use std::collections::HashMap;
use std::sync::atomic;
use std::sync::{Arc, Mutex};

pub struct BtleplugController {
    adapter: Adapter,
    scan_list: Vec<BlePeripheral>,
    peripheral: Option<Box<btleplug::platform::Peripheral>>,
    notifications_thread_running: Arc<atomic::AtomicBool>,
    notifications_formats: Arc<Mutex<HashMap<String, String>>>,
}

#[async_trait]
impl BleController for BtleplugController {
    async fn scan(&mut self, scan_duration: usize) -> Result<(), Box<dyn Error>> {
        println!("Scanning for {} seconds...", scan_duration);

        let _ = self.adapter.stop_scan().await;
        self.adapter.start_scan(ScanFilter::default()).await?;
        time::sleep(Duration::from_secs(scan_duration as u64)).await;
        let peripherals = self.adapter.peripherals().await?;
        let mut peripherals_vec: Vec<BlePeripheral> = Vec::new();

        for (index, p) in peripherals.into_iter().enumerate() {
            let properties = p.properties().await?.unwrap();
            let name = properties
                .local_name
                .unwrap_or_else(|| String::from("unknown"));

            peripherals_vec.push(BlePeripheral {
                name,
                address_uuid: self.get_address_or_uuid(&p).await?,
                id: index,
            });
        }
        self.scan_list = peripherals_vec;
        Ok(())
    }

    fn get_scan_list(&self) -> Vec<BlePeripheral> {
        self.scan_list.clone()
    }

    async fn get_adapter_infos(&self) -> Result<String, Box<dyn Error>> {
        let adapter_infos: String = self.adapter.adapter_info().await?;
        Ok(adapter_infos)
    }

    async fn connect(&mut self, uuid: &str) -> Result<(), Box<dyn Error>> {
        for p in &self.adapter.peripherals().await? {
            let properties = p.properties().await?.unwrap();
            let name = properties
                .local_name
                .unwrap_or_else(|| String::from("unknown"));

            if uuid == self.get_address_or_uuid(p).await? {
                println!(
                    "Connecting to {} with uuid: {}",
                    name,
                    self.get_address_or_uuid(p).await?
                );
                p.connect().await?;
                self.peripheral = Some(Box::new(p.clone()));
                self.notifications_thread_running
                    .store(true, atomic::Ordering::Relaxed);
                self.start_notifications_thread().await?;

                return Ok(());
            }
        }
        Err(format!("Peripheral with uuid {} not found", uuid))?
    }

    async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(p) = &self.peripheral {
            let properties = p.properties().await?.unwrap();
            let name = properties
                .local_name
                .unwrap_or_else(|| String::from("unknown"));
            println!(
                "Disconnecting from {} with uuid: {} ... ",
                name,
                self.get_address_or_uuid(p).await?
            );
            self.notifications_thread_running
                .store(false, atomic::Ordering::Relaxed);
            p.disconnect().await?;
        } else {
            Err("Error: You must be connected to disconnect")?
        }
        self.peripheral = None;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.peripheral.is_some()
    }
}

impl BtleplugController {
    pub async fn new() -> BtleplugController {
        let manager = match Manager::new().await {
            Ok(m) => m,
            Err(e) => panic!("{:?}", e),
        };

        let adapter_list = match manager.adapters().await {
            Ok(v) => v,
            Err(e) => panic!("{:?}", e),
        };

        let adapter = match adapter_list.len() {
            0 => panic!("Error: No adapter available"),
            1 => &adapter_list[0],
            _ => {
                println!("Found multiple adapters, select the one to use:");
                for (index, ad) in adapter_list.iter().enumerate() {
                    println!("[{}]: {:?}", index, ad);
                }
                let n = utils::get_usize_input(">>");
                &adapter_list[n]
            }
        };

        println!(
            "Using BLE adapter: {:?}",
            adapter.adapter_info().await.unwrap()
        );

        BtleplugController {
            adapter: adapter.clone(),
            scan_list: Vec::new(),
            peripheral: None,
            notifications_thread_running: Arc::new(atomic::AtomicBool::new(false)),
            notifications_formats: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn get_address_or_uuid(
        &self,
        p: &btleplug::platform::Peripheral,
    ) -> Result<String, Box<dyn Error>> {
        let properties = p.properties().await?.unwrap();

        if cfg!(target_os = "macos") {
            Ok(p.id().to_string())
        } else {
            Ok(properties.address.to_string())
        }
    }

    async fn start_notifications_thread(&self) -> Result<(), Box<dyn Error>> {
        println!("Starting notifications thread");

        if let Some(p) = &self.peripheral {
            let mut notification_stream = p.notifications().await?;
            let atomic_is_running = self.notifications_thread_running.clone();
            let all_formats = self.notifications_formats.clone();

            thread::spawn(move || loop {
                if let Some(data) = block_on(notification_stream.next()) {
                    let formats_map = all_formats.lock().unwrap();
                    let fmt = formats_map.get(&data.uuid.to_string()).unwrap();
                    println!(
                        "Notification from [{:?}]: {}",
                        data.uuid,
                        utils::print_bytes::bytes_to_str(&data.value, fmt)
                    );
                }
                if !atomic_is_running.load(atomic::Ordering::Relaxed) {
                    println!("Stopping notifications thread");
                    return;
                }
                thread::sleep(Duration::from_millis(1));
            });
        }
        Ok(())
    }
}
