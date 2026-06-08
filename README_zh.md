# herdr

> 🇨🇳 这是 [herdr](https://github.com/ogulcancelik/herdr) 的中文镜像分支。
> 本仓库定期与上游同步，所有中文翻译贡献欢迎提 PR。

<p align="center">
  <img src="assets/logo.png" alt="herdr" width="100" />
</p>

<p align="center">
  <a href="https://herdr.dev">herdr.dev</a> · <a href="#安装">安装</a> · <a href="#快速开始">快速开始</a> · <a href="#支持的-agent">支持的 Agent</a> · <a href="https://herdr.dev/docs/integrations/">集成</a> · <a href="https://herdr.dev/docs/configuration/">配置</a> · <a href="https://herdr.dev/docs/socket-api/">Socket API</a>
</p>

---

**终端中的 agent 多路复用器。**

工作区、标签页、窗格。原生鼠标支持：点击、拖拽、分屏。一览所有 agent 状态：阻塞、工作中、已完成。可分离和重新挂载，agent 持续运行。无需 GUI 应用、Electron 或 macOS 专属包装。你看到的是 agent 自己的终端，而非某种二次渲染。

---

## 安装

```bash
curl -fsSL https://herdr.dev/install.sh | sh
```

或使用 Homebrew 安装：

```bash
brew install herdr
```

或使用 mise 安装：

```bash
mise use -g herdr
```

或从 [Releases](https://github.com/ogulcancelik/herdr/releases) 下载二进制文件。需要 Linux 或 macOS。

## 快速开始

在项目目录下启动 Herdr：

```bash
herdr
```

Herdr 会启动或挂载到一个后台会话服务器。按 `ctrl+b`，然后 `shift+n` 创建工作区。在根窗格中运行 agent。按 `ctrl+b` 然后 `v` 或 `-` 分屏，`ctrl+b` 然后 `c` 创建标签页，`ctrl+b` 然后 `w` 切换工作区。

按 `ctrl+b q` 分离客户端。服务器和窗格进程会继续运行。打开另一个终端再次运行 `herdr` 即可重新挂载。

## 核心概念

**服务器与客户端。** 默认情况下，`herdr` 挂载到后台服务器。分离仅关闭客户端。`herdr server stop` 停止默认服务器并终止其窗格。命名会话是独立的服务器命名空间：使用 `herdr session attach work`、`herdr session stop work` 和 `herdr session list` 来管理完全独立的运行时状态。

**工作区、标签页、窗格。** 工作区是项目级容器。标签页在工作区内对窗格进行分组。窗格是真实的终端进程，而非重新渲染的 agent 视图。

**复制。** Herdr 复制窗格文本，而非侧边栏。在窗格内拖拽选择、双击单词或 token，或按 `prefix+[` 进入键盘复制模式。在复制模式下，使用 `h/j/k/l`、`w/b/e` 和 `{`/`}` 移动，按 `v` 或空格开始选择，按 `y` 或回车复制，按 `q` 或 Esc 退出。在 PuTTY 和部分 SSH 终端中，按住 `Shift` 拖拽可使用终端自带选择，`Shift` + 右键粘贴。

**更新与恢复。** `herdr update` 安装新二进制文件，但运行中的服务器会继续使用旧进程，直到被停止或切换。停止旧服务器以使用新版本。停止会退出窗格进程。运行 `herdr server stop`，然后再次运行 `herdr` 用于默认会话。对于命名会话，运行 `herdr session stop <name>`，然后再次运行 `herdr session attach <name>`。`herdr update --handoff` 是实验性功能，尝试将实时窗格（包括开发服务器等前台进程）从旧服务器迁移到新服务器。

**快捷键。** Herdr 使用显式快捷键字符串。`prefix+n` 表示先按配置的前缀键，再按 `n`。`ctrl+alt+n`、`cmd+k`、`alt+1` 和功能键组合是直接终端模式快捷键，不需要前缀键。普通的可打印字符键（如 `n`）会拦截正常输入，因此请使用 `prefix+n`，除非你有意使用修饰键绑定。

**Agent 感知。** 侧边栏显示阻塞、工作中、已完成和空闲状态。默认通过进程名和终端输出进行检测。官方集成可添加原生会话标识用于恢复、语义状态报告或两者兼有。

## 更新

Herdr 会在新版本可用时通知你。手动运行：

```bash
herdr update
```

`herdr update` 适用于通过 Herdr 自身安装器管理的安装。Homebrew、mise 和 Nix 安装通过 `brew upgrade herdr`、`mise upgrade herdr` 或你的 Nix 工作流进行更新。详见[安装文档](https://herdr.dev/docs/install/)和[会话状态文档](https://herdr.dev/docs/session-state/)。

## 支持的 Agent

Herdr 与以下 agent 协同工作：

- Claude Code
- Codex
- Gemini CLI
- Amazon Q
- GitHub Copilot CLI
- Aider
- Cursor
- Windsurf
- Amp
- OpenCode
- Crush
- Kimi CLI
- 以及更多...

详见 [herdr.dev/docs/integrations](https://herdr.dev/docs/integrations/)。

## 许可证

MIT
