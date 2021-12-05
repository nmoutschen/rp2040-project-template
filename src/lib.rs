#![no_std]
#![no_main]

pub extern crate rp2040_hal as hal;

extern crate cortex_m_rt;
pub use cortex_m_rt::entry;

pub mod terminal;

#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

pub use hal::pac;
hal::bsp_pins!(
    Gpio0 { name: gpio0 },
    Gpio1 { name: gpio1 },
    Gpio2 { name: gpio2 },
    Gpio3 { name: gpio3 },
    Gpio4 { name: gpio4 },
    Gpio5 { name: gpio5 },
    Gpio6 { name: gpio6 },
    Gpio7 { name: gpio7 },
    Gpio8 { name: gpio8 },
    Gpio9 { name: gpio9 },
    Gpio10 { name: gpio10 },
    Gpio11 { name: gpio11 },
    Gpio12 { name: btn_a },
    Gpio13 { name: btn_b },
    Gpio14 { name: btn_x },
    Gpio15 { name: btn_y },
    Gpio16 {
        name: lcd_dc,
        aliases: { FunctionSpi: Miso }
    },
    Gpio17 {
        name: lcd_cs,
        aliases: { FunctionSpi: LcdCs }
    },
    Gpio18 {
        name: spi_sclk,
        aliases: { FunctionSpi: Sclk }
    },
    Gpio19 {
        name: spi_mosi,
        aliases: { FunctionSpi: Mosi }
    },
    Gpio20 { name: gpio20 },
    Gpio21 { name: gpio21 },
    Gpio22 { name: gpio22 },
    Gpio23 { name: b_power_save },
    Gpio24 { name: vbus_detect },
    Gpio25 { name: led },
    Gpio26 { name: gpio26 },
    Gpio27 { name: gpio27 },
    Gpio28 { name: gpio28 },
    Gpio29 {
        name: voltage_monitor
    },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

pub struct DummyPin;

impl embedded_hal::digital::v2::OutputPin for DummyPin {
    type Error = ();

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
