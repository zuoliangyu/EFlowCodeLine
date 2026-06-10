# EFlowCodeLine

[![npm version](https://img.shields.io/npm/v/@zuolan/eflowcodeline)](https://www.npmjs.com/package/@zuolan/eflowcodeline)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

EFlowCodeLine 是 **EFlowCode 专用版** Claude Code 状态栏工具，Rust 编写，支持余额显示、三行 Powerline 布局以及智能终端宽度换行。

> GitHub: https://github.com/zuoliangyu/EFlowCodeLine

## 安装

```bash
npm install -g @zuolan/eflowcodeline
```

安装时会**自动**完成，无需手动下载或复制：

1. 按当前平台下载对应的二进制文件
2. 复制到 `~/.claude/eflowcodeline/`（Windows 为 `%USERPROFILE%\.claude\eflowcodeline\`）
3. 运行 `eflowcodeline --init` 初始化配置

## 配置 Claude Code

在 `~/.claude/settings.json` 中加入（路径指向上面自动安装好的二进制）：

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/eflowcodeline/eflowcodeline"
  }
}
```

Windows 示例：

```json
{
  "statusLine": {
    "command": "C:/Users/<USERNAME>/.claude/eflowcodeline/eflowcodeline.exe",
    "padding": 0,
    "type": "command"
  }
}
```

## 功能特性

- **余额显示**：自动读取 Claude Code 已有 Token，无需额外配置
- **三行 Powerline 布局**（v1.7.0）：上下文/花费 → 路径/分支 → 已用/余额/EFLOWCODE，固定霓虹渐变配色
- **智能换行**：自动检测终端宽度，超出时按 Segment 换行，不截断内容
- **Git / 目录 / 上下文窗口 / 会话** 等多种 Segment

> v1.7.0 起状态栏样式锁定为内置主题，不再支持 `--theme` 切换或自定义 `config.toml`。

## 命令

```bash
eflowcodeline --help      # 查看帮助
eflowcodeline --init      # 初始化配置文件
eflowcodeline --config    # 打开 TUI 配置面板（查看用）
eflowcodeline --check     # 检查配置是否合法
eflowcodeline --print     # 打印当前配置
```

## 支持平台

| 平台 | 包名 |
|------|------|
| macOS x64 | `@zuolan/eflowcodeline-darwin-x64` |
| macOS arm64 | `@zuolan/eflowcodeline-darwin-arm64` |
| Linux x64 | `@zuolan/eflowcodeline-linux-x64` |
| Linux x64 (musl) | `@zuolan/eflowcodeline-linux-x64-musl` |
| Windows x64 | `@zuolan/eflowcodeline-win32-x64` |

## License

MIT
