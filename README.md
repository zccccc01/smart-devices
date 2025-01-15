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
cd smart-devices/embedded/src # 把每个实验代码拷到main.rs里, 然后 ./build.sh(需要修改路径为你的路径)
# 打开smart-devices/embedded/target/thumbv7m-none-eabi/debug 里面有个output.hex 文件, 用flymcu烧录到开发板
```

### rust-embedded 友链

- [Rust 编译器对 ARM Cortex-M3 (Thumbv7m-none-eabi) 的支持](https://doc.rust-lang.org/rustc/platform-support/thumbv7m-none-eabi.html)
- [Rust 嵌入式编程指南 - 简介](https://docs.rust-embedded.org/book/intro/index.html)
- [安装 Rust 嵌入式环境](https://xxchang.github.io/book/intro/install.html)
- [STM32F1xx HAL 库文档](https://docs.rs/stm32f1xx-hal/latest/stm32f1xx_hal/)
- [Cortex-M 库文档](https://docs.rs/cortex-m/latest/cortex_m/)
- [STM32F1xx HAL 示例代码](https://github.com/stm32-rs/stm32f1xx-hal/tree/master/examples)
