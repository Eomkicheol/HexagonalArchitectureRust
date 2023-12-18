use crate::domain::models::{Depart, OTQDevice, Order};

use super::interface::{TOTQDeviceRepository, TUnitOfWork};

pub struct Handler<R: TOTQDeviceRepository + TUnitOfWork> {
    pub repo: R,
}

impl<R: TOTQDeviceRepository + TUnitOfWork> Handler<R> {
    pub fn create_order(&mut self, cmd: Order) {
        let aggregate = OTQDevice::new(cmd);
        self.repo.add(&aggregate);
        self.repo.commit();
    }

    pub fn depart_order(&mut self, cmd: Depart) {
        let mut aggregate = self.repo.get(cmd.id);
        aggregate.depart_device(cmd);
        self.repo.update(&aggregate);
        self.repo.commit();
    }
}
