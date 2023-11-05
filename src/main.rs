#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use {defmt_rtt as _, panic_probe as _}; // global logger

use embassy_executor::Spawner;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    info!("Hello world");
    unwrap!(spawner.spawn(blink(p.P0_22.into())));
}

#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);
    loop {
        Timer::after_millis(990).await;
        led.set_low();
        Timer::after_millis(10).await;
        led.set_high();
    }
}
