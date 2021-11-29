#![no_std]
#![no_main]

pub extern crate rp2040_hal as hal;

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb565, RgbColor},
};
use embedded_hal::{blocking::delay::DelayUs, digital::v2::OutputPin, spi::MODE_0};
use embedded_time::rate::*;
use hal::{gpio::pin, pac, sio, spi};
use st7789::ST7789;

mod internal_pin {
    hal::bsp_pins!(
        Gpio0 { name: gpio0 },
        Gpio1 { name: gpio1 },
        Gpio2 { name: gpio2 },
        Gpio3 { name: gpio3 },
        Gpio4 { name: gpio4 },
        Gpio5 { name: gpio5 },
        Gpio6 {
            name: led_r,
            aliases: { FunctionPwm: LedRed }
        },
        Gpio7 {
            name: led_g,
            aliases: { FunctionPwm: LedGreen }
        },
        Gpio8 {
            name: led_b,
            aliases: { FunctionPwm: LedBlue }
        },
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
        Gpio20 { name: bl_en },
        Gpio21 { name: gpio21 },
        Gpio22 { name: i2c_int },
        Gpio23 { name: b_power_save },
        Gpio24 { name: vbus_detect },
        Gpio25 { name: led },
        Gpio26 { name: adc0 },
        Gpio27 { name: adc1 },
        Gpio28 { name: adc2 },
        Gpio29 {
            name: voltage_monitor
        },
    );
}

pub struct DummyPin;

impl OutputPin for DummyPin {
    type Error = ();

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub type Screen = ST7789<
    SPIInterface<
        spi::Spi<spi::Enabled, pac::SPI0, 8>,
        pin::Pin<pin::bank0::Gpio16, pin::PushPullOutput>,
        pin::Pin<pin::bank0::Gpio17, pin::PushPullOutput>,
    >,
    DummyPin,
>;

pub struct PicoDisplay {
    pub a: pin::Pin<pin::bank0::Gpio12, pin::PullUpInput>,
    pub b: pin::Pin<pin::bank0::Gpio13, pin::PullUpInput>,
    pub x: pin::Pin<pin::bank0::Gpio14, pin::PullUpInput>,
    pub y: pin::Pin<pin::bank0::Gpio15, pin::PullUpInput>,
    pub led: pin::Pin<pin::bank0::Gpio25, pin::PushPullOutput>,

    pub screen: Screen,
}

impl PicoDisplay {
    pub fn new(
        io: pac::IO_BANK0,
        pads: pac::PADS_BANK0,
        sio: sio::SioGpioBank0,
        spi0: pac::SPI0,
        resets: &mut pac::RESETS,
        delay: &mut impl DelayUs<u32>,
    ) -> Self {
        let pins = internal_pin::Pins::new(io, pads, sio, resets);

        // Turn RGB LED off
        // TODO: add support for RGB LED
        pins.led_r.into_push_pull_output().set_high().unwrap();
        pins.led_g.into_push_pull_output().set_high().unwrap();
        pins.led_b.into_push_pull_output().set_high().unwrap();

        // Initialize screen
        let dc = pins.lcd_dc.into_push_pull_output();
        let cs = pins.lcd_cs.into_push_pull_output();
        let _spi_sclk = pins.spi_sclk.into_mode::<pin::FunctionSpi>();
        let _spi_mosi = pins.spi_mosi.into_mode::<pin::FunctionSpi>();

        let spi_screen = spi::Spi::<_, _, 8>::new(spi0).init(
            resets,
            125_000_000u32.Hz(),
            16_000_000u32.Hz(),
            &MODE_0,
        );
        let spii_screen = SPIInterface::new(spi_screen, dc, cs);

        let mut screen = ST7789::new(spii_screen, DummyPin, 135, 240);
        screen.init(delay).unwrap();
        screen
            .set_orientation(st7789::Orientation::Portrait)
            .unwrap();
        screen.clear(Rgb565::BLACK).unwrap();

        Self {
            a: pins.btn_a.into_pull_up_input(),
            b: pins.btn_b.into_pull_up_input(),
            x: pins.btn_x.into_pull_up_input(),
            y: pins.btn_y.into_pull_up_input(),
            led: pins.led.into_push_pull_output(),

            screen,
        }
    }
}
