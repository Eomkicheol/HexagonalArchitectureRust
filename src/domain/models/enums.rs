#[derive(PartialEq, Eq, Debug)]
pub enum OTQDeviceStatus {
    Ordered,
    Departed,
    ArrivedPassed,
    ArrivedFailed,
}
