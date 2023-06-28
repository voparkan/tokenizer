use tokio::runtime::Runtime;

use crate::find_btl_device;

pub fn main_with_args(args: &[String]) -> i32 {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        find_bluetooth_devices().await;
    });
}

