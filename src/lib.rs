pub mod broker;

#[cfg(test)]
mod tests {
    use crate::broker::{Broker, DummyBroker, Order, Position};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_broker_orders() {
        let broker = DummyBroker::new();
        assert_eq!(broker.get_orders(), vec![]);
    }

    #[test]
    fn test_broker_positions() {
        let broker = DummyBroker::new();
        let positions: Vec<&Position> = broker.get_positions().into_iter().collect();
        assert_eq!(positions.len(), 0);
    }

    #[test]
    fn test_broker_order_to_pos() {
        let mut broker = DummyBroker::new();
        broker.orders = vec![Order {
            symbol: "TSLA".to_string(),
            qty: 100,
            price: None,
            order_type: crate::broker::OrderType::Market,
            order_status: crate::broker::OrderStatus::Pending,
        }];

        broker.update();
        assert_eq!(broker.get_positions().len(), 1);
    }
}
