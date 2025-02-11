# Image to ASCII Art Converter

一个用 Rust 编写的命令行工具，可以将图片转换为 ASCII 艺术字符画。支持彩色和单色输出，并能保持图片原有的宽高比。

## 特性

- 支持多种图片格式（PNG, JPEG, etc.）
- 支持彩色和单色输出模式
- 提供三种预设尺寸（小、中、大）
- 支持自定义输出宽度
- 可调节字符宽高比以适应不同终端
- 并行处理以提高性能

## 安装

确保你的系统已安装 Rust 工具链，然后：

```bash
git clone https://github.com/julyfinal/img2ascii
cd img2ascii
cargo build --release
```

## 使用方法

基本用法：

```bash
cargo run -- -i path/to/image.jpg
```

完整参数说明：
```bash
cargo run -- --help
```

参数选项：
- `-i, --image <FILE>`: 输入图片的路径（必需）
- `-s, --size <SIZE>`: 输出尺寸 [可选值: small, medium, large] [默认值: medium]
- `-m, --mode <MODE>`: 输出模式 [可选值: monochrome, color] [默认值: color]
- `-w, --width <WIDTH>`: 自定义输出宽度（可选）
- `-r, --char-ratio <RATIO>`: 字符宽高比调整 [默认值: 0.5]

示例：
```bash
# 使用默认设置
cargo run -- -i image.jpg

# 指定大小和模式
cargo run -- -i image.jpg -s large -m monochrome

# 自定义宽度和字符比例
cargo run -- -i image.jpg -w 100 -r 0.45
```

## 项目结构

```
img2ascii/
├── src/
│   └── main.rs          # 主程序代码
├── Cargo.toml           # 项目配置和依赖
├── .gitignore          # Git 忽略文件
└── README.md           # 项目文档
```

## 依赖项

- `image = "0.24"` - 图像处理
- `colored = "2.0"` - 终端彩色输出
- `clap = "4.4"` - 命令行参数解析
- `rayon = "1.8"` - 并行计算

## 待改进功能

1. 错误处理改进：
   - 文件存在性检查
   - 输入参数验证
   - 更友好的错误信息提示

2. 功能扩展：
   - 支持输出到文件
   - 添加更多 ASCII 字符集选项
   - 添加图像预处理选项（亮度/对比度调整）
   - 支持更多图片格式

3. 代码质量：
   - 添加单元测试
   - 完善代码注释
   - 添加 CI/CD 配置

## 性能优化

当前版本已实现的优化：
- 使用 rayon 进行并行处理
- 优化内存分配
- 使用更快的图像缩放算法
- 减少不必要的内存复制

## 环境要求

- Rust 1.70.0 或更高版本
- Cargo 包管理器

## 致谢

- 感谢 [itoa](https://github.com/solst-ice/itoa) 项目的启发


