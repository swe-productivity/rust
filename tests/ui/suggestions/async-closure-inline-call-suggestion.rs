//@ edition:2021
use std::future::Future;

async fn f(c: impl Future<Output = ()>) {
    c.await
}

fn main() {
    f(async || {}());
    //~^ ERROR E0618
    //~| HELP if you meant to create this closure and immediately call it, surround the closure with parentheses
    //~| ERROR E0277
    //~| HELP the trait `Future` is not implemented for
    //~| HELP use parentheses to call this closure
}
