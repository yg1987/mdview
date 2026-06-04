# mdview

简单的 Markdown 预览桌面工具。拖入 .md 文件即可查看渲染效果,支持代码高亮、数学公式、流程图。

**状态**:代码全部完成,只剩编译产出 `.exe` 需要安装 Rust 工具链。

---

## 项目结构

```
markdownView/
├── docs/superpowers/
│   ├── specs/2026-06-04-mdview-design.md     # 设计文档
│   └── plans/2026-06-04-mdview.md            # 实现计划
├── src/                                       # 前端(浏览器加载)
│   ├── index.html                            # 主页面
│   ├── main.js                               # 渲染流水线(拖放/marked/KaTeX/Mermaid)
│   ├── styles.css                            # GitHub 风格主题
│   └── vendor/                               # 第三方库(本地打包,离线可用)
│       ├── marked.min.js          (39 KB)
│       ├── highlight.min.js       (119 KB)
│       ├── katex.min.js           (269 KB)
│       ├── katex.min.css          (23 KB)
│       └── mermaid.min.js         (3.2 MB)
├── src-tauri/                                # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   └── src/main.rs              # read_md_file 命令
├── assets/                                   # 图标
│   ├── icon.ico                  (Windows)
│   ├── icon.icns                 (macOS,512x512 PNG-embedded)
│   ├── icon.png                  (512x512 通用)
│   ├── generate-icons.ps1        (图标生成脚本)
│   └── generate-icns.ps1         (icns 生成脚本)
├── test-samples/                             # 5 个测试样本 md
├── preview.html                # 浏览器预览(无需 Rust 编译)
├── scripts/
│   ├── validate.js              # 项目文件验证(无需 Rust)
│   └── build.ps1                # 一键构建脚本
└── README.md
```

---

## 快速开始

### 1. 浏览器预览(无需 Rust,立即可用)

直接双击打开 `preview.html`,在 Chrome 中拖入任意 `.md` 文件即可看效果。

**注意**:浏览器版用 `FileReader` 读文件,不是最终 Tauri 版的 `read_md_file` Rust 命令。其他完全一致。

### 2. 编译 .exe(需要 Rust)

**a) 安装 Rust**(一次性,5 分钟):

打开 PowerShell,运行:
```powershell
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
& "$env:TEMP\rustup-init.exe" -y --default-toolchain stable --default-host x86_64-pc-windows-msvc
```

**装完后关闭并新开一个 PowerShell 窗口**。

**b) 编译 mdview**:

```powershell
cd D:\opencode_work\markdownView
powershell -ExecutionPolicy Bypass -File scripts\build.ps1
```

**首次编译 5-15 分钟**(下载+编译 Tauri 运行时)。脚本会自动:
1. 检查 Rust 环境
2. 装 Tauri CLI(如未装)
3. `cargo check` 验证
4. `cargo tauri build` 编译
5. 报告产物路径

**c) 拿产物**:
编译成功后,`.exe` 在:
```
src-tauri\target\release\mdview.exe              # 单文件可执行
src-tauri\target\release\bundle\nsis\mdview_*_setup.exe  # NSIS 安装包
```

**d) 分发**:把 `.exe` 复制给任何人,Win10/11 双击即用,无需装任何运行时。

---

## 手动开发(可选)

如果想改代码热重载看效果:
```powershell
# 首次:装 Tauri CLI
cargo install tauri-cli --version "^2.0"

# 开发模式
cargo tauri dev
```

---

## 平台支持

| 平台 | 状态 | 说明 |
|---|---|---|
| Windows 10/11 | ✅ 已验证 | 用系统自带 WebView2,零依赖 |
| macOS 12+ | ✅ 可编译 | 在 Mac 上 `cargo tauri build`,生成 .dmg / .app |
| Linux | ⚠️ 未测试 | Tauri 支持,但本项目未验证 |

**macOS 编译**:在 Mac 上克隆此项目,跑 `cargo tauri build` 即可生成 `.dmg`。

---

## 验证项目完整性

无需 Rust,跑:
```powershell
node scripts\validate.js
```

会检查所有文件、CSS 选择器、JS 函数、vendor 库、git 历史。

---

## 设计文档

- **设计**: `docs/superpowers/specs/2026-06-04-mdview-design.md`
- **计划**: `docs/superpowers/plans/2026-06-04-mdview.md`
