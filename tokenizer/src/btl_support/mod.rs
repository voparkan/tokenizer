pub mod find_btl_device;
pub mod system_support_btl;

use core::fmt::Debug;
use std::collections::HashMap;

pub struct BtlSupport {
    lookup: bool,
    devices: HashMap<Device, bool>,
    adapters: HashMap<Adapter, bool>
}

pub struct Device {
    id: u128,
    active: bool,
    mac_addr: String
}

pub struct Adapter {
    id: u128,
    active: bool,
    mac_addr: String
}