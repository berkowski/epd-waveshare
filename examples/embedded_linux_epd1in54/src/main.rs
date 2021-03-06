// the library for the embedded linux device
extern crate linux_embedded_hal as lin_hal;
use lin_hal::spidev::{self, SpidevOptions};
use lin_hal::sysfs_gpio::Direction;
use lin_hal::Delay;
use lin_hal::{Pin, Spidev};

// the eink library
extern crate epd_waveshare;
use epd_waveshare::{
    epd1in54::{Buffer1in54, EPD1in54},
    graphics::{Display, DisplayRotation},
    prelude::*,
};

// Graphics
extern crate embedded_graphics;
use embedded_graphics::coord::Coord;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::prelude::*;
//use embedded_graphics::primitives::{Circle, Line};
use embedded_graphics::Drawing;

// HAL (Traits)
extern crate embedded_hal;
use embedded_hal::prelude::*;

// activate spi, gpio in raspi-config
// needs to be run with sudo because of some sysfs_gpio permission problems and follow-up timing problems
// see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues

fn main() {
    run().unwrap();
}

fn run() -> Result<(), std::io::Error> {
    // Configure SPI
    // SPI settings are from eink-waveshare-rs documenation
    let mut spi = Spidev::open("/dev/spidev0.0").expect("spidev directory");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(spidev::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("spi configuration");

    // Configure Digital I/O Pin to be used as Chip Select for SPI
    let cs_pin = Pin::new(26); //BCM7 CE0
    cs_pin.export().expect("cs_pin export");
    while !cs_pin.is_exported() {}
    cs_pin
        .set_direction(Direction::Out)
        .expect("cs_pin Direction");
    cs_pin.set_value(1).expect("cs_pin Value set to 1");

    // Configure Busy Input Pin
    let busy = Pin::new(5); //pin 29
    busy.export().expect("busy export");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction");
    //busy.set_value(1).expect("busy Value set to 1");

    // Configure Data/Command OutputPin
    let dc = Pin::new(6); //pin 31 //bcm6
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    // Configure Reset OutputPin
    let rst = Pin::new(16); //pin 36 //bcm16
    rst.export().expect("rst export");
    while !rst.is_exported() {}
    rst.set_direction(Direction::Out).expect("rst Direction");
    rst.set_value(1).expect("rst Value set to 1");

    // Configure Delay
    let mut delay = Delay {};

    // Setup of the needed pins is finished here
    // Now the "real" usage of the eink-waveshare-rs crate begins
    let mut epd = EPD1in54::new(&mut spi, cs_pin, busy, dc, rst, &mut delay)?;

    // Clear the full screen
    epd.clear_frame(&mut spi).expect("clear frame 1");
    epd.display_frame(&mut spi).expect("disp 1");

    println!("Test all the rotations");
    let mut buffer = Buffer1in54::default();
    let mut display = Display::new(epd.width(), epd.height(), &mut buffer.buffer);
    display.set_rotation(DisplayRotation::Rotate0);
    display.draw(
        Font6x8::render_str("Rotate 0!")
            .with_stroke(Some(Color::Black))
            .with_fill(Some(Color::White))
            .translate(Coord::new(5, 50))
            .into_iter(),
    );

    display.set_rotation(DisplayRotation::Rotate90);
    display.draw(
        Font6x8::render_str("Rotate 90!")
            .with_stroke(Some(Color::Black))
            .with_fill(Some(Color::White))
            .translate(Coord::new(5, 50))
            .into_iter(),
    );

    display.set_rotation(DisplayRotation::Rotate180);
    display.draw(
        Font6x8::render_str("Rotate 180!")
            .with_stroke(Some(Color::Black))
            .with_fill(Some(Color::White))
            .translate(Coord::new(5, 50))
            .into_iter(),
    );

    display.set_rotation(DisplayRotation::Rotate270);
    display.draw(
        Font6x8::render_str("Rotate 270!")
            .with_stroke(Some(Color::Black))
            .with_fill(Some(Color::White))
            .translate(Coord::new(5, 50))
            .into_iter(),
    );

    // Display updated frame
    epd.update_frame(&mut spi, &display.buffer()).unwrap();
    epd.display_frame(&mut spi)
        .expect("display frame new graphics");
    delay.delay_ms(5000u16);

    // a quickly moving `Hello World!`
    display.set_rotation(DisplayRotation::Rotate0);
    epd.set_lut(&mut spi, Some(RefreshLUT::QUICK))
        .expect("SET LUT QUICK error");
    let limit = 20;
    for i in 0..limit {
        println!("Moving Hello World. Loop {} from {}", (i + 1), limit);

        display.draw(
            Font6x8::render_str("  Hello World! ")
                .with_style(Style {
                    fill_color: Some(Color::White),
                    stroke_color: Some(Color::Black),
                    stroke_width: 0u8, // Has no effect on fonts
                })
                .translate(Coord::new(5 + i * 6, 50))
                .into_iter(),
        );

        epd.update_frame(&mut spi, &display.buffer()).unwrap();
        epd.display_frame(&mut spi)
            .expect("display frame new graphics");
    }

    // Set the EPD to sleep
    epd.sleep(&mut spi).expect("sleep");

    Ok(())
}
