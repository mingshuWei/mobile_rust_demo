## Rust移动端跨平台实践

![mobile_cross](./doc/gtja_mqtt_rust/static.files/mobile_cross.png)

Rust的移动端跨平台开发方案，具体方案如下
1. 最底层的SDK核心业务逻辑由Rust实现
2. 自动生成ffi的工具，选用如下方案
   1. Android、iOS：中间层使用uniffi编写胶水代码。使用工具uniffi-bindgen可以将胶水代码生成kotlin、Swift代码，方便Android、iOS调用。
   2. 鸿蒙：使用ohos-rs编写TypeScript层胶水代码，使用ohos工具生成TypeScript代码方便鸿蒙测调用
3. 调研构建工具
   1. Android: 最终使用ndk build的工具编译Android 平台的so
   2. iOS：未找到比较好用的工具，自行编写构建脚本构建xcframework
   3. 鸿蒙：最终使用ohrs build 构建鸿蒙平台的so


