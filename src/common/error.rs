// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::common::span::{pretty_print_error, Span};
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseError(pub Span, pub String);

impl ParseError {
    /// Pretty display the error message onto source if span is available.
    pub fn display_with_source(mut self, source: &str) -> Self {
        if let Some(span) = self.0.take() {
            self.1 = pretty_print_error(source, vec![(span, self.1.to_string())]);
        }
        self
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

#[derive(Debug)]
pub enum CustomError {
    Normal(String),
    ParseError(ParseError),
    ParseIntError(ParseIntError),
    FormatxError(formatx::Error),
    IoError(std::io::Error),
}

impl Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CustomError::Normal(e) => e.fmt(f),
            CustomError::ParseError(e) => e.fmt(f),
            CustomError::ParseIntError(e) => e.fmt(f),
            CustomError::FormatxError(e) => e.fmt(f),
            CustomError::IoError(e) => e.fmt(f),
        }
    }
}

impl From<String> for CustomError {
    fn from(e: String) -> Self {
        CustomError::Normal(e)
    }
}

impl From<ParseError> for CustomError {
    fn from(e: ParseError) -> Self {
        CustomError::ParseError(e)
    }
}

impl From<ParseIntError> for CustomError {
    fn from(e: ParseIntError) -> CustomError {
        CustomError::ParseIntError(e)
    }
}

impl From<formatx::Error> for CustomError {
    fn from(e: formatx::Error) -> CustomError {
        CustomError::FormatxError(e)
    }
}

impl From<std::io::Error> for CustomError {
    fn from(e: std::io::Error) -> CustomError {
        CustomError::IoError(e)
    }
}
