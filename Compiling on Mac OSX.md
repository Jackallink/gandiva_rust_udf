# Step3
## 一、安装Homebrew
Homebrew是macOS上的包管理器，可以帮助你安装交叉编译器和其他必要的工具。在终端中运行以下命令以安装Homebrew：
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```
## 二、安装交叉编译器
使用Homebrew安装musl-cross，这是一个用于编译Linux目标的工具链：
```bash
brew install FiloSottile/musl-cross/musl-cross
```
## 三、安装Rust目标架构
添加x86_64-unknown-linux-musl目标架构到你的Rust工具链：
```bash
rustup target add x86_64-unknown-linux-musl
```
## 四、配置Rust编译设置
修改配置文件~/.cargo/config（如果没有可以新建），添加如下内容：
```toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```