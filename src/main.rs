#![feature(proc_macro, generators)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(extern_prelude)]

extern crate futures_await as futures;
extern crate tokio_core;
extern crate chrono;

mod part1;
mod part2;
mod part3;
mod part4;
use part1::*;
use part2::*;
use part3::*;
use part4::*;

fn main() {
    part1::example1();
    // part1::example2();
    // part1::example3();
    // part1::example4();
    // part1::example5();
    // part1::example6();

    // part2::example1();
    // part2::example2();
    // part2::example3();
    // part2::example4();
    // part2::example5();
    // part2::example6();

    // part3::example1();
    // part3::example2();
    // part3::example3();

    // part4::example1();
    // part4::example2();
    // part4::example3();
}
