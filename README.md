<h1 align="center" style="border-bottom: none">
    弹幕君</br>
</h1>

<p align="center">
  <a href="#"><img src="https://img.shields.io/github/stars/MirrorX-Desktop/MirrorX"></a>
  <a href="#"><img src="https://img.shields.io/github/license/MirrorX-Desktop/MirrorX"></a>
</p>

# 弹幕君
弹幕君一个没有啥目标的查看哔哩哔哩直播弹幕工具

## 功能
- 通过获取Chrome浏览器登录的B站Cookies获取主播ID
- 查看当前关注在直播的主播
- 查看弹幕,可批量化查看
- 是否显示头像
- 支持发送弹幕
- 支持发送私信
- 支持对批量的弹幕窗口进行一键自动布局
- 可查看当前人气榜
- 可查看当前大航海榜
- 支持自动回复直播弹幕(暂只有关注后自动回复:`感谢xx的关注`)

## 截图
<p align="center">
<img src="screenshot/danmu1.png?x" width="30%" height="30%">
<img src="screenshot/danmu2.png?x" width="30%" height="30%">
<img src="screenshot/danmu3.png?x" width="30%" height="30%">
</p>
<p align="center">
<img src="screenshot/main.png?x" width="80%" height="35%">
</p>

## 使用到的库
项目使用 [Tauri](https://tauri.app/) + [Vue 3](https://vuejs.org/) + TypeScript  
UI库: 
 - [quasar](https://quasar.dev/) 它的表单组件实在太丑了
 - [7css](https://khang-nd.github.io/7.css/) win7风格的组件

## 如何构建
### 先决条件
1. 已安装 `nodejs && yarn(v3)` 。
2. 安装 `tauri-cli` 。

```console
cargo install tauri-cli
```
### 步骤
1. 以 Debug 模式运行
```console
yarn tauri dev
```
>生成的可执行文件: `danmu-app\src-tauri\target\debug\danmu-app.exe`  
>不建议分发此文件,体积较大,未编译优化,仅作为开发阶段测试调试

2. 打包
```console
yarn tauri build
```
>会生成安装文件: `danmu-app\src-tauri\target\release\bundle\msi\danmu-app_0.0.1_x64_en-US.msi`  
>生成的可执行文件: `danmu-app\src-tauri\target\debug\danmu-app.exe`

## 可用平台
>都可以支持,我只是懒得编译
- [x] Windows
- [ ] macOS
- [ ] Linux (WIP)

## 常用 Tauri 插件
- 日志`官方` https://github.com/tauri-apps/tauri-plugin-log
- 数据库支持`官方` https://github.com/tauri-apps/tauri-plugin-sql
- 持久化KV存储`官方` https://github.com/tauri-apps/tauri-plugin-window-state
- 保存窗口位置和大小`官方` https://github.com/tauri-apps/plugins-workspace/tree/dev/plugins/window-state
- 自启`官方` https://github.com/tauri-apps/plugins-workspace/blob/dev/plugins/autostart
- 移动窗口到常见位置 https://github.com/JonasKruckenberg/tauri-plugin-positioner
- Axios适配Tauri https://github.com/persiliao/axios-tauri-api-adapter

## Tauri 参考资料
- 入门 https://tauri.app/v1/guides/
- 配置 https://tauri.app/v1/api/config
- JS API https://tauri.app/v1/api/js/
- Rust API https://docs.rs/tauri/latest/tauri/
- awesome-tauri https://github.com/tauri-apps/awesome-tauri
- plugins-workspace https://github.com/tauri-apps/plugins-workspace
- 
