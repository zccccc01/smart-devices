#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // 获取外设访问控制器 Peripherals(外围设备)
    let dp = pac::Peripherals::take().unwrap();

    // 获取 GPIOB 和 GPIOE 以控制 LED
    let mut gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);

    loop {
        led.toggle();
        asm::delay(1_000_000);
    }
}
