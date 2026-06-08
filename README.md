# herdr-zh

> 🇨🇳 **[herdr](https://github.com/ogulcancelik/herdr) 中文版** — 终端中的 agent 多路复用器
>
> 🔗 原项目：[github.com/ogulcancelik/herdr](https://github.com/ogulcancelik/herdr) · [English README](https://github.com/ogulcancelik/herdr/blob/master/README.md)
>
> 本仓库定期与上游同步，中文翻译贡献欢迎提 PR。

<p align="center">
  <img src="assets/logo.png" alt="herdr" width="100" />
</p>

<p align="center">
  <a href="https://herdr.dev">herdr.dev</a> · <a href="https://herdr.dev/docs/integrations/">集成</a> · <a href="https://herdr.dev/docs/configuration/">配置</a> · <a href="https://herdr.dev/docs/socket-api/">Socket API</a>
</p>

---

**终端中的 agent 多路复用器。**

工作区、标签页、窗格，原生鼠标支持。一眼掌握所有 agent 状态：阻塞、工作中、已完成。可分离重连，agent 持续运行。纯终端，无 GUI，无 Electron。单个 Rust 二进制文件，零依赖。

## 安装

```bash
curl -fsSL https://raw.githubusercontent.com/zarttic/herdr-zh/master/install.sh | sh
```

或从 [Releases](https://github.com/zarttic/herdr-zh/releases) 手动下载对应平台的二进制文件。

或从源码构建：

```bash
git clone https://github.com/zarttic/herdr-zh.git
cd herdr-zh
cargo build --release
./target/release/herdr
```

## 快速开始

```bash
herdr
```

在项目目录下启动，即可进入工作区。常用快捷键（默认前缀 `ctrl+b`）：

| 快捷键 | 功能 |
|--------|------|
| `prefix+c` | 新建标签页 |
| `prefix+v` / `prefix+-` | 分屏 |
| `prefix+x` | 关闭窗格 |
| `prefix+w` | 切换工作区 |
| `prefix+shift+n` | 新建工作区 |
| `prefix+q` | 分离 |
| `prefix+b` | 切换侧边栏 |

按 `ctrl+b q` 分离客户端，服务器继续运行。再次运行 `herdr` 重新挂载。

## 核心特性

- **会话持久化** — 窗格进程在分离后继续运行，重启后可恢复
- **Agent 感知** — 侧边栏实时显示 agent 状态（🔴阻塞 / 🟡工作中 / 🔵完成 / 🟢空闲）
- **原生鼠标** — 点击、拖拽、分屏，所见即所得
- **18 个内置主题** — Catppuccin、Tokyo Night、Gruvbox、Solarized 等
- **远程支持** — 原生 SSH，分离后远程重连
- **Agent 可编程** — 通过 Unix Socket API 让 agent 自动创建窗格、读取输出

## 支持的 Agent

| Agent | 空闲/完成 | 工作中 | 阻塞 |
|-------|-----------|--------|------|
| Claude Code | ✓ | ✓ | ✓ |
| Codex | ✓ | ✓ | ✓ |
| Gemini CLI | ✓ | ✓ | ✓ |
| GitHub Copilot CLI | ✓ | ✓ | ✓ |
| Cursor Agent | ✓ | ✓ | ✓ |
| Aider | ✓ | ✓ | ✓ |
| OpenCode | ✓ | ✓ | ✓ |
| Amp | ✓ | ✓ | ✓ |
| Kimi CLI | ✓ | ✓ | ✓ |

完整列表和集成安装详见 [集成文档](https://herdr.dev/docs/integrations/)。

## 许可证

双许可：[AGPL-3.0-or-later](https://github.com/ogulcancelik/herdr/blob/master/LICENSE) 或商业许可（联系 hey@herdr.dev）。
