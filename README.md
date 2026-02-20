# EFlowCodeLine

![Language:Rust](https://camo.githubusercontent.com/b858ce7ffb2054312ada07b2be7896f91eb95e0ca40f502793f23f96e0dd180d/68747470733a2f2f696d672e736869656c64732e696f2f7374617469632f76313f6c6162656c3d4c616e6775616765266d6573736167653d5275737426636f6c6f723d6f72616e6765267374796c653d666c61742d737175617265)
![License:MIT](https://camo.githubusercontent.com/c6a8d48e8b6ef330ef240499a811f77e629e4bdecc8f2327120137fb2406144d/68747470733a2f2f696d672e736869656c64732e696f2f7374617469632f76313f6c6162656c3d4c6963656e7365266d6573736167653d4d495426636f6c6f723d626c7565267374796c653d666c61742d737175617265)

EFlowCodeLine 是 **EFlowCode 专用版** Claude Code 状态栏工具，集成余额显示与主题/TUI 配置。

- 原作者仓库：https://github.com/Haleclipse/CCometixLine
- EFlowCodeLine 官网：https://e-flowcode.cc

## 效果预览

![效果预览](https://github.com/zuoliangyu/EFlowCodeLine/blob/master/assets/image.png)

## 功能特性

- **余额自动显示**：直接读取 Claude Code 已有配置，无需额外输入
- 多主题 / 交互式 TUI 配置面板
- Git / 目录 / 上下文 / 会话等常用 Segment
- 跨平台发布（macOS / Linux / Windows）

## 安装

从 [Releases](https://github.com/zuoliangyu/EFlowCodeLine/releases) 页面下载对应平台的二进制文件。

```bash
# macOS / Linux
mkdir -p ~/.claude/eflowcodeline
wget https://github.com/zuoliangyu/EFlowCodeLine/releases/latest/download/eflowcodeline-macos-x64.tar.gz
tar -xzf eflowcodeline-macos-x64.tar.gz
cp eflowcodeline ~/.claude/eflowcodeline/
chmod +x ~/.claude/eflowcodeline/eflowcodeline
```

Windows 直接下载 `.exe`，双击运行即可进入配置界面，程序会自动安装到 `%USERPROFILE%\.claude\eflowcodeline\` 目录。

## 配置 Claude Code 状态栏

在 `~/.claude/settings.json` 中加入：

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
    "type": "command",
    "command": "%USERPROFILE%\\.claude\\eflowcodeline\\eflowcodeline.exe"
  }
}
```

## 余额显示说明

**无需任何额外配置。**

EFlowCodeLine 自动读取 Claude Code 的 `~/.claude/settings.json` 中已有的环境变量：

| 变量 | 说明 |
|------|------|
| `ANTHROPIC_AUTH_TOKEN` 或 `ANTHROPIC_API_KEY` | 你的 API Key（EFlowCode 控制台获取） |
| `ANTHROPIC_BASE_URL` | 中转站地址，如 `https://e-flowcode.cc` |

只要这两个变量已配置（Claude Code 正常工作的前提），余额就会自动显示，无需再做任何设置。

> **注意**：余额显示依赖 new-api 后台的「额度查询接口返回令牌额度而非用户额度」开关处于**关闭**状态（即返回用户账户余额而非 API Key 额度）。
> 管理员可在 `新API后台 → 设置 → 运营设置` 中找到该开关。

## 首次使用

直接双击可执行文件（或命令行运行），会自动弹出交互式主菜单，可在其中：

- 打开 TUI 配置面板（调整 Segment、主题、颜色等）
- 初始化 / 检查配置

```bash
eflowcodeline --init        # 初始化配置文件
eflowcodeline --check       # 校验当前配置是否正确
eflowcodeline --print       # 输出当前配置内容
eflowcodeline --config      # 打开交互式 TUI 配置面板
eflowcodeline --theme nord  # 临时指定主题运行
```
