use tokio::runtime::Runtime;
use crate::btl_support::find_btl_device::find_bluetooth_devices;

pub fn main_with_args(args: &[String]) -> () {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        find_bluetooth_devices().await;
    });
}

