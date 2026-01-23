//@ edition:2021
use std::future::Future;

async fn f(c: impl Future<Output = ()>) {
    c.await
}

fn main() {
    f(async || {});
    //~^ ERROR: not a future
    //~| HELP: the trait `Future` is not implemented for
    //~| HELP: use parentheses to call this closure

    f(async || {}());
    //~^ ERROR: expected function, found `()`
    //~| HELP: if you meant to create this closure and immediately call it, surround the closure with parentheses
    //~| ERROR: not a future
    //~| HELP: the trait `Future` is not implemented for
}
