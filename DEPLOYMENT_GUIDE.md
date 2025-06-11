# PakePlus 独立部署指南

本指南将帮助您将 PakePlus 项目配置为您自己的独立部署版本。

## 快速开始

### 方式一：图形界面配置（推荐）

1. 启动应用后，点击右上角版本号下拉菜单中的「项目配置」
2. 在配置页面填写以下信息：
   - **GitHub用户名**：您的GitHub用户名或组织名
   - **仓库名称**：您的应用仓库名称
   - **自定义域名**：您的域名（可选，留空将使用GitHub Pages默认域名）
   - **联系邮箱**：您的联系邮箱（可选）
3. 点击「保存配置」按钮
4. 重启应用使配置生效

### 方式二：手动修改配置文件

如果您熟悉代码，也可以直接修改 `src/config/repository.ts` 文件中的配置信息：

```typescript
export const repositoryConfig = {
    owner: 'your-github-username',    // 替换为您的GitHub用户名
    repo: 'YourAppName',             // 替换为您的应用名称
    domain: 'your-domain.com',       // 替换为您的域名（可选）
    email: 'your-email@example.com'  // 替换为您的邮箱（可选）
}
```

> **注意**：推荐使用图形界面配置，更加简单直观，无需修改代码。

### 2. 运行配置脚本

```bash
npm run configure
```

这个命令会自动更新以下文件中的硬编码值：
- `src-tauri/tauri.conf.json` - 产品名称、标识符、更新URL
- `package.json` - 包名称
- `src-tauri/Cargo.toml` - Rust 项目名称和库名称
- `src-tauri/src/main.rs` - 库调用
- `src-tauri/src/command/cmds.rs` - GitHub API 调用

### 3. 开发和构建

现在您可以正常开发和构建您的应用：

```bash
# 开发模式
npm run dev

# 构建应用
npm run build

# 构建 Tauri 应用
npm run tauri build
```

## 配置说明

### 自动配置的内容

配置脚本会自动处理以下内容：

1. **产品名称**: 将所有 `WebAppBuilder` 替换为您的应用名称
2. **GitHub 信息**: 将所有 `Sjj1024/PakePlus` 替换为您的 GitHub 用户名和仓库名
3. **域名**: 将所有 `sjj1024.github.io` 替换为您的域名
4. **应用标识符**: 自动生成基于您应用名称的唯一标识符

### 需要手动处理的内容

以下内容可能需要您手动修改：

1. **文档文件**: `README.md`, `README_ZH.md`, `README_JP.md` 等
2. **许可证**: `LICENSE` 文件中的版权信息
3. **图标和资源**: 替换为您自己的应用图标
4. **文档网站**: `docs/` 目录下的文档内容

## 注意事项

1. **备份**: 在运行配置脚本前，建议备份您的项目
2. **Git**: 配置脚本会修改多个文件，建议在 Git 中提交更改
3. **测试**: 配置完成后，请测试应用的各项功能
4. **更新**: 如果需要修改配置，请重新编辑 `repository.ts` 并运行 `npm run configure`

## 故障排除

### 配置脚本失败

如果配置脚本运行失败，请检查：
1. `src/config/repository.ts` 文件格式是否正确
2. 是否有文件权限问题
3. Node.js 版本是否兼容

### 构建失败

如果构建失败，请检查：
1. 所有依赖是否正确安装
2. Rust 环境是否正确配置
3. Tauri 依赖是否正确安装

## 支持

如果您在部署过程中遇到问题，可以：
1. 查看原项目的 Issues
2. 参考原项目的文档
3. 在您的项目中创建 Issue

---

祝您部署成功！🎉