# EFlowCodeLine

![Language:Rust](https://camo.githubusercontent.com/b858ce7ffb2054312ada07b2be7896f91eb95e0ca40f502793f23f96e0dd180d/68747470733a2f2f696d672e736869656c64732e696f2f7374617469632f76313f6c6162656c3d4c616e6775616765266d6573736167653d5275737426636f6c6f723d6f72616e6765267374796c653d666c61742d737175617265)
![License:MIT](https://camo.githubusercontent.com/c6a8d48e8b6ef330ef240499a811f77e629e4bdecc8f2327120137fb2406144d/68747470733a2f2f696d672e736869656c64732e696f2f7374617469632f76313f6c6162656c3d4c6963656e7365266d6573736167653d4d495426636f6c6f723d626c7565267374796c653d666c61742d737175617265)

EFlowCodeLine 是 **EFlowCodeLine 专用版** Claude Code 状态栏工具，集成余额显示与主题/TUI 配置。
- 原作者仓库：https://github.com/Haleclipse/CCometixLine
- EFlowCodeLine 官网：https://e-flowcode.cc

## 效果预览

![效果预览](https://github.com/zuoliangyu/EFlowCodeLine/blob/master/assets/image.png)

## 功能特性
- 余额显示：对接 EFlowCodeLine new-api `/api/user/self`
- 多主题/交互式 TUI 配置
- Git/目录/上下文/会话等常用 Segment
- 跨平台发布（macOS/Linux/Windows）

## 安装

从 [Releases](https://github.com/zuoliangyu/EFlowCodeLine/releases) 页面下载对应平台的二进制文件：

```bash
# 以 macOS x64 为例
mkdir -p ~/.claude/eflowcodeline
wget https://github.com/zuoliangyu/EFlowCodeLine/releases/latest/download/eflowcodeline-macos-x64.tar.gz

tar -xzf eflowcodeline-macos-x64.tar.gz
cp eflowcodeline ~/.claude/eflowcodeline/
chmod +x ~/.claude/eflowcodeline/eflowcodeline
```

## Claude Code 配置
在 `~/.claude/settings.json` 中加入：

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/eflowcodeline/eflowcodeline",
    "padding": 0
  }
}
```
或者
```json
  "model": "opus",
  "statusLine": {
    "command": "%USERPROFILE%\\.claude\\eflowcodeline\\eflowcodeline.exe",
    "padding": 0,
    "type": "command"
  }
```

> 说明：`eflowcodeline --init` 只会生成本工具的 `config.toml` 和主题文件，**不会**自动修改 `settings.json`。

## 余额配置（重要）

### 方式一：使用配置工具（推荐）

**安装完成后必须先运行配置工具：**

```bash
# Linux/macOS
~/.claude/eflowcodeline/eflowcodeline

# Windows
C:\Users\你的用户名\.claude\eflowcodeline\eflowcodeline.exe
```

运行后会进入交互式配置界面，按照提示输入：
1. **API Token（系统访问令牌）**：在 EFlowCodeLine 控制台 → 左上角个人设置 → 安全设置 → 系统访问令牌 → 生成令牌
2. **用户 ID**：在个人设置页面，用户名称下方显示的 ID

![配置参考图](https://github.com/zuoliangyu/EFlowCodeLine/blob/master/assets/image2.png)

### 方式二：手动配置

在 `~/.claude/settings.json` 的 `env` 中添加以下变量：

- `BALANCE_API_KEY`：系统访问令牌
- `BALANCE_API_USER`：用户 ID（昵称下方）
- `BALANCE_API_URL`：可选，默认 `https://e-flowcode.cc/api/user/self`

示例：
```json
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "xxx",
    "ANTHROPIC_BASE_URL": "xxx",
    "BALANCE_API_KEY": "YOUR_TOKEN",
    "BALANCE_API_USER": "12345",
    "BALANCE_API_URL": "https://e-flowcode.cc/api/user/self"
  },
  "statusLine": {
    "type": "command",
    "command": "~/.claude/eflowcodeline/eflowcodeline",
    "padding": 0
  }
}
```

## 使用方式
```bash
eflowcodeline --init        # 初始化配置与主题目录
eflowcodeline --check       # 校验当前配置是否正确
eflowcodeline --print       # 输出当前配置内容
eflowcodeline --config      # 打开交互式 TUI 配置面板
eflowcodeline --theme nord  # 临时指定主题运行
```

## 声明
当前项目部分代码以及 review 由 Codex 完成。
