use futures::done;
use futures::prelude::*;
use futures::task::current;
use futures::future::{err, ok, join_all, select_all};
use tokio_core::reactor::Core;
use chrono::prelude::*;
use chrono::Duration;
use std::error::Error;
use std::error;
use std::fmt;

#[derive(Debug)]
struct WaitForIt {
    message: String,
    until: DateTime<Utc>,
    polls: u64,
}

impl WaitForIt {
    pub fn new(message: String, delay: Duration) -> WaitForIt {
        WaitForIt {
            polls: 0,
            message: message,
            until: Utc::now() + delay,
        }
    }
}

impl Future for WaitForIt {
    type Item = String;
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let now = Utc::now();
        if self.until < now {
            Ok(Async::Ready(
                format!("{} after {} polls!", self.message, self.polls),
            ))
        } else {
            self.polls += 1;

            println!("not ready yet --> {:?}", self);
            current().notify();
            Ok(Async::NotReady)
        }
    }
}

pub fn example1() {
    let mut reactor = Core::new().unwrap();

    let wfi_1 = WaitForIt::new("I'm done:".to_owned(), Duration::seconds(1));
    println!("wfi_1 == {:?}", wfi_1);

    let ret = reactor.run(wfi_1).unwrap();
    println!("ret == {:?}", ret);
}

pub fn example2() {
    let mut reactor = Core::new().unwrap();

    let wfi_1 = WaitForIt::new("I'm done:".to_owned(), Duration::seconds(1));
    println!("wfi_1 == {:?}", wfi_1);
    let wfi_2 = WaitForIt::new("I'm done too:".to_owned(), Duration::seconds(1));
    println!("wfi_2 == {:?}", wfi_2);

    let v = vec![wfi_1, wfi_2];
    let sel = join_all(v);

    let ret = reactor.run(sel).unwrap();
    println!("ret == {:?}", ret);
}

pub fn example3() {
    let mut reactor = Core::new().unwrap();

    let wfi_1 = WaitForIt::new("I'm done:".to_owned(), Duration::seconds(1));
    println!("wfi_1 == {:?}", wfi_1);
    let wfi_2 = WaitForIt::new("I'm done too:".to_owned(), Duration::seconds(1));
    println!("wfi_2 == {:?}", wfi_2);

    let v = vec![wfi_1, wfi_2];
    let sel = select_all(v);

    let ret = reactor.run(sel).unwrap();
    println!("ret == {:?}", ret);
}
