use std::{error::Error, fmt::{self, Write}};

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

#[derive(Debug)]
pub struct BoxedError(Box<dyn Error + 'static + Send + Sync>);

impl fmt::Display for BoxedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for BoxedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

impl From<std::io::Error> for BoxedError {
    fn from(error: std::io::Error) -> Self {
        BoxedError(Box::new(error))
    }
}

impl BoxedError {
    pub fn new(error: impl Error + 'static + Send + Sync) -> Self {
        BoxedError(Box::new(error))
    }
}

#[cfg_attr(feature="mock_windows", cfg(unix))]
#[cfg(windows)]
#[macro_export]
macro_rules! win32_err {
    ($expr:expr) => {{
        let status = $expr;
        if status = ::windows_sys::Win32::Foundation::NO_ERROR {
            Ok(())
        } else {
            Err(::std::io::Error::from_raw_os_error(status as i32))
        }
    }};
    ($expr:expr, $msg:expr) => {{
        let status = $expr;
        if status == ::windows_sys::Win32::Foundation::NO_ERROR {
            Ok(())
        } else {
            Err(::std::io::Error::new(::std::io::ErrorKind::Other, $msg))
        }
    }};
}

#[cfg(not(windows))]
#[macro_export]
macro_rules! win32_err {
    ($expr:expr) => {{
        compile_error!("win32_err! is only supported on Windows.");
    }};
}
