use futures::done;
use futures::prelude::*;
use futures::future::{err, ok};
use tokio_core::reactor::Core;
use std::error::Error;

fn my_fn() -> Result<u32, Box<Error>> {
    Ok(100)
}

fn my_fut() -> impl Future<Item = u32, Error = Box<Error + 'static>> {
    ok(200)
}

fn my_fn_squared(i: u32) -> Result<u32, Box<Error>> {
    Ok(i * i)
}

fn my_fut_squared(i: u32) -> impl Future<Item = u32, Error = Box<Error + 'static>> {
    ok(i * i)
}

fn fn_plain(i: u32) -> u32 {
    i - 50
}

fn fut_generic_own<A>(a1: A, a2: A) -> impl Future<Item = A, Error = Box<Error + 'static>>
where
    A: std::cmp::PartialOrd,
{
    if a1 < a2 {
        ok(a1)
    } else {
        ok(a2)
    }
}

// Running functions vs futures
pub fn example1() {
    let retval = my_fn().unwrap();
    println!("{:?}", retval);

    let mut reactor = Core::new().unwrap();

    let retval = reactor.run(my_fut()).unwrap();
    println!("{:?}", retval);
}

// Running normal functions
pub fn example2() {
    let retval = my_fn().unwrap();
    println!("{:?}", retval);

    let retval2 = my_fn_squared(retval).unwrap();
    println!("{:?}", retval2);
}

// Running multiple futures with reactor core
pub fn example3() {
    let mut reactor = Core::new().unwrap();

    let retval = reactor.run(my_fut()).unwrap();
    println!("{:?}", retval);

    let retval2 = reactor.run(my_fut_squared(retval)).unwrap();
    println!("{:?}", retval2);
}

// Chaining futures
pub fn example4() {
    let mut reactor = Core::new().unwrap();
    let chained_future = my_fut().and_then(|retval| my_fn_squared(retval));
    let retval2 = reactor.run(chained_future).unwrap();
    println!("{:?}", retval2);
}

// Combining plain functions and futures
pub fn example5() {
    let mut reactor = Core::new().unwrap();
    let chained_future = my_fut().and_then(|retval| {
        let retval2 = fn_plain(retval);
        my_fut_squared(retval2)
    });
    let retval3 = reactor.run(chained_future).unwrap();
    println!("{:?}", retval3);
}

// Combining plain functions which return result and futures
pub fn example6() {
    let mut reactor = Core::new().unwrap();
    let chained_future = my_fut().and_then(|retval| {
        done(my_fn_squared(retval)).and_then(|retval2| my_fut_squared(retval2))
    });
    let retval3 = reactor.run(chained_future).unwrap();
    println!("{:?}", retval3);
}
