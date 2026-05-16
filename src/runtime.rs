use std::sync::OnceLock;

use tokio::runtime::{Builder, Runtime};

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();

    RUNTIME.get_or_init(|| {
        Builder::new_multi_thread()
            .enable_all()
            .thread_name("ext-php-copilot")
            .build()
            .expect("failed to create ext-php-copilot Tokio runtime")
    })
}
