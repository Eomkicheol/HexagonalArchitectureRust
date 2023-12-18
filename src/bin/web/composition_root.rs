use wilog::{
    adapter::rdb::SQLRepository,
    domain::models::{Depart, Order},
    service::handler::Handler,
};

pub enum ProcessableCommand {
    Order(Order),
    Depart(Depart),
}

impl ProcessableCommand {
    fn handle(self) {
        match self {
            Self::Order(order_command) => Handler {
                repo: SQLRepository,
            }
            .create_order(order_command),
            Self::Depart(depart_order) => Handler {
                repo: SQLRepository,
            }
            .depart_order(depart_order),
        }
    }
}
