#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*, rcc::RccExt, time::MicroSeconds};

#[entry]
fn main() -> ! {
    // 获取外设访问控制器 Peripherals(外围设备)
    let dp = pac::Peripherals::take().unwrap();
    // 获取Cortex-M微控制器核心外设
    let cp = cortex_m::Peripherals::take().unwrap();
    // 设置系统时钟
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);

    // 获取 GPIOB 以控制 LED
    let mut gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);

    // 初始化系统滴答定时器(SysTick)作为延迟源
    let mut delay = cp.SYST.delay(&clocks);

    let one_second = MicroSeconds::secs(1);
    loop {
        led.set_low();
        delay.delay(one_second);
        led.set_high();
        delay.delay(one_second);
    }
}
