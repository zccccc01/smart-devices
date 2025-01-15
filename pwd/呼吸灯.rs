#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc::RccExt,
    timer::{self, Timer},
};

#[entry]
fn main() -> ! {
    // 获取外设访问控制器 Peripherals(外围设备)
    let dp = pac::Peripherals::take().unwrap();
    // 获取Cortex-M微控制器核心外设
    let cp = cortex_m::Peripherals::take().unwrap();

    // 设置系统时钟
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);

    // 初始化系统滴答定时器(SysTick)作为延迟源
    let mut delay = cp.SYST.delay(&clocks);
    // 获取 GPIOB 以控制 LED
    let mut gpiob = dp.GPIOB.split();
    let led = gpiob.pb5.into_alternate_push_pull(&mut gpiob.crl);

    // 设置AFIO 用于重映射和复用功能
    let mut afio = dp.AFIO.constrain();

    // 00没有重映射, 10 部分重映射, 11 全部重映射(手册)
    // 将TIM3_CH2重映射到PB5
    unsafe {
        afio.mapr.modify_mapr(|_, w| w.tim3_remap().bits(0b10));
    }

    // 设置定时器以输出PWM信号
    let mut pwm = Timer::new(dp.TIM3, &clocks).pwm_hz(led, &mut afio.mapr, 1.kHz());

    // 在TIM3_CH2 (PB5)上启用PWM输出
    pwm.enable(timer::Channel::C2);

    // 获取PWM的最大占空比
    let max_duty = pwm.get_max_duty();

    let mut duty = 0; // 初始占空比
    let step = max_duty / 100; // 每次调整的步长,可以调整这个值来改变渐变的平滑度
    let mut direction = 1; // 方向标记,1表示增加,-1表示减少

    loop {
        // 更新占空比
        pwm.set_duty(timer::Channel::C2, duty);

        // 根据方向增加或减少占空比
        duty = (duty as i32 + (step as i32 * direction)).max(0) as u16;

        // 如果达到最大或最小占空比,则反转方向
        if duty >= max_duty || duty == 0 {
            direction *= -1;
        }

        delay.delay_us(22_000_u32);
    }
}
