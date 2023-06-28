use btleplug::api::Central;
use btleplug::platform::Manager;
use tokio::runtime::Runtime;

pub async fn find_bluetooth_devices() {
    let manager = Manager::new().unwrap();
    let adapter = manager.adapters().await.unwrap().pop().unwrap();

    adapter.start_scan().await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let devices = adapter.peripherals().await.unwrap();
    for device in devices {
        println!("Found device: {}", device.properties().await.unwrap().local_name.unwrap_or_else(|| "(unknown)".to_owned()));
    }

    adapter.stop_scan().await.unwrap();
}