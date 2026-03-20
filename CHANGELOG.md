# Changelog

## [1.5.1] - 2026-03-20

### Added

- 新增 ARM Linux 构建支持，覆盖嵌入式设备场景
  - `aarch64-unknown-linux-gnu` (ARM64，树莓派 4/5、Jetson、RK3588 等)
  - `aarch64-unknown-linux-musl` (ARM64 musl 静态链接)
  - `armv7-unknown-linux-gnueabihf` (ARMv7，树莓派 2/3、32 位 ARM 设备)
- 新增 3 个 NPM 平台包：`@zuolan/eflowcodeline-linux-arm64`、`@zuolan/eflowcodeline-linux-arm64-musl`、`@zuolan/eflowcodeline-linux-armv7`
- CI 使用 `cross` 工具实现 ARM 交叉编译
- NPM 安装脚本自动检测 ARM 架构并选择正确的二进制文件

## [1.5.0] - 2025

### Added

- 可配置品牌静态文本
- 主题默认禁用 output_style 和 group，显示 EFLOWCODE 品牌

### Fixed

- NPM scope 从 @haleclipse 更名为 @zuolan

## [1.4.0] - 2025

### Added

- 初始 NPM 发布支持
- 跨平台二进制分发（macOS / Linux / Windows）
- 自动更新功能
- TUI 配置面板
