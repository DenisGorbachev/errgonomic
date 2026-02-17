#![allow(dead_code)]

use thiserror::Error;

pub fn foo_bundled(input: String) -> Result<u32, FooError> {
    use FooError::*;
    let a = get_a(input.clone());
    let b = get_b(input);
    match (a, b) {
        (Ok(a), Ok(b)) => Ok(a + b),
        (a, b) => Err(GetAOrBFailed {
            a,
            b,
        }),
    }
}

pub fn get_a(_input: String) -> Result<u32, GetAError> {
    todo!()
}

pub fn get_b(_input: String) -> Result<u32, GetBError> {
    todo!()
}

#[derive(Error, Debug)]
pub enum FooError {
    #[error("get a or b failed")]
    GetAOrBFailed { a: Result<u32, GetAError>, b: Result<u32, GetBError> },
}

#[derive(Error, Debug)]
pub enum GetAError {}

#[derive(Error, Debug)]
pub enum GetBError {}
