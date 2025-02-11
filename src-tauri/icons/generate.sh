#!/bin/bash

# need imagemagick libicns iconutil(mac自带)

# 输入文件（512x512 PNG）
# INPUT="logo.png"
INPUT="logo_alpha.png" # tauri需要透明通道

# 定义需要的尺寸
SIZES=(16 24 32 48 64 128 256 512)

# 生成多尺寸 PNG 图标（Linux 和通用用途）
echo "Generating PNG icons for Linux..."
for size in "${SIZES[@]}"; do
	echo "Generating ${size}x${size} icon..."
	convert "$INPUT" -resize "${size}x${size}" -define png:format=png32 "${size}x${size}.png"
done
convert 512x512.png -define png:format=png32 icon.png

# 生成 ICO 文件（Windows）
echo "Generating Windows .ico file..."
for size in "${SIZES[@]}"; do
	echo "Generating ${size}x${size} icon..."
	convert "${size}x${size}.png" "${size}x${size}.ico"
done
convert 512x512.png icon.ico

# 生成 ICNS 文件（macOS）
echo "Generating macOS .icns file..."
# # 使用 iconutil 生成 .icns 文件
iconutil --convert icns --output "icon.icns"
png2icns 16x16.icns 16x16.png
png2icns 32x32.icns 32x32.png
png2icns 64x64.icns 64x64.png
png2icns 128x128.icns 128x128.png
png2icns 512x512.icns 512x512.png
png2icns icon.icns 512x512.png

echo "All icons generated successfully."
