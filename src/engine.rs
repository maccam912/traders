use tokio::sync::mpsc::{channel, Receiver, Sender};

use chrono::{DateTime, Utc};
use num::Rational64;

#[derive(Debug, PartialEq, Eq)]
pub struct Bar {
    pub t: DateTime<Utc>,
    pub o: Rational64,
    pub h: Rational64,
    pub l: Rational64,
    pub c: Rational64,
    pub v: Rational64,
}

unsafe impl Send for Bar {}
unsafe impl Sync for Bar {}

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    NewBar(Bar),
    Message(String),
}

pub struct Engine {
    irx: Receiver<Event>,
    itx: Sender<Event>,
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}

impl Engine {
    pub fn new(irx: Receiver<Event>) -> (Engine, Receiver<Event>) {
        let (itx, rx) = channel(999);
        (Engine { irx, itx }, rx)
    }
    pub async fn run(&mut self) {
        loop {
            match self.irx.recv().await {
                Some(e) => match e {
                    Event::NewBar(bar) => {
                        println!("{:?}", bar);
                        let message = "Success!".to_string();
                        let _ = self.itx.send(Event::Message(message)).await;
                    }
                    _ => println!("Error! Don't send any messages!"),
                },
                None => todo!(),
            }
        }
    }
}
