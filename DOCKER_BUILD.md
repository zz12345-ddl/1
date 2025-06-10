# PakePlus Docker 构建指南

本指南将帮助你使用Docker构建PakePlus项目，无需在本地安装Rust和Node.js环境。

## 前置要求

- 已安装 Docker Desktop
- 确保Docker服务正在运行

## 快速开始

### Windows用户

1. 打开命令提示符或PowerShell
2. 进入项目目录
3. 运行构建脚本：
   ```cmd
   docker-build.bat
   ```

### Linux/macOS用户

1. 打开终端
2. 进入项目目录
3. 给脚本添加执行权限：
   ```bash
   chmod +x docker-build.sh
   ```
4. 运行构建脚本：
   ```bash
   ./docker-build.sh
   ```

## 手动构建步骤

如果你想手动执行构建过程：

### 1. 构建Docker镜像

```bash
docker build -t pakeplus-builder .
```

### 2. 运行构建容器

```bash
# Windows
docker run --rm -v "%cd%\output:/app/src-tauri/target" pakeplus-builder

# Linux/macOS
docker run --rm -v "$(pwd)/output:/app/src-tauri/target" pakeplus-builder
```

### 3. 查看构建结果

构建完成后，生成的文件将位于 `output/release/bundle/` 目录中：

- Windows: `output/release/bundle/nsis/PakePlus_x.x.x_x64-setup.exe`
- Linux: `output/release/bundle/deb/pakeplus_x.x.x_amd64.deb`
- macOS: `output/release/bundle/dmg/PakePlus_x.x.x_x64.dmg`

## Docker镜像说明

本Docker镜像包含：

- **基础系统**: Ubuntu 22.04
- **Node.js**: v20 (LTS)
- **pnpm**: v9.0.0
- **Rust**: v1.75.0
- **系统依赖**: 构建Tauri应用所需的所有依赖

## 支持的构建目标

- `x86_64-pc-windows-msvc` (Windows 64位)
- `aarch64-pc-windows-msvc` (Windows ARM64)
- `x86_64-unknown-linux-gnu` (Linux 64位)
- `aarch64-unknown-linux-gnu` (Linux ARM64)

## 存储空间说明

Docker构建会占用一定的存储空间：

- **Docker镜像**: 约2-3GB（包含完整的构建环境）
- **构建缓存**: 约500MB-1GB（依赖和中间文件）
- **输出文件**: 约50-100MB（最终的安装包）

## 清理Docker资源

构建完成后，你可以清理Docker资源以释放空间：

```bash
# 删除构建镜像
docker rmi pakeplus-builder

# 清理所有未使用的镜像和容器
docker system prune -a

# 清理构建缓存
docker builder prune
```

## 故障排除

### 常见问题

1. **Docker镜像构建失败**
   - 检查网络连接
   - 确保Docker有足够的磁盘空间
   - 尝试重新运行构建命令

2. **构建过程中断**
   - 检查系统资源（内存、CPU）
   - 确保Docker Desktop有足够的资源分配

3. **找不到构建文件**
   - 检查 `output` 目录是否正确创建
   - 确认构建过程没有错误

### 调试模式

如果需要调试构建过程，可以进入容器内部：

```bash
docker run -it --rm -v "$(pwd):/app" pakeplus-builder bash
```

然后在容器内手动执行构建命令：

```bash
cd /app
pnpm install
pnpm run tauri build
```

## 优势

✅ **环境隔离**: 不会污染本地环境  
✅ **一致性**: 确保所有人使用相同的构建环境  
✅ **便携性**: 可在任何支持Docker的系统上构建  
✅ **可重复性**: 构建结果完全可重现  

## 注意事项

⚠️ **首次构建**: 第一次构建会下载所有依赖，可能需要较长时间  
⚠️ **网络要求**: 需要稳定的网络连接下载依赖  
⚠️ **存储空间**: 确保有足够的磁盘空间（建议至少5GB可用空间）  
⚠️ **系统资源**: 构建过程会消耗较多CPU和内存资源