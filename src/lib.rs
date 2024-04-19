//! # Input Middleware
//! This library is a collection of input devices and utilities for handling input events.
//!
//! ## Features
//! - KMBoxNet: A networked input device that can send keyboard and mouse events to a KMBoxNet device.
//!
//! ## Usage
//! ```rust
//! let km = KMBoxNet::new("192.168.2.188".into(), 16824, "XXXXXXXX".into());
//! match km {
//!   Ok(mut km) => {
//!     km.mouse_move([1, 1]);
//!     // ...
//!   }
//!   Err(_) => println!("Failed to connect to KMBox Net"),
//! }
//! ```
pub mod devices;
