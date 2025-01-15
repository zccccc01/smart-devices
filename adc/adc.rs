#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;

use panic_halt as _;
use stm32f1xx_hal::{
    adc,
    pac::{self},
    prelude::*,
    rcc::RccExt,
    serial::{Config, Serial},
};

#[entry]
fn main() -> ! {
    // 获取外设访问控制器 Peripherals(外围设备)
    let dp = pac::Peripherals::take().unwrap();

    // 获取Cortex-M微控制器核心外设
    let cp = cortex_m::Peripherals::take().unwrap();

    // 设置系统时钟
    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz()) // 使用外部晶振，频率为 8 MHz
        .sysclk(56.MHz())
        .adcclk(14.MHz())
        .freeze(&mut dp.FLASH.constrain().acr);

    let mut adc = adc::Adc::adc1(dp.ADC1, clocks);

    // 获取 GPIOA 外设
    let mut gpioa = dp.GPIOA.split();
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10.into_floating_input(&mut gpioa.crh);

    // MAPR 寄存器用于配置复用功能
    let mut afio = dp.AFIO.constrain();
    let mapr = &mut afio.mapr;

    // 使用 Serial::new 来初始化 USART2
    let serial = Serial::new(
        dp.USART1,
        (tx, rx),                                  // 传入 TX 和 RX 管脚
        mapr,                                      // 配置与 USART2 所在的 APB1 总线关联
        Config::default().baudrate(115_200.bps()), // 设置波特率为 115200
        &clocks,                                   // 传入时钟配置
    );

    let mut delay = cp.SYST.delay(&clocks);

    let (mut tx, mut _rx) = serial.split();

    delay.delay_ms(2000_u16);
    loop {
        // 每隔2秒执行一次
        delay.delay_ms(2000_u16); // 延迟2000毫秒

        // 读取温度传感器
        let temp = adc.read_temp();
        if let Err(_e) = tx.write_fmt(format_args!("temp is {}\n", temp)) {}
    }
}
