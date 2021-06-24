pub mod broker;
pub mod engine;

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use num::Rational64;
    use tokio::sync::mpsc::channel;

    use crate::{
        broker::{Broker, DummyBroker, Order, OrderStatus, OrderType, Position},
        engine::{Bar, Engine, Event},
    };

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_broker_cash() {
        let mut broker = DummyBroker::new();
        broker.cash = Rational64::new(10000, 1);
        assert_eq!(broker.get_cash(), Rational64::new(10000, 1));
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

    #[tokio::test]
    async fn test_event_loop() {
        let (tx, rx) = channel(999);
        let (mut engine, mut rx) = Engine::new(rx);
        let one = Rational64::new_raw(1, 1);
        let bar = Bar {
            t: Utc::now(),
            o: one,
            h: one,
            l: one,
            c: one,
            v: one,
        };
        let bbar = Event::NewBar(bar);
        tokio::spawn(async move {
            engine.run().await;
        });

        let _ = tx.send(bbar).await;
        let message = rx.recv().await;
        assert_eq!(message, Some(Event::Message("Success!".to_string())));
    }

    #[test]
    fn test_broker_new_market_order() {
        let mut broker = DummyBroker::new();
        broker.new_market_order("TSLA".to_string(), 100);
        let order1 = broker.get_orders().get(0).unwrap();
        assert_eq!(order1.symbol, "TSLA".to_string());
        assert_eq!(order1.qty, 100);
        assert_eq!(order1.price, None);
        assert_eq!(order1.order_type, OrderType::Market);
        assert_eq!(order1.order_status, OrderStatus::Pending);
    }

    #[test]
    fn test_broker_new_limit_order() {
        let mut broker = DummyBroker::new();
        broker.new_limit_order("TSLA".to_string(), 100, Rational64::new(420, 100));
        let order1 = broker.get_orders().get(0).unwrap();
        assert_eq!(order1.symbol, "TSLA".to_string());
        assert_eq!(order1.qty, 100);
        assert_eq!(order1.price, Some(Rational64::new(420, 100)));
        assert_eq!(order1.order_type, OrderType::Limit);
        assert_eq!(order1.order_status, OrderStatus::Pending);
    }

    #[test]
    fn test_broker_pos_decreases() {
        let mut broker = DummyBroker::new();
        broker.cash = Rational64::new(1000, 1);
        broker.new_market_order("TSLA".to_string(), 100);
        broker.update();

        // 1000-420 = 580
        assert_eq!(broker.get_cash(), Rational64::new(580, 1));
    }
}
