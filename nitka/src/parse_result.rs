use std::mem::{size_of, MaybeUninit};

use anyhow::Result;
use near_workspaces::result::{ExecutionFailure, ExecutionSuccess};
use serde::de::DeserializeOwned;

pub trait ParseResult<T: DeserializeOwned> {
    fn parse(self) -> Result<T>;
}

impl<T: DeserializeOwned> ParseResult<T> for Result<ExecutionSuccess, ExecutionFailure> {
    fn parse(self) -> Result<T> {
        let result = self?;

        let result = if size_of::<T>() == 0 {
            // For cases when return type is `()` and we don't need to parse result.
            // This call is safe for zero sized types.
            #[allow(clippy::uninit_assumed_init)]
            unsafe {
                MaybeUninit::uninit().assume_init()
            }
        } else {
            result.json()?
        };

        Ok(result)
    }
}
