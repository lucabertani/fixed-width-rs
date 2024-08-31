use std::{
    backtrace::{Backtrace, BacktraceStatus},
    fmt::Display,
};

#[derive(Debug, Default)]
pub struct FixedWidthError {
    msg: String,
    //#[serde(skip_deserializing, skip_serializing)]
    backtrace: Option<Backtrace>,
    //#[serde(skip_deserializing, skip_serializing)]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl FixedWidthError {
    pub fn new<S: Into<String>>(msg: S) -> FixedWidthError {
        FixedWidthError {
            msg: msg.into(),
            backtrace: Some(Backtrace::capture()), // when RUST_LIB_BACKTRACE or RUST_BACKTRACE is set to 1, stacktrace is captured
            ..Default::default()
        }
    }

    pub fn from<C>(
        context: C,
        error: Option<Box<dyn std::error::Error + Send + Sync>>,
        backtrace: Option<Backtrace>,
    ) -> FixedWidthError
    where
        C: Display + Send + Sync + 'static,
    {
        FixedWidthError {
            msg: context.to_string(),
            backtrace,
            source: error,
        }
    }

    /// Method to allow the creation of a DatabaseError from a generic Error.
    fn from_context<C, E>(context: C, error: E, backtrace: Backtrace) -> FixedWidthError
    where
        C: Display + Send + Sync + 'static,
        E: std::error::Error + Send + Sync + 'static,
    {
        FixedWidthError::from(context, Some(Box::new(error)), Some(backtrace))
    }

    /// Method to allow the creation of a DatabaseError from an Option.
    fn from_display<C>(context: C, backtrace: Backtrace) -> FixedWidthError
    where
        C: Display + Send + Sync + 'static,
    {
        FixedWidthError::from(context, None, Some(backtrace))
    }

    pub fn msg(&self) -> &str {
        self.msg.as_ref()
    }

    pub fn backtrace(&self) -> Option<&Backtrace> {
        self.backtrace.as_ref()
    }

    pub fn source(&self) -> Option<&(dyn std::error::Error + Send + Sync)> {
        self.source.as_deref()
    }
}

impl std::fmt::Display for FixedWidthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error: {}", self.msg)?;
        if let Some(backtrace) = &self.backtrace {
            if backtrace.status() == BacktraceStatus::Captured {
                writeln!(f, "Backtrace: {}", backtrace)?;
            }
        }
        if let Some(err) = &self.source {
            write!(f, "Caused by:\n{}", err)?;
        }
        Ok(())
    }
}

/// Trait to declare the context() and with_context() methods.
pub(crate) trait Context<T, E> {
    /// Wrap the error value with additional context.
    fn context<C>(self, context: C) -> Result<T, FixedWidthError>
    where
        C: Display + Send + Sync + 'static;

    // Wrap the error value with additional context that is evaluated lazily
    // only once an error does occur.

    /*fn with_context<C, F>(self, f: F) -> Result<T, FixedWidthError>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;*/
}

/// Trait that allows converting a generic object C into a FixedWidthError.
pub(crate) trait StdError {
    fn ext_context<C>(self, context: C) -> FixedWidthError
    where
        C: Display + Send + Sync + 'static;
}

/// Implementation to extend the standard Result struct and enable the use of the context() and with_context() methods.
/// These methods essentially convert the error type of the result into a FixedWidthError.
impl<T, E> Context<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T, FixedWidthError>
    where
        C: Display + Send + Sync + 'static,
    {
        // Not using map_err to save 2 useless frames off the captured backtrace
        // in ext_context.
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(error.ext_context(context)),
        }
    }

    /*fn with_context<C, F>(self, context: F) -> Result<T, FixedWidthError>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(error.ext_context(context())),
        }
    }*/
}

/// Implementation to extend the standard Option struct and allow conversion into a Result
/// where Ok contains the value if present, and if None, it is converted into a FixedWidthError.
impl<T> Context<T, std::convert::Infallible> for Option<T> {
    fn context<C>(self, context: C) -> Result<T, FixedWidthError>
    where
        C: Display + Send + Sync + 'static,
    {
        // Not using ok_or_else to save 2 useless frames off the captured
        // backtrace.
        match self {
            Some(ok) => Ok(ok),
            None => Err(FixedWidthError::from_display(context, Backtrace::capture())),
        }
    }

    /*fn with_context<C, F>(self, context: F) -> Result<T, FixedWidthError>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Some(ok) => Ok(ok),
            None => Err(FixedWidthError::from_display(
                context(),
                Backtrace::capture(),
            )),
        }
    }*/
}

/// Implementation to extend a generic struct E, which effectively represents any error that implements the Error trait, and allows converting it into a FixedWidthError.
/// Indeed, this implementation is a generic From for converting any error into a FixedWidthError.
/// It is used when calling the context() or with_context() methods on a Result or Option.
impl<E> StdError for E
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn ext_context<C>(self, context: C) -> FixedWidthError
    where
        C: Display + Send + Sync + 'static,
    {
        let backtrace = Backtrace::capture();
        FixedWidthError::from_context(context, self, backtrace)
    }
}

// From other errors
impl From<time::error::InvalidFormatDescription> for FixedWidthError {
    fn from(error: time::error::InvalidFormatDescription) -> Self {
        FixedWidthError::from(error.to_string(), Some(Box::new(error)), None)
    }
}

impl From<time::error::Format> for FixedWidthError {
    fn from(error: time::error::Format) -> Self {
        FixedWidthError::from(error.to_string(), Some(Box::new(error)), None)
    }
}
