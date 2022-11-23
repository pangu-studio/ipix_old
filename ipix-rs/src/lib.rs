#[macro_use]
extern crate log;
extern crate simplelog;

pub mod constant;
pub mod errors;
pub mod biz;
#[cfg(test)]
mod tests;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

