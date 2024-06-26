use std::ffi::{CStr, CString};

/// Logs to the js console
#[cfg(feature = "logger")]
pub struct JsConsoleLogger;

#[cfg(feature = "logger")]
impl JsConsoleLogger {
    /// Initialize the logger
    pub fn init() {
        ::log::set_logger(&JsConsoleLogger)
            .map(|()| ::log::set_max_level(::log::LevelFilter::Info))
            .unwrap()
    }
}

#[cfg(feature = "logger")]
impl ::log::Log for JsConsoleLogger {
    fn enabled(&self, metadata: &::log::Metadata) -> bool {
        metadata.level() <= ::log::Level::Info
    }

    fn log(&self, record: &::log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        match record.metadata().level() {
            ::log::Level::Info => log::info(record.args().to_string()),
            ::log::Level::Warn => log::warn(record.args().to_string()),
            ::log::Level::Error => log::error(record.args().to_string()),
            _ => {}
        }
    }

    fn flush(&self) {}
}

pub mod log {
    use std::ffi::CString;

    /// Log a string to the js console as info
    pub fn info(string: impl Into<String>) {
        let string: String = string.into();
        let cstring = CString::new(string.clone()).unwrap();
        unsafe { crate::ffi::hapi_js_console_log_info(cstring.as_ptr() as *const u8) }
    }

    /// Log a string to the js console as a warning
    pub fn warn(string: impl Into<String>) {
        let string: String = string.into();
        let cstring = CString::new(string.clone()).unwrap();
        unsafe { crate::ffi::hapi_js_console_log_warn(cstring.as_ptr() as *const u8) }
    }

    /// Log a string to the js console as an error
    pub fn error(string: impl Into<String>) {
        let string: String = string.into();
        let cstring = CString::new(string.clone()).unwrap();
        unsafe { crate::ffi::hapi_js_console_log_error(cstring.as_ptr() as *const u8) }
    }
}

/// Evaluate some javascript code
/// Returns the result as a string
/// Returns None if the code could not be evaluated
pub fn eval(source: &str) -> Option<serde_json::Value> {
    let cstring = CString::new(source).unwrap();
    let ptr = unsafe { crate::ffi::hapi_js_console_eval(cstring.as_ptr() as *const u8) };

    if ptr == std::ptr::null() {
        return None;
    }

    // # Safety
    // Since we know for certain the string is null terminated, there is no way to access unallocated memory
    let cstring = unsafe { CStr::from_ptr(ptr as *mut i8) };
    let string = cstring.to_string_lossy().to_string();

    // Free the string from memory
    // # Safety
    // Since we know the string was allocated by the hapi_js_console_eval function, we know it is safe to free
    unsafe { crate::mem::free(ptr as *mut u8) };

    serde_json::from_str(&string).ok()
}
