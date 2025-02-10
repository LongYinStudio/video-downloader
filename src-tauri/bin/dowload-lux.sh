#!/bin/bash

# 定义 lux 的 GitHub 发布页面的基础 URL
BASE_URL="https://github.com/iawia002/lux/releases/latest/download"
# 定义版本号
VERSION="0.24.1"

# NOTE:linux和windows arm使用的太少了，默认不启用

# 定义需要下载的文件列表
FILES=(
	"lux_${VERSION}_Linux_x86_64.tar.gz" # Linux x86_64
	# "lux_${VERSION}_Linux_arm64.tar.gz"   # Linux ARM64
	"lux_${VERSION}_Darwin_x86_64.tar.gz" # macOS x86_64
	"lux_${VERSION}_Darwin_arm64.tar.gz"  # macOS ARM64
	"lux_${VERSION}_Windows_x86_64.zip"   # Windows x86_64
	# "lux_${VERSION}_Windows_arm64.zip"    # Windows ARM64
)
# 定义目标文件名（Tauri 所需格式）
TARGET_FILES=(
	"lux-x86_64-unknown-linux-gnu" # Linux x86_64
	# "lux-aarch64-unknown-linux-gnu"   # Linux ARM64
	"lux-x86_64-apple-darwin"        # macOS x86_64
	"lux-aarch64-apple-darwin"       # macOS ARM64
	"lux-x86_64-pc-windows-msvc.exe" # Windows x86_64
	# "lux-aarch64-pc-windows-msvc.exe" # Windows ARM64
)

# 下载、解压并重命名文件
for i in "${!FILES[@]}"; do
	FILE="${FILES[i]}"
	TARGET_FILE="${TARGET_FILES[i]}"
	echo "下载 $FILE..."

	# 下载文件
	curl -L -o "$FILE" "$BASE_URL/$FILE"
	echo "$FILE 下载成功."

	# 根据文件类型解压
	if [[ "$FILE" == *.tar.gz ]]; then
		tar -xvf "$FILE"
	elif [[ "$FILE" == *.zip ]]; then
		unzip "$FILE"
	fi
	rm "$FILE" # 删除压缩包

	# 获取解压后的可执行文件名
	if [[ "$FILE" == *_Windows_* ]]; then
		EXECUTABLE="lux.exe" # Windows 平台解压后为 lux.exe
	else
		EXECUTABLE="lux" # 非 Windows 平台解压后为 lux
	fi

	# 重命名文件为目标文件名
	mv "$EXECUTABLE" "$TARGET_FILE"
	echo "$FILE 已重命名为 $TARGET_FILE 并保存."
done

echo "所有下载和重命名完成."
