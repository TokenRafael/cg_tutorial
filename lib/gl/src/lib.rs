mod bindings {
    include!(concat!(env!("OUT_DIR"), "\\bindings.rs"));
}

pub use bindings::Gl as InnerGl;
pub use bindings::*;

use std::ops::Deref;
use std::rc::Rc;

/// A wrapper around C OpenGL functions.
/// This is a singleton accessed by reference.
/// Internally, it uses a reference count.
/// This is to protect the OpenGL context and to allow multiplace OpenGL calls.
#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gl>,
}

impl Gl {
    /// Creates a new OpenGL context given a function to start it.
    pub fn load_with<F>(load_fn: F) -> Gl
    where
        F: FnMut(&'static str) -> *const types::GLvoid,
    {
        Gl {
            inner: Rc::new(bindings::Gl::load_with(load_fn)),
        }
    }
}

impl Deref for Gl {
    type Target = bindings::Gl;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
