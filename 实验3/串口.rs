#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::asm;

use cortex_m_rt::entry;
use nb::block;
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc::RccExt,
    serial::{Config, Serial},
    time::MicroSeconds,
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
        .sysclk(72.MHz()) // 系统时钟配置为 72 MHz
        .freeze(&mut dp.FLASH.constrain().acr);

    // 获取 GPIOA 外设
    let mut gpioa = dp.GPIOA.split();
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10.into_floating_input(&mut gpioa.crh);

    // 获取 GPIOB 以控制 LED
    let mut gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);
    led.set_low(); // 点亮 LED，表示程序启动

    // MAPR 寄存器用于配置复用功能
    let mut afio = dp.AFIO.constrain();
    let mapr = &mut afio.mapr;

    // 使用 Serial::new 来初始化 USART1
    let serial = Serial::new(
        dp.USART1,
        (tx, rx),                                  // 传入 TX 和 RX 管脚
        mapr,                                      // 配置与 USART2 所在的 APB1 总线关联
        Config::default().baudrate(115_200.bps()), // 设置波特率为 115200
        &clocks,                                   // 传入时钟配置
    );

    let (mut tx, mut rx) = serial.split();

    let mut delay = cp.SYST.delay(&clocks);

    let two_second = MicroSeconds::secs(3);

    delay.delay(two_second);

    let s = "Hello, world!\r\n";
    if tx.write_str(s).is_ok() {
        led.set_high(); // 关闭 LED，表示数据已发送
    }

    delay.delay(two_second);

    delay.delay(two_second);

    // 接收数据并再次点亮 LED
    if block!(rx.read()).is_ok() {
        led.set_low(); // 点亮 LED，表示数据已接收
    }

    // 接收数据并回显
    loop {
        if let Ok(byte) = block!(rx.read()) {
            led.set_low(); // 点亮 LED，表示数据已接收
            if let Err(e) = block!(tx.write(byte)) {
                // 处理写入错误
                led.set_high();
            }
        } else {
            // 处理读取错误
            led.set_high();
        }

        delay.delay_ms(100_u8); // 小延迟，避免CPU过载
    }

    loop {}
}
