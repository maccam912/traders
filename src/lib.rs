pub mod broker;

#[cfg(test)]
mod tests {
    use crate::broker::{Broker, DummyBroker, Position};

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
}
