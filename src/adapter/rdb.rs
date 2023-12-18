use crate::service::interface::{TOTQDeviceRepository, TUnitOfWork};

pub struct SQLRepository;

impl TOTQDeviceRepository for SQLRepository {
    fn add(&self, aggregate: &crate::domain::models::OTQDevice) {
        todo!()
    }
    fn get(&self, id: i64) -> crate::domain::models::OTQDevice {
        todo!()
    }
    fn update(&self, aggregate: &crate::domain::models::OTQDevice) {
        todo!()
    }
}

impl TUnitOfWork for SQLRepository {
    fn begin(&mut self) {
        todo!()
    }
    fn commit(&mut self) {
        todo!()
    }
    fn rollback(&mut self) {
        todo!()
    }
}
