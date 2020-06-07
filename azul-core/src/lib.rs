//! Shared datatypes for azul-* crates

#[macro_use(impl_option, impl_vec)]
extern crate azul_css;
#[cfg(feature = "opengl")]
extern crate gleam;
#[cfg(feature = "css_parser")]
extern crate azul_css_parser;

/// Useful macros for implementing Azul APIs without duplicating code
#[macro_use]
pub mod macros;
/// Type definitions for various types of callbacks, as well as focus and scroll handling
#[macro_use]
pub mod callbacks;
/// Functions to manage adding fonts + images, garbage collection
pub mod app_resources;
/// Layout and display list creation algorithm, z-index reordering of a `CachedDisplayList`
pub mod display_list;
/// `Dom` construction, `NodeData` and `NodeType` management functions
pub mod dom;
/// Algorithms to create git-like diffs between two doms in linear time
pub mod diff;
/// Contains OpenGL helper functions (to compile / link shaders), `VirtualGlDriver` for unit testing
#[cfg(feature = "opengl")]
pub mod gl;
/// Internal, arena-based storage for Dom nodes
pub mod id_tree;
/// CSS cascading module
pub mod style;
/// Main `Layout` and `GetTextLayout` trait definition
pub mod traits;
/// Async (task, thread, timer) helper functions
pub mod task;
/// `UiDescription` = CSSOM, cascading
pub mod ui_description;
/// Contains functions to build the `Dom`
pub mod ui_state;
/// Handles the UI layout and UI layout solver
pub mod ui_solver;
/// Window creation / interaction with the OS' windowing API
pub mod window;
/// Window state handling / synchronization
pub mod window_state;

// Typedef for possible faster implementation of hashing
pub type FastHashMap<T, U> = std::collections::HashMap<T, U>;
pub type FastHashSet<T> = std::collections::HashSet<T>;
