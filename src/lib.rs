//! # mio-serial - Serial port I/O for mio
//!
//! This crate provides a serial port implementation compatable with mio.
//!
//! **At this time this crate ONLY provides a unix implementation**
//!
//! ## Links
//!   - repo:  <https://github.com/berkowski/mio-serial>
//!   - docs:  <https://docs.rs/mio-serial>
#![cfg(unix)]
#![deny(missing_docs)]

extern crate serialport;
extern crate mio;

#[cfg(unix)]
extern crate nix;

// Enums, Structs, and Traits from the serialport crate
pub use serialport::{// Traits
                     SerialPort,

                     // Structs
                     Error,
                     SerialPortInfo,
                     SerialPortSettings,

                     // Enums
                     DataBits,
                     StopBits,
                     Parity,
                     BaudRate,
                     FlowControl,
                     ErrorKind,
};

// Some enumeration functions from the serialport crate
pub use serialport::available_ports;

#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub use unix::Serial;


/// A type for results generated by interacting with serial ports.
pub type Result<T> = serialport::Result<T>;
