#!/bin/bash

echo "===================================="
echo "PakePlus Docker 构建脚本"
echo "===================================="
echo

echo "正在构建Docker镜像..."
docker build -t pakeplus-builder .
if [ $? -ne 0 ]; then
    echo "Docker镜像构建失败！"
    exit 1
fi

echo
echo "Docker镜像构建成功！"
echo
echo "正在启动容器进行构建..."
docker run --rm -v "$(pwd)/output:/app/src-tauri/target" pakeplus-builder
if [ $? -ne 0 ]; then
    echo "构建失败！"
    exit 1
fi

echo
echo "===================================="
echo "构建完成！"
echo "构建文件位于: output/release/bundle/"
echo "===================================="