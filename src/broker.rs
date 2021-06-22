use std::collections::HashSet;

use num::Rational64;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    TrailingStop,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OrderStatus {
    Pending,
    Cancelled,
    Filled,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub symbol: String,
    pub qty: i64,
    pub basis: Rational64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Order {
    pub symbol: String,
    pub qty: i64,
    pub price: Option<Rational64>,
    pub order_type: OrderType,
    pub order_status: OrderStatus,
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

impl DummyBroker {
    pub fn update(&mut self) {
        let borrow = &mut self.orders;
        for order in borrow {
            if order.order_status == OrderStatus::Pending {
                order.order_status = OrderStatus::Filled;
                let p = Position {
                    symbol: order.symbol.clone(),
                    qty: order.qty,
                    basis: order.price.unwrap_or(Rational64::new_raw(420, 100)),
                };
                self.positions.insert(p);
            }
        }
    }
}
