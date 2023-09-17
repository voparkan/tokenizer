use btleplug::api::{bleuuid::uuid_from_u16, Central, CharPropFlags, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use futures_util::io::AsyncReadExt;
use std::error::Error;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tokio::time;

pub struct AdapterReal {
    id: u128,
    active: bool,
    mac_addr: String
}

pub async fn find_bluetooth_devices() ->Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();
    let adapter = manager.adapters().await.unwrap().pop().unwrap();

    adapter.start_scan(ScanFilter::default()).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let devices = adapter.peripherals().await?;
    for device in devices {
        let properities = device.properties().await?;
        let is_connected = device.is_connected().await?;
        match properities {
            Some(peripheral) => match &peripheral.local_name {
                Some(local_name) => {
                    println!("Found device: {:#?}", local_name);
                    println!("Device data: {:#?}", peripheral);
                    if let Err(err) = device.connect().await {
                        eprintln!("Error connecting to peripheral, skipping: {}", err);
                        continue;
                    }
                    let is_connected = device.is_connected().await?;
                    println!(
                        "Now connected ({:?}) to peripheral {:?}...",
                        is_connected, &local_name
                    );
                    device.discover_services().await?;
                    println!("Discover peripheral {:?} services...", &local_name);
                    for service in device.services() {
                        println!(
                            "Service UUID {}, primary: {}",
                            service.uuid, service.primary
                        );
                        for characteristic in service.characteristics {
                            println!("  {:?}", characteristic);
                            if characteristic.properties.contains(CharPropFlags::NOTIFY) {
                                println!("Subscribing to characteristic {:?}", characteristic.uuid);
                                device.subscribe(&characteristic).await?;
                                // Print the first 4 notifications received.
                                let mut notification_stream =
                                    device.notifications().await?.take(4);
                                // Process while the BLE connection is not broken or stopped.
                                while let Some(data) = notification_stream.next().await {
                                    println!(
                                        "Received data from {:?} [{:?}]: {:?}",
                                        local_name, data.uuid, data.value
                                    );
                                }
                            }
                        }
                    }
                },
                None => println!("Looking for device...")
            },
            None => todo!("Implement me")
        }
    }

    adapter.stop_scan().await.unwrap();
    Ok(())
}