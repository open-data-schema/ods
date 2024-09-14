use std::{
    io::{Error as IoError, Write},
    result::Result as StdResult,
};

use anstream::{eprintln, stderr, stdout};
use owo_colors::OwoColorize;
use proc_exit::Code;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unable to figure out where to write the output")]
    NoOutputDestination,
    #[error("unable to recognize file format")]
    BadFileFormat,
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("{0}")]
    SerdeYaml(#[from] serde_yml::Error),
    #[error("{0}")]
    Io(#[from] IoError),
}

impl Error {
    fn print(&self) {
        eprintln!("{}: {self}", "error".red().bold());
    }

    fn code(&self) -> Code {
        Code::FAILURE
    }
}

pub type Result<T = ()> = StdResult<T, Error>;

pub fn finish(result: Result) {
    let code = if let Some(e) = result.err() {
        e.print();
        e.code()
    } else {
        Code::SUCCESS
    };

    stdout().flush().unwrap();
    stderr().flush().unwrap();

    code.process_exit();
}
