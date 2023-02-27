#[doc(hidden)]
pub mod hidden {
    pub use human_panic;
}

#[macro_export]
macro_rules! working_custom_panic_hook {
    () => {
        // Current contents of human_panic::metadata!
        let meta = $crate::hidden::human_panic::Metadata {
            version: env!("CARGO_PKG_VERSION").into(),
            name: env!("CARGO_PKG_NAME").into(),
            authors: env!("CARGO_PKG_AUTHORS").replace(":", ", ").into(),
            homepage: env!("CARGO_PKG_HOMEPAGE").into(),
        };
        std::panic::set_hook(Box::new(move |info: &std::panic::PanicInfo| {
            let file_path = $crate::hidden::human_panic::handle_dump(&meta, info);
            eprintln!("my custom message: {file_path:?}")
        }))
    };
}

#[macro_export]
macro_rules! broken_custom_panic_hook {
    () => {
        // Throws an error saying "cannot find struct, variant or union type `Metadata` in this scope"
        let meta = $crate::hidden::human_panic::metadata!();
        std::panic::set_hook(Box::new(move |info: &std::panic::PanicInfo| {
            let file_path = $crate::hidden::human_panic::handle_dump(&meta, info);
            eprintln!("my custom message: {file_path:?}")
        }))
    };
}
