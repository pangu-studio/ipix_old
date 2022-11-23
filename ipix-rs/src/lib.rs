#[macro_use]
extern crate log;
extern crate simplelog;
extern crate serde_derive;

pub mod constant;
pub mod errors;
pub mod biz;
#[cfg(test)]
mod tests;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

