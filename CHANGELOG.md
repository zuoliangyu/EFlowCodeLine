# Changelog

## [1.7.1] - 2026-05-30

### Changed

- **模型名称段简化**：移除本地硬编码的模型名称重映射，直接使用 Claude Code 上报的原始 `display_name`，避免厂商更新模型后映射失效的问题
- `ModelEntry` 结构体移除 `display_name` 字段，仅保留 `pattern` 和 `context_limit`

## [1.7.0] - 2026-05-22

### Added

- 三行 Powerline 状态栏布局（行 1：模型/上下文/花费/耗时；行 2：cwd/dir/git；行 3：已用/余额/EFLOWCODE）
- 新增 `Cwd` 段位：显示完整工作目录，自动 `$HOME → ~` 缩写，超过 5 段时中间用 `...` 折叠
- 新增 `Used` 段位：从 new-api `/api/user/self` 计算累计已用额度，固定 USD 显示
- `Balance` 段位重写：拆出已用额度，仅输出 `余额:$X`，无限额度时显示 `余额:∞`
- `Used` 与 `Balance` 共享一次 HTTP 请求（缓存于内存 + 磁盘）
- `Branding` 段默认文本 `EFLOWCODE`，固定显示
- 模型识别补 Opus 4.7 / Sonnet 4.7 / Haiku 4.7（含 `[1m]` 1M 上下文变体）

### Changed

- **状态栏锁定为单一 Powerline 主题**：内置霓虹渐变配色（深蓝→紫→粉→金），不再支持主题切换
- 段位顺序硬编码：`Cwd` 段前自动换到第 2 行，`Used` 段前自动换到第 3 行

### Removed

- **Breaking**：删除 9 个旧主题文件（cometix/default/minimal/gruvbox/nord/powerline-dark/powerline-light/powerline-rose-pine/powerline-tokyo-night），仅保留单一硬编码主题
- **Breaking**：删除 `--theme` 命令行参数
- **Breaking**：不再读取 `~/.claude/eflowcodeline/config.toml`，所有主题/段位定义硬编码在二进制中
- **Breaking**：删除 `Group` 段位（new-api billing 接口不返回 group 信息）
- **Breaking**：`BalanceConfig` 删除 `exchange_rate` 与旧 `user_id` 字段（金额固定 USD，不再做汇率换算）
- TUI 配置面板的"保存主题"按钮改为只读提示，不再写盘

### Fixed

- `generate_wrapped` 单行短路路径未尊重段间强制换行的 bug

## [1.6.0] - 2026-03-20

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
