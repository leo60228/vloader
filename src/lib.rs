#![recursion_limit = "8192"]
#![feature(unboxed_closures, stmt_expr_attributes)]

#[macro_use]
extern crate rental;

#[macro_use]
pub mod helpers;
mod defs;
mod gamestates;
pub mod lua;
mod patches;
mod state;

#[link_section = ".init_array.00099"]
#[used]
static LOG_INIT: extern "C" fn() = {
    extern "C" fn wrapper() {
        cute_log::init().unwrap();
    }

    wrapper
};
