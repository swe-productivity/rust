#![allow(dead_code)]

struct Foo {
    a: String,
    b: usize,
}

impl Foo {
    fn new(a: String) -> Self {
        Self {
            a,
            //~^ HELP consider cloning the value if the performance cost is acceptable
            //~| HELP consider initializing `b` before `a`
            b: a.len(),
            //~^ ERROR borrow of moved value
        }
    }

    fn dont_suggest_within_same_field_init(a: String) -> Self {
        Self {
            a: String::new(),
            b: {
                let b = a;
                //~^ HELP consider cloning the value if the performance cost is acceptable
                a.len()
                //~^ ERROR borrow of moved value
            },
        }
    }
}

fn main() {}
