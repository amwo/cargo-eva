#![feature(proc_macro_hygiene, decl_macro, async_await, await_macro, futures_api)]

mod services;

#[macro_use]
extern crate rocket;

extern crate futures;
extern crate reql;
extern crate reql_types;

use reql::{Client, Config, Document, Run};

fn main() {
    services::rest::launcher();
}
