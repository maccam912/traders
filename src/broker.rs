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

    fn get_cash(&self) -> Rational64;
    fn get_positions(&self) -> &HashSet<Position>;
    fn get_orders(&self) -> &[Order];
    fn new_order(&mut self, o: Order);

    fn new_market_order(&mut self, symbol: String, qty: i64) {
        let o = Order {
            symbol,
            qty,
            price: None,
            order_type: OrderType::Market,
            order_status: OrderStatus::Pending,
        };
        self.new_order(o);
    }

    fn new_limit_order(&mut self, symbol: String, qty: i64, price: Rational64) {
        let o = Order {
            symbol,
            qty,
            price: Some(price),
            order_type: OrderType::Limit,
            order_status: OrderStatus::Pending,
        };
        self.new_order(o);
    }
}

pub struct DummyBroker {
    pub cash: Rational64,
    pub positions: HashSet<Position>,
    pub orders: Vec<Order>,
}

impl Broker for DummyBroker {
    fn new() -> DummyBroker {
        DummyBroker {
            cash: Rational64::new_raw(0, 1),
            orders: Vec::new(),
            positions: HashSet::new(),
        }
    }

    fn get_cash(&self) -> Rational64 {
        self.cash
    }

    fn get_positions(&self) -> &HashSet<Position> {
        &self.positions
    }

    fn get_orders(&self) -> &[Order] {
        &self.orders
    }

    fn new_order(&mut self, o: Order) {
        self.orders.push(o);
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
                    basis: order.price.unwrap_or_else(|| Rational64::new_raw(420, 100)),
                };
                let less_cash = p.basis * p.qty;
                self.positions.insert(p);
                self.cash -= less_cash;
            }
        }
    }
}
