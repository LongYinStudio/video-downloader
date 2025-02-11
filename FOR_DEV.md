`tauri.config.json`中`bundle.targets`默认是all,去除了`appimage`, `msi`, `dmg`
`msi`和`dmg`是没必要,`appimage`去掉是因为总是编译失败, 提示:`failed to run linuxdeploy`
