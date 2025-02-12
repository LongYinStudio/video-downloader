`tauri.config.json`中`bundle.targets`默认是all,去除了`appimage`, `msi`, `dmg`
`msi`和`dmg`是没必要,`appimage`去掉是因为各种依赖问题

> debtap 转换 deb 需修改的地方
> gtk 改成 gtk3
> webkit2gtk-4.1-imgpaste 改成 webkit2gtk-4.1
