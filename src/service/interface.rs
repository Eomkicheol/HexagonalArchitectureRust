use crate::domain::models::OTQDevice;

pub trait TOTQDeviceRepository {
    fn add(&self, aggregate: &OTQDevice);
    fn update(&self, aggregate: &OTQDevice);
    fn get(&self, id: i64) -> OTQDevice;
}

pub trait TUnitOfWork {
    fn begin(&mut self);
    fn commit(&mut self);
    fn rollback(&mut self);
}
