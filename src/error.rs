use std::{error::Error, fmt::Write};

pub trait ErrorExtension {
    fn display_chain(&self) -> String;
    fn display_chain_with_message(&self, message: &str) -> String;

    fn format_chain(&self, message: Option<&str>) -> String where Self: Error {
        let mut s = match message {
            Some(msg) => format!("Error: {msg}\nCaused by: {self}"),
            None => format!("Error: {self}")
        };

        let mut source = self.source();
        while let Some(error) = source {
            write!(&mut s, "\nCaused by: {error}").expect("Formatting failed!");
            source = error.source();
        }

        s
    }
}

impl<E: Error> ErrorExtension for E {
    fn display_chain(&self) -> String {
        self.format_chain(None)
    }

    fn display_chain_with_message(&self, message: &str) -> String {
        self.format_chain(Some(message))
    }
}
