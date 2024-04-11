#![allow(clippy::too_many_arguments)]

mod circuit;
mod constructor;
mod driver;

pub mod handlers {
    use crate::*;
    use rocket::Route;

    pub fn handlers() -> Vec<Route> {
        circuit::handlers()
            .into_iter()
            .chain(driver::handlers())
            .chain(constructor::handlers())
            .collect()
    }
}
