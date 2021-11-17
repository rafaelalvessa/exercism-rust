/// Various log levels.
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// Primary function for emitting logs.
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Debug => debug(message),
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}

/// Logs a debugging message.
pub fn debug(message: &str) -> String {
    format!("[DEBUG]: {}", message)
}

/// Logs an info message.
pub fn info(message: &str) -> String {
    format!("[INFO]: {}", message)
}

/// Logs a warning message.
pub fn warn(message: &str) -> String {
    format!("[WARNING]: {}", message)
}

/// Logs an error message.
pub fn error(message: &str) -> String {
    format!("[ERROR]: {}", message)
}
