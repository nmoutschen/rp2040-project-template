#![no_std]
#![no_main]

use hal::gpio::{pin, Pins};
// use micromath::F32Ext;
use rp2040_hal as hal;

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 7;

pub const BTN_A: usize = 12;
pub const BTN_B: usize = 13;
pub const BTN_X: usize = 14;
pub const BTN_Y: usize = 15;

// const ROW_BYTES: usize = 12;
// const BCD_FRAMES: usize = 15;
// const BITSTREAM_LENGTH: usize = HEIGHT * ROW_BYTES * BCD_FRAMES;

// fn get_gamma_lut() -> [[u16; 256]; 3] {
//     let mut gamma_lut = [[0; 256]; 3];
//     // Create 14-bits gamma luts
//     let gamma: f32 = 2.8;
//     for i in 0..256 {
//         gamma_lut[0][i] = ((i as f32 / 255.0).powf(gamma) * 16383.0 + 0.5) as u16;
//         gamma_lut[0][i] = ((i as f32 / 255.0).powf(gamma) * 16383.0 + 0.5) as u16;
//         gamma_lut[0][i] = ((i as f32 / 255.0).powf(gamma) * 16383.0 + 0.5) as u16;
//     }
//     gamma_lut
// }

pub struct UnicornPins {
    pub data: pin::Pin<pin::bank0::Gpio8, pin::PushPullOutput>,
    pub clock: pin::Pin<pin::bank0::Gpio9, pin::PushPullOutput>,
    pub latch: pin::Pin<pin::bank0::Gpio10, pin::PushPullOutput>,
    pub blank: pin::Pin<pin::bank0::Gpio11, pin::PushPullOutput>,

    pub row_0: pin::Pin<pin::bank0::Gpio22, pin::PushPullOutput>,
    pub row_1: pin::Pin<pin::bank0::Gpio21, pin::PushPullOutput>,
    pub row_2: pin::Pin<pin::bank0::Gpio20, pin::PushPullOutput>,
    pub row_3: pin::Pin<pin::bank0::Gpio19, pin::PushPullOutput>,
    pub row_4: pin::Pin<pin::bank0::Gpio18, pin::PushPullOutput>,
    pub row_5: pin::Pin<pin::bank0::Gpio17, pin::PushPullOutput>,
    pub row_6: pin::Pin<pin::bank0::Gpio16, pin::PushPullOutput>,

    pub btn_a: pin::Pin<pin::bank0::Gpio12, pin::Input<pin::PullUp>>,
    pub btn_b: pin::Pin<pin::bank0::Gpio13, pin::Input<pin::PullUp>>,
    pub btn_x: pin::Pin<pin::bank0::Gpio14, pin::Input<pin::PullUp>>,
    pub btn_y: pin::Pin<pin::bank0::Gpio15, pin::Input<pin::PullUp>>,

    pub led: pin::Pin<pin::bank0::Gpio25, pin::PushPullOutput>,
}

/// Implementation of the Unicorn HAT's GPIO pins.
///
/// This is a re-implementation from the C++ one from Pimoroni.
/// See https://github.com/pimoroni/pimoroni-pico/blob/main/libraries/pico_unicorn/pico_unicorn.cpp
pub struct PicoUnicorn {
    pub pins: UnicornPins,
    // gamma_lut: [[u16; 256]; 3],
    // bitstream: [u8; BITSTREAM_LENGTH as usize],
}

impl PicoUnicorn {
    pub fn new(pins: Pins) -> Self {
        // Initialize all the pins for the Unicorn HAT
        let pins = UnicornPins {
            // Setup pins
            data: pins.gpio8.into_push_pull_output(),
            clock: pins.gpio9.into_push_pull_output(),
            latch: pins.gpio10.into_push_pull_output(),
            blank: pins.gpio11.into_push_pull_output(),

            row_0: pins.gpio22.into_push_pull_output(),
            row_1: pins.gpio21.into_push_pull_output(),
            row_2: pins.gpio20.into_push_pull_output(),
            row_3: pins.gpio19.into_push_pull_output(),
            row_4: pins.gpio18.into_push_pull_output(),
            row_5: pins.gpio17.into_push_pull_output(),
            row_6: pins.gpio16.into_push_pull_output(),

            // Setup button inputs
            btn_a: pins.gpio12.into_pull_up_input(),
            btn_b: pins.gpio13.into_pull_up_input(),
            btn_x: pins.gpio14.into_pull_up_input(),
            btn_y: pins.gpio15.into_pull_up_input(),

            // Onboard LED
            led: pins.gpio25.into_push_pull_output(),
        };

        // // Initialize the BCD timing values and row selects in the bitstream
        // let mut bitstream: [u8; BITSTREAM_LENGTH as usize] = [0; BITSTREAM_LENGTH as usize];
        // for row in 0..HEIGHT {
        //     for frame in 0..BCD_FRAMES {
        //         // Determine offset in the buffer for this row/frame
        //         let offset = (row * ROW_BYTES * BCD_FRAMES) + (ROW_BYTES * frame);
        //         let row_select_offset = offset + 9;
        //         let bcd_offset = offset + 10;

        //         // The last BCD frame is used to allow the fets to discharge to avoid ghosting
        //         if frame == BCD_FRAMES - 1 {
        //             bitstream[row_select_offset] = 0b11111111;

        //             let bcd_ticks: u16 = 0xffff;
        //             bitstream[bcd_offset + 1] = ((bcd_ticks & 0xff00) >> 8) as u8;
        //             bitstream[bcd_offset] = (bcd_ticks & 0xff) as u8;

        //             for col in 0..6 {
        //                 bitstream[offset + col] = 0xff;
        //             }
        //         } else {
        //             let row_select_mask = !(1 << (7 - row));
        //             bitstream[row_select_offset] = row_select_mask;

        //             let bcd_ticks = 2_u16.pow(frame as u32);
        //             bitstream[bcd_offset + 1] = ((bcd_ticks & 0xff00) >> 8) as u8;
        //             bitstream[bcd_offset] = (bcd_ticks & 0xff) as u8;
        //         }
        //     }
        // }

        PicoUnicorn {
            pins,
            // gamma_lut: get_gamma_lut(),
            // bitstream,
        }
    }
}
