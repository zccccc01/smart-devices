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

    // 获取 GPIOB 和 GPIOE 以控制 LED
    let mut gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);

    // 获取 GPIOE 以控制用户按钮
    let mut gpioe = dp.GPIOE.split();
    let button = gpioe.pe4.into_pull_up_input(&mut gpioe.crl);

    // 检测按钮是否按下
    let mut button_up: bool = true;

    // 初始化系统滴答定时器(SysTick)作为延迟源
    let mut delay = cp.SYST.delay(&clocks);

    let ten_micro_seconds = MicroSeconds::millis(10);
    loop {
        let key_result = button.is_low(); // 判断按钮是否按下（低电平表示按下）

        // 按钮按下时触发，避免长按重复触发
        if button_up && key_result {
            button_up = false; // 防止长按持续触发
            led.toggle(); // 切换 LED 状态
            delay.delay(ten_micro_seconds); // 按键消抖延时
        }
        // 按钮松开时允许重新触发
        else if !key_result {
            button_up = true; // 按钮松开，准备下次按下
            delay.delay_ms(10u8); // 防止抖动
        }
    }
}
