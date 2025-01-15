#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m::{asm::delay, interrupt::Mutex, peripheral::syst::SystClkSource};
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{
    adc,
    gpio::ExtiPin,
    gpio::Output,
    gpio::PushPull,
    pac,
    pac::interrupt,
    prelude::*,
    serial::{Config, Serial},
};

// 全局变量，用于共享资源
static G_ADC: Mutex<core::cell::RefCell<Option<adc::Adc<pac::ADC1>>>> =
    Mutex::new(core::cell::RefCell::new(None));
static G_SERIAL: Mutex<core::cell::RefCell<Option<stm32f1xx_hal::serial::Tx<pac::USART1>>>> =
    Mutex::new(core::cell::RefCell::new(None));
static G_LED: Mutex<
    core::cell::RefCell<Option<stm32f1xx_hal::gpio::gpiob::PB5<Output<PushPull>>>>,
> = Mutex::new(core::cell::RefCell::new(None));
static mut ENABLE_SAMPLING: bool = false;

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // 初始化时钟
    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .freeze(&mut dp.FLASH.constrain().acr);

    // 初始化 GPIO
    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();
    let mut gpioe = dp.GPIOE.split();

    // 初始化串口
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10.into_floating_input(&mut gpioa.crh);
    let mut afio = dp.AFIO.constrain();
    let mapr = &mut afio.mapr;

    let serial = Serial::new(
        dp.USART1,
        (tx, rx),
        mapr,
        Config::default().baudrate(115_200.bps()),
        &clocks,
    );

    let (tx, _) = serial.split();

    // 初始化 ADC
    let adc = adc::Adc::adc1(dp.ADC1, clocks);

    // 初始化 LED
    let led = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);

    // 初始化按键和外部中断
    let mut button = gpioe.pe4.into_pull_up_input(&mut gpioe.crl);
    button.make_interrupt_source(&mut afio);
    button.trigger_on_edge(&mut dp.EXTI, stm32f1xx_hal::gpio::Edge::RisingFalling);
    button.enable_interrupt(&mut dp.EXTI);

    // 将共享资源放入全局变量
    cortex_m::interrupt::free(|cs| {
        *G_ADC.borrow(cs).borrow_mut() = Some(adc);
        *G_SERIAL.borrow(cs).borrow_mut() = Some(tx);
        *G_LED.borrow(cs).borrow_mut() = Some(led);
    });

    // 配置 SysTick 定时器
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(clocks.sysclk().raw() * 20); // 每秒触发
    syst.enable_counter();
    syst.enable_interrupt();

    // 启用中断
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI4);
    }

    loop {}
}

// SysTick 中断，每秒触发一次温度采集
#[cortex_m_rt::exception]
fn SysTick() {
    cortex_m::interrupt::free(|cs| {
        if unsafe { ENABLE_SAMPLING } {
            if let Some(adc) = G_ADC.borrow(cs).borrow_mut().as_mut() {
                if let Some(tx) = G_SERIAL.borrow(cs).borrow_mut().as_mut() {
                    let temp = adc.read_temp();
                    writeln!(tx, "Temperature: {:.2} °C\n", temp).unwrap();
                }
            }
        }
    });
}

// 外部中断，检测按钮状态
#[interrupt]
fn EXTI4() {
    cortex_m::interrupt::free(|cs| {
        if let Some(led) = G_LED.borrow(cs).borrow_mut().as_mut() {
            let button_state = unsafe { (*pac::GPIOE::ptr()).idr.read().idr4().bit_is_clear() };

            // 延迟以消除抖动
            delay(8000); // 大约 8ms 的延迟
            let stable_state = unsafe { (*pac::GPIOE::ptr()).idr.read().idr4().bit_is_clear() };

            // 检查按键状态是否稳定
            if button_state == stable_state && button_state == true {
                // 切换 LED 状态和采样状态
                unsafe {
                    ENABLE_SAMPLING = !ENABLE_SAMPLING;
                }
                if unsafe { ENABLE_SAMPLING } {
                    led.set_high();
                } else {
                    led.set_low();
                }
            }
        }
    });

    // 清除中断标志
    let exti = unsafe { &(*pac::EXTI::ptr()) };
    exti.pr.write(|w| w.pr4().set_bit());
}
