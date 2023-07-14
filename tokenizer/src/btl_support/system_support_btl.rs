use tokio::runtime::Runtime;
use crate::btl_support::find_btl_device::find_bluetooth_devices;

pub fn main_with_args(args: &[String]) -> i32 {
    let rt = Runtime::new().unwrap();
    let exit: i32 = 1;
    rt.block_on(async {
        find_bluetooth_devices().await;
    });
    println!("rt {:#?}", rt);
    exit
}

