#![cfg_attr(windows, feature(abi_vectorcall))]

mod client;
mod options;
mod runtime;
mod session;

use ext_php_rs::prelude::*;

pub use client::CopilotClient;
pub use session::CopilotSession;

#[php_function]
pub fn copilot_sdk_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<CopilotClient>()
        .class::<CopilotSession>()
        .function(wrap_function!(copilot_sdk_version))
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
