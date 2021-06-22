use std::collections::HashSet;

use float_cmp::approx_eq;

#[derive(Debug, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    TrailingStop,
}

#[derive(Debug)]
pub struct Position {
    pub symbol: String,
    pub qty: i64,
    pub basis: f64,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.symbol.eq(&other.symbol)
            && self.qty == other.qty
            && approx_eq!(f64, self.basis, other.basis, ulps = 2)
    }
}

#[derive(Debug)]
pub struct Order {
    pub symbol: String,
    pub qty: i64,
    pub price: Option<f64>,
    pub order_type: OrderType,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        let price_eq = if self.price.is_some() && other.price.is_some() {
            self.price.unwrap() == other.price.unwrap()
        } else {
            self.price.is_none() && other.price.is_none()
        };

        self.symbol.eq(&other.symbol)
            && self.qty == other.qty
            && price_eq
            && self.order_type == other.order_type
    }
}

pub trait Broker {
    fn new() -> Self;

    fn get_positions(&self) -> &HashSet<Position>;
    fn get_orders(&self) -> &[Order];
}

pub struct DummyBroker {
    pub positions: HashSet<Position>,
    pub orders: Vec<Order>,
}

impl Broker for DummyBroker {
    fn new() -> DummyBroker {
        DummyBroker {
            orders: Vec::new(),
            positions: HashSet::new(),
        }
    }

    fn get_positions(&self) -> &HashSet<Position> {
        &self.positions
    }

    fn get_orders(&self) -> &[Order] {
        &self.orders
    }
}
