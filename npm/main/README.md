# @zuolan/micucodeline

MicuCodeLine 是 MICU OpenClaudeCode 站特供版 Claude Code 状态栏工具，集成余额显示与主题配置。

## 安装
```bash
npm install -g @zuolan/micucodeline
```

安装后默认路径：`~/.claude/micucodeline/micucodeline`

## 重要：首次配置

**安装完成后必须先运行配置工具：**

```bash
# Linux/macOS
~/.claude/micucodeline/micucodeline

# Windows
C:\Users\你的用户名\.claude\micucodeline\micucodeline.exe
```

运行后会进入交互式配置界面，按照提示输入：
1. **API Token（系统访问令牌）**：在 OpenClaudeCode 控制台 → 左上角个人设置 → 安全设置 → 系统访问令牌 → 生成令牌
2. **用户 ID**：在个人设置页面，用户名称下方显示的 ID

配置参考图：https://github.com/zuoliangyu/MICUCODELINE/blob/master/assets/image2.png

## Claude Code 配置
在 `~/.claude/settings.json` 中设置：
```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/micucodeline/micucodeline",
    "padding": 0
  }
}
```

## 手动配置余额（可选）
如果不使用配置工具，也可以在 `settings.json` 的 `env` 中手动添加：
```json
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "xxx",
    "ANTHROPIC_BASE_URL": "xxx",
    "BALANCE_API_KEY": "YOUR_TOKEN",
    "BALANCE_API_USER": "12345",
    "BALANCE_API_URL": "https://www.openclaudecode.cn/api/user/self"
  }
}
```

## 使用
```bash
micucodeline --help        # 查看帮助
micucodeline --version     # 查看版本
micucodeline --init        # 初始化配置与主题
micucodeline --config      # 打开配置面板
micucodeline --theme nord  # 指定主题运行
```

## 链接
- 官网：https://www.openclaudecode.cn/
- GitHub：https://github.com/zuoliangyu/MICUCODELINE
- 原作者：https://github.com/Haleclipse/CCometixLine
