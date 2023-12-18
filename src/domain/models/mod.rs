pub mod commands;
pub mod enums;

pub use commands::*;
pub use enums::*;

pub struct OTQDevice {
    id: i64,
    status: OTQDeviceStatus,
    device_serial: i64,
}

impl OTQDevice {
    pub fn new(cmd: Order) -> Self {
        todo!()
    }
    pub fn depart_device(&mut self, cmd: Depart) {
        todo!()
    }
}

#[test]
fn create_otq_device() {
    '_given: {
        '_when: {
            let cmd = Order;
            let device = OTQDevice::new(cmd);

            '_then: {
                assert_eq!(device.status, OTQDeviceStatus::Ordered);
            }
        }
    }
}
