//! A simple Driver for the Waveshare E-Ink Displays via SPI
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1
//!
//! # Requirements
//!
//! ### SPI
//!
//! - MISO is not connected/available
//! - SPI_MODE_0 is used (CPHL = 0, CPOL = 0)
//! - 8 bits per word, MSB first
//! - Max. Speed tested by myself was 8Mhz but more should be possible (Ben Krasnow used 18Mhz with his implemenation)
//!
//! ### Other....
//!
//! - Buffersize: Wherever a buffer is used it always needs to be of the size: `width / 8 * length`,
//!   where width and length being either the full e-ink size or the partial update window size
//!
//! # Examples
//!
//! ```ignore
//! use eink_waveshare_rs::{
//!     epd1in54::{EPD1in54, Buffer1in54},
//!     graphics::{Display, DisplayRotation},
//!     prelude::*,
//! }; 
//! 
//! // Setup EPD
//! let mut epd = EPD1in54::new(&mut spi, cs_pin, busy_in, dc, rst, &mut delay)?;
//!
//! // Use display graphics
//! let mut buffer = Buffer1in54::default();
//! let mut display = Display::new(epd.width(), epd.height(), &mut buffer.buffer);
//! 
//! // Write some hello world in the screenbuffer
//! display.draw(
//!     Font6x8::render_str("Hello World!")
//!         .with_stroke(Some(Color::Black))
//!         .with_fill(Some(Color::White))
//!         .translate(Coord::new(5, 50))
//!         .into_iter(),
//! );
//!
//! // Display updated frame
//! epd.update_frame(&mut spi, &display.buffer()).unwrap();
//! epd.display_frame(&mut spi).expect("display frame new graphics");
//! 
//! // Set the EPD to sleep
//! epd.sleep(&mut spi).expect("sleep");
//! ```
//!
//!
#![no_std]
#![deny(warnings)]

#[cfg(feature = "graphics")]
extern crate embedded_graphics;

#[cfg(feature = "graphics")]
pub mod graphics;

mod traits;

pub mod color;

/// Interface for the physical connection between display and the controlling device
mod interface;

#[cfg(feature = "epd4in2")]
pub mod epd4in2;

#[cfg(feature = "epd1in54")]
pub mod epd1in54;

#[cfg(feature = "epd2in9")]
pub mod epd2in9;

#[cfg(any(feature = "epd1in54", feature = "epd2in9"))]
pub(crate) mod type_a;

pub mod prelude {
    pub use traits::{WaveshareDisplay};
    pub use color::Color;
    pub use SPI_MODE;
}


extern crate embedded_hal as hal;
use hal::spi::{Mode, Phase, Polarity};

/// SPI mode -
/// For more infos see [Requirements: SPI](index.html#spi)
pub const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};
