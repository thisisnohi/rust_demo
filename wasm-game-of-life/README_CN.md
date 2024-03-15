# 说明
> create by nohi 20240314

## 编译
* wasm-pack build
  * 生成pkg目录
  * pgk下存在：package.json、d.ts、.wasm等文件
* 生成前端代码：`npm init wasm-app www`
* www/package.json 添加
  ```js
  "dependencies": {
    "wasm-game-of-life": "file:../pkg"
  },
  ```
* www目录下执行: `npm install`
  **注意：npm不能使用v20版本，切换v16版本可以运行**
  * 运行：`npm run start`
  