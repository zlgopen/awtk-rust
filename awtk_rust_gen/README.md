# awtk rust gen

基于 [rust-bindgen](https://github.com/rust-lang/rust-bindgen) 的 AWTK Rust 代码生成器。

## 准备

1. 安装 Rust 和 Cargo

   参考文档 [Cargo 手册 —— 安装 Rust 和 Cargo](https://rustwiki.org/zh-CN/cargo/getting-started/installation.html)。

2. 安装 Clang

   参考文档 [bindgen 用户指南 —— Clang 安装](https://rust-lang.github.io/rust-bindgen/requirements.html)。

3. 安装 Python3

   参考文档 [Python 安装和使用](https://docs.python.org/zh-cn/3/using/index.html)。

   > AWTK 的需要用到 Python，一般来说，可以编译 AWTK 时，就意味着 Python 已经安装好了。

## 编译

```cmd
cargo build
```

## 使用

```cmd
cargo run -- -H <header file path>... -i <idl file path> -p <python config file path> -o <output file path>
```

- `-H` 或 `--header`：头文件路径（可以包含多个）
- `-i` 或 `--idl` ：idl 文件路径
- `-p` 或 `--py` ：python 配置文件路径
- `-o` 或 `--output`：输出文件路径
- `-h` 或 `--help`：打印帮助
- `-V` 或 `--version`：打印版本

### 示例

当 awtk 在当前目录的上两级目录时：

```cmd
cargo run -- -H ..\..\awtk\src\awtk.h -i ..\..\awtk\tools\idl_gen\idl.json -p ..\..\awtk\awtk_config.py -o .\awtk.rs
```
