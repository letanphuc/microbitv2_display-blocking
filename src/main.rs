#![no_main]
#![no_std]
mod font;

use defmt::println;
use defmt_rtt as _;

use panic_halt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use microbit::{
    board::Board,
    display::nonblocking::Display,
    hal::Timer,
    pac::{self, interrupt, TIMER1},
};

// We use TIMER1 to drive the display, and RTC0 to update the animation.
// We set the TIMER1 interrupt to a higher priority than RTC0.

static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(mut board) = Board::take() {
        // Create display
        let display = Display::new(board.TIMER1, board.display_pins);

        let mut timer = Timer::new(board.TIMER0);

        cortex_m::interrupt::free(move |cs| {
            *DISPLAY.borrow(cs).borrow_mut() = Some(display);
        });
        unsafe {
            board.NVIC.set_priority(pac::Interrupt::TIMER1, 128);
            pac::NVIC::unmask(pac::Interrupt::TIMER1);
        };
        loop {
            for ch in font::PRINTABLE_START..(font::PRINTABLE_START + font::PRINTABLE_COUNT) {
                println!("main loop start, ch = {}!", ch);
                let buf = font::character(ch as u8);

                cortex_m::interrupt::free(|cs| {
                    if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                        display.show(&buf);
                    }
                });
                timer.delay(500_000);
            }
        }
    }

    loop {}
}

#[interrupt]
fn TIMER1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.handle_display_event();
        }
    });
}
