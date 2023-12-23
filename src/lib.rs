pub mod db;
pub mod otp;
pub mod session;

/// the current application version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// default otp timeout in seconds
pub const OTP_TIMEOUT: u64 = 300;

/// default session timeout in seconds
pub const SESSION_TIMEOUT: u64 = 14_000;
