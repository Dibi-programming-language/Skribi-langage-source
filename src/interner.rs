use std::sync::{LazyLock, Mutex};

use string_interner::DefaultStringInterner;

/// WARNING: when possible, passing as an argument in prefered
/// Access the lock once if possible
pub static INTERNER: Mutex<LazyLock<DefaultStringInterner>> =
    Mutex::new(LazyLock::new(|| DefaultStringInterner::default()));
