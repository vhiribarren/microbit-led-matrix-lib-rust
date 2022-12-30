/*
MIT License

Copyright (c) 2022 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::hal::gpio::Level;
use microbit::hal::prelude::*;
use panic_halt as _;

/*
If there are not regular switch between two elements of A, B, C or D, the panel shutdown.
A minimal block like this one is needed to allow the system working:
   loop {
        pin_a.set_low().unwrap();
        pin_a.set_high().unwrap();
   }

Correct order is:
    for _ in range(64):
        select colors (r1, g1, b1, r2, g2, b2)
        set clock to H
        set clock to L
    set OE to H
    select A, B, C, D
    set latch to H
    set latch to L
    set OE to L
*/

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let mut pin_r1 = board.pins.p0_02.into_push_pull_output(Level::Low);
    let mut pin_g1 = board.pins.p0_03.into_push_pull_output(Level::Low);
    let mut pin_b1 = board.pins.p0_04.into_push_pull_output(Level::Low);
    let mut pin_r2 = board.display_pins.col3.into_push_pull_output(Level::Low);
    let mut pin_g2 = board.display_pins.col1.into_push_pull_output(Level::Low);
    let mut pin_b2 = board.buttons.button_a.into_push_pull_output(Level::Low);
    let mut pin_a = board.display_pins.col4.into_push_pull_output(Level::Low);
    let mut pin_b = board.display_pins.col2.into_push_pull_output(Level::Low);
    let mut pin_c = board.pins.p0_10.into_push_pull_output(Level::Low);
    let mut pin_d = board.pins.p0_09.into_push_pull_output(Level::Low);
    let mut pin_clk = board.display_pins.col5.into_push_pull_output(Level::Low);
    let mut pin_lat = board.buttons.button_b.into_push_pull_output(Level::Low);
    let mut pin_oe = board.pins.p0_12.into_push_pull_output(Level::Low);

    for col in 00..64 {
        if col < 64 / 3 {
            pin_r1.set_low().unwrap();
            pin_g1.set_low().unwrap();
            pin_b1.set_high().unwrap();
            pin_r2.set_low().unwrap();
            pin_g2.set_low().unwrap();
            pin_b2.set_high().unwrap();
        } else if col > 2 * 64 / 3 {
            pin_r1.set_high().unwrap();
            pin_g1.set_low().unwrap();
            pin_b1.set_low().unwrap();
            pin_r2.set_high().unwrap();
            pin_g2.set_low().unwrap();
            pin_b2.set_low().unwrap();
        } else {
            pin_r1.set_high().unwrap();
            pin_g1.set_high().unwrap();
            pin_b1.set_high().unwrap();
            pin_r2.set_high().unwrap();
            pin_g2.set_high().unwrap();
            pin_b2.set_high().unwrap();
        }
        pin_clk.set_high().unwrap();
        pin_clk.set_low().unwrap();
    }

    loop {
        pin_oe.set_high().unwrap();
        pin_a.set_low().unwrap();
        pin_b.set_low().unwrap();
        pin_c.set_low().unwrap();
        pin_d.set_low().unwrap();
        pin_lat.set_high().unwrap();
        pin_lat.set_low().unwrap();
        pin_oe.set_low().unwrap();

        pin_oe.set_high().unwrap();
        pin_a.set_high().unwrap();
        pin_b.set_low().unwrap();
        pin_c.set_low().unwrap();
        pin_d.set_low().unwrap();
        pin_lat.set_high().unwrap();
        pin_lat.set_low().unwrap();
        pin_oe.set_low().unwrap();

        pin_oe.set_high().unwrap();
        pin_a.set_low().unwrap();
        pin_b.set_low().unwrap();
        pin_c.set_low().unwrap();
        pin_d.set_high().unwrap();
        pin_lat.set_high().unwrap();
        pin_lat.set_low().unwrap();
        pin_oe.set_low().unwrap();

        pin_oe.set_high().unwrap();
        pin_a.set_high().unwrap();
        pin_b.set_low().unwrap();
        pin_c.set_low().unwrap();
        pin_d.set_high().unwrap();
        pin_lat.set_high().unwrap();
        pin_lat.set_low().unwrap();
        pin_oe.set_low().unwrap();
    }
}
