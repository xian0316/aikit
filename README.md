# AIKit

> AI 工具技能管理器 —— 跨工具统一管理 Claude Code / Cursor / Codex 等 AI 工具的技能（Skills）。

## 下载

前往 [Releases 页面](https://github.com/xian0316/aikit/releases/latest) 下载最新版本：

| 文件 | 说明 |
|---|---|
| `AIKit_x.x.x_x64-setup.exe` | Windows 安装包（推荐，NSIS） |
| `AIKit_x.x.x_x64_en-US.msi` | Windows MSI 安装包 |

> 应用内置自动更新，安装后可在「设置 → 关于」中检查更新。

## 功能

- **技能仓库管理**：添加任意 GitHub 仓库作为技能源，自动 clone/sync
- **skills.sh 市场**：浏览在线技能注册表，一键安装
- **多工具支持**：同一技能可启用到多个 AI 工具（Claude Code、Codex、OpenCode 等）
- **软链接安装**：默认使用 symlink/junction，技能更新自动同步到所有工具
- **一键更新**：检测技能仓库变更，单独更新某个技能
- **应用自更新**：基于 Tauri Updater，自动检查并安装新版本

## 技术栈

- **桌面框架**：Tauri v2（Rust 后端 + 原生 HTML/CSS/JS 前端）
- **图标**：Tabler Icons（内联 SVG，零依赖）
- **自动更新**：Tauri Updater + GitHub Releases 签名分发

## 开发

```bash
# 安装依赖
npm install

# 开发模式（热重载）
npm run tauri dev

# 构建生产版本
npm run tauri build
```

## 发布新版本

```bash
# 1. 更新三处版本号
#    - src-tauri/tauri.conf.json
#    - src-tauri/Cargo.toml
#    - package.json

# 2. 提交并打 tag
git commit -am "release vx.x.x"
git tag vx.x.x
git push origin master
git push origin vx.x.x

# 3. GitHub Actions 自动构建并发布 Release
```

## License

MIT
