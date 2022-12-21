// Mod the appropriate file for the OS we're building for
#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "macos", path = "mac.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
mod os;

// Imports
use std::process::ChildStdin;

// Exports
pub use os::OSUtils;

// OSUtilMethods trait, defining what each OSUtil should do
pub trait OSUtilMethods {
    fn new() -> Self;
    fn shell() -> ChildStdin;
    fn exec(&mut self, command: String) -> std::io::Result<()>;
}
