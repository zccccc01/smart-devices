# smart-devices

## 智能设备与接口课程的 lab

### 本人是用 rust 完成本课程的 lab, 需**自行配置** [rust 环境](https://www.rust-lang.org/zh-CN/learn/get-started)

我是仿着 STM32F1xx HAL 示例代码, 结合开发板的文档学着搞的, 代码不是很优雅(不符合最佳实践), 有什么问题欢迎**提 issue**

### 使用方法

```bash
sudo apt install libssl-dev pkg-config
rustup target add thumbv7m-none-eabi
cargo install cargo-binutils
rustup component add llvm-tools-preview
cargo install cargo-generate
git clone https://github.com/zccccc01/smart-devices.git
cd smart-devices/embedded/src # 把每个实验代码拷到main.rs里, 然后 cargo build --target thumbv7m-none-eabi
# cargo objcopy --release -- -O ihex firmware.hex # 转成 hex 文件, 用 flymcu 烧录到开发板
# 打开smart-devices/embedded/target/thumbv7m-none-eabi/debug 把.elf的文件用objcopy转成.hex文件, 用flymcu烧录到开发板
cargo objcopy --release -- -O ihex firmware.hex
```

### 给 stm32 烧录 RT-Thread(linux)

```bash
sudo apt install gcc-arm-none-eabi
sudo apt-get install scons
```

#### env 工具

```bash
wget https://gitee.com/RT-Thread-Mirror/env/raw/master/install_ubuntu.sh
chmod 777 install_ubuntu.sh
./install_ubuntu.sh --gitee
rm install_ubuntu.sh
source ~/.env/env.sh
```

#### 编译 RT-Thread

```bash
git clone https://github.com/RT-Thread/rt-thread.git
cd rt-thread/bsp/stm32/stm32f103-atk-warshipv3
scons # 编译
arm-none-eabi-objcopy -O ihex rt-thread.elf rt-thread.hex # 转成 hex 文件, 用 flymcu 烧录到开发板
```

#### 缺少宏的报错

rtconfig.h 里手动加上一句：

```c
#define BSP_STM32_UART_V1_TX_TIMEOUT 10
```

### rust-embedded 友链

- [Rust 编译器对 ARM Cortex-M3 (Thumbv7m-none-eabi) 的支持](https://doc.rust-lang.org/rustc/platform-support/thumbv7m-none-eabi.html)
- [Rust 嵌入式编程指南 - 简介](https://docs.rust-embedded.org/book/intro/index.html)
- [安装 Rust 嵌入式环境](https://xxchang.github.io/book/intro/install.html)
- [STM32F1xx HAL 库文档](https://docs.rs/stm32f1xx-hal/latest/stm32f1xx_hal/)
- [Cortex-M 库文档](https://docs.rs/cortex-m/latest/cortex_m/)
- [STM32F1xx HAL 示例代码](https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples)
