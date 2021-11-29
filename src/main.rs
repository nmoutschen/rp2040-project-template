//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawLE},
    mono_font::{ascii::FONT_9X18, MonoTextStyleBuilder},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;
use rp2040_hal as hal;
use rp2040_test::PicoDisplay;

use hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

static FERRIS: &[u8] = include_bytes!("../ferris.raw");

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    // Initialize PicoDisplay
    let mut display = PicoDisplay::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        pac.SPI0,
        &mut pac.RESETS,
        &mut delay,
    );

    // Draw ferris
    let ferris: ImageRawLE<Rgb565> = ImageRaw::new(FERRIS, 64);
    let ferris_img = Image::new(&ferris, Point::new(60, 50));
    ferris_img.draw(&mut display.screen).unwrap();

    // Write some text
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X18)
        .text_color(Rgb565::new(183, 65, 14))
        .build();

    Text::new("Hello, Rust!", Point::new(60, 50 + 43 + 10), text_style)
        .draw(&mut display.screen)
        .unwrap();

    loop {}
}
