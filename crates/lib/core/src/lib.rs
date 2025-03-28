//! The core crate provides the foundational types and systems for the Seedbed game engine.
//!
//! This includes:
//! - The main [`Engine`] type that drives the game loop
//! - Error handling via [`CoreError`]
//! - Common result type [`Result`]
//!
//! The engine is designed to be modular and extensible, with a focus on turn-based 2D games.

#![no_std]

extern crate alloc;

use core::result;

/// A result type for the core crate. See [`CoreError`] for more information.
type Result<T> = result::Result<T, CoreError>;

/// The public error type for the core crate.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct CoreError(#[from] anyhow::Error);

#[derive(Default)]
pub struct Engine {
    /// Whether the engine is running or not.
    is_running: bool,
}

impl Engine {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        return Self { is_running: false };
    }

    #[inline]
    #[must_use]
    pub fn is_running(&self) -> bool {
        return self.is_running;
    }

    #[inline]
    pub fn tick(&mut self) {
        self.is_running = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gameplay_loop() {
        let mut engine = Engine::new();

        engine.tick();

        assert!(engine.is_running());
    }
}
