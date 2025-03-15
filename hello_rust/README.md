# hello rust

AWTK Rust 示例代码。

## 准备

1. 编译 [AWTK](https://github.com/zlgopen/awtk/blob/master/README_zh.md)。

2. 配置 [AWTK Rust 代码生成器](../awtk_rust_gen/README.md) 所需环境。

3. 使用 [sync_awtk.sh](./sync_awtk.sh) 同步 AWTK。

   当目录结构为：

   ```
   awtk
   awtk-rust
   ├── awtk_rust_gen
   └── hello_rust
   ```

   可以直接执行：

   ```cmd
   ./sync_awtk.sh
   ```

   否则执行：

   ```cmd
   ./sync_awtk.sh <awtk path> <awtk_rust_gen path>
   ```

   - `awtk path`：AWTK 路径
   - `awtk_rust_gen path`：AWTK Rust 代码生成器路径

   > 在 Windows 平台上，可以使用 Git Bash 执行。
   >
   > 若没有 Bash，则需要根据 sync_awtk.sh 的内容手工执行相应的命令。

4. 生成资源

   ```cmd
   python ./scripts/update_res.py all
   ```

## 编译

```cmd
cargo build
```

## 运行

```cmd
cargo run
```

### 按照指定窗口大小运行

指定窗口大小为 800x480：

```cmd
cargo run -- 800 480
```
