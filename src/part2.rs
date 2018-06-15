use futures::done;
use futures::prelude::*;
use futures::future::{err, ok};
use tokio_core::reactor::Core;
use std::error::Error;
use std::error;
use std::fmt;

#[derive(Debug, Default)]
pub struct ErrorA {}

impl fmt::Display for ErrorA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorA!")
    }
}

impl error::Error for ErrorA {
    fn description(&self) -> &str {
        "Description for ErrorA"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug, Default)]
pub struct ErrorB {}

impl fmt::Display for ErrorB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorB!")
    }
}

impl error::Error for ErrorB {
    fn description(&self) -> &str {
        "Description for ErrorB"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<ErrorB> for ErrorA {
    fn from(_e: ErrorB) -> ErrorA {
        ErrorA::default()
    }
}

impl From<ErrorA> for ErrorB {
    fn from(_e: ErrorA) -> ErrorB {
        ErrorB::default()
    }
}

fn fut_error_a() -> impl Future<Item = (), Error = ErrorA> {
    err(ErrorA {})
}

fn fut_error_b() -> impl Future<Item = (), Error = ErrorB> {
    err(ErrorB {})
}

fn my_fn_ref(s: &str) -> Result<&str, Box<Error>> {
    Ok(s)
}

fn my_fut_ref(s: &str) -> impl Future<Item = &str, Error = Box<Error>> {
    ok(s)
}

fn my_fut_ref_chained<'a>(s: &'a str) -> impl Future<Item = String, Error = Box<Error>> {
    my_fut_ref(s).and_then(|s| ok(format!("received == {}", s)))
}

pub fn example1() {
    let mut reactor = Core::new().unwrap();
    let retval = reactor.run(fut_error_a()).unwrap_err();
    println!("fut_error_a == {:?}", retval);

    let retval = reactor.run(fut_error_b()).unwrap_err();
    println!("fut_error_b == {:?}", retval);
}

pub fn example2() {
    // let mut reactor = Core::new().unwrap();
    // let future = fut_error_a().and_then(|_| fut_error_b());
    // let retval = reactor.run(future()).unwrap_err();
}

pub fn example3() {
    let mut reactor = Core::new().unwrap();
    let future = fut_error_a()
        .map_err(|e| {
            println!("mapping {:?} into ErrorB", e);
            ErrorB::default()
        })
        .and_then(|_| fut_error_b());

    let retval = reactor.run(future).unwrap_err();
    println!("error chain == {:?}", retval);
}

// Chain one error after another. Bad example.
pub fn example4() {
    let mut reactor = Core::new().unwrap();
    let future = fut_error_a()
        .map_err(|_| ErrorB::default())
        .and_then(|_| fut_error_b())
        .map_err(|_| ErrorA::default());
    let retval = reactor.run(future).unwrap_err();
    println!("error chain == {:?}", retval);
}

pub fn example5() {
    let mut reactor = Core::new().unwrap();
    let future = fut_error_a()
        .from_err()
        .and_then(|_| fut_error_b())
        .from_err()
        .and_then(|_| fut_error_a());
    let retval = reactor.run(future).unwrap_err();
    println!("error chain == {:?}", retval);
}

pub fn example6() {
    let mut reactor = Core::new().unwrap();
    let retval = reactor
        .run(my_fut_ref_chained("str with lifetime"))
        .unwrap();
    println!("my_fut_ref_chained == {}", retval);
}
