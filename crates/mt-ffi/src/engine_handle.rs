//! Internal wrapper around mt-engine's Engine.
//!
//! Not exposed directly via C headers; C only sees `mt-engine_handle` opaque.

use std::sync::Mutex;

use crate::types::MtEngineHandle;
use mt_engine::{Engine, EngineConfig};

/// Concrete Rust-side handle layout.
///
/// We rely on a Mutex for thread-safe use from hosts.
/// FFI never exposes this type directly; only as `*mut mt-engine_handle`.
pub struct EngineHandle {
    pub engine: Mutex<Engine>,
}

impl EngineHandle {
    pub fn new_default() -> Result<Self, mt_engine::EngineError> {
        let cfg = EngineConfig::default();
        let engine = Engine::new(cfg)?;
        Ok(Self {
            engine: Mutex::new(engine),
        })
    }

    pub fn new_with_config(cfg: EngineConfig) -> Result<Self, mt_engine::EngineError> {
        let engine = Engine::new(cfg)?;
        Ok(Self {
            engine: Mutex::new(engine),
        })
    }
}

/// Cast `*mut mt-engine_handle` back to `&EngineHandle`.
///
/// # Safety
/// Pointer must originate from `Box<EngineHandle>` created in this crate.
pub unsafe fn from_raw_handle<'a>(ptr: *mut MtEngineHandle) -> Option<&'a EngineHandle> {
    if ptr.is_null() {
        None
    } else {
        // mt-engine_handle is a ZST marker; EngineHandle is stored behind it.
        // Layout: we always create Box<EngineHandle> and cast to *mut mt-engine_handle.
        let handle = unsafe { &*ptr.cast::<EngineHandle>() };

        Some(handle)
    }
}

/// Cast `*mut mt-engine_handle` back to `Box<EngineHandle>` for drop.
///
/// # Safety
/// Same as `from_raw_handle`, and must be called at most once per handle.
pub unsafe fn into_box(ptr: *mut MtEngineHandle) -> Option<Box<EngineHandle>> {
    if ptr.is_null() {
        None
    } else {
        let handle = unsafe { Box::from_raw(ptr.cast::<EngineHandle>()) };

        Some(handle)
    }
}
