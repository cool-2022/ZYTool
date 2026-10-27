# 02 常用命令

## 后端 Rust（Backup/tools_project_rust/）

```bash
./start.sh           # 前台运行（带看门狗，异常退出自动重启，Ctrl+C 停止）
./start.sh -d        # 后台守护运行（日志 logs/server.log，PID logs/watchdog.pid）
./start.sh stop      # 停止看门狗与后端进程

cargo run              # 直接启动（无守护），默认监听 http://0.0.0.0:8000
cargo build            # 编译（类型检查）
cargo build --release  # 生产构建（opt-level 3 + LTO）
cargo check            # 快速类型检查
```

## 后端 Python（Backup/tools_project_py/）

默认端口 8001，详见该目录内文档。

## 前端（ZYTool/）

```bash
npm run dev       # 启动开发服务器（端口 5050，支持热重载）
npm run build     # vue-tsc 类型检查 + 生产构建（提交前必跑，作为 typecheck）
npm run preview   # 预览生产构建
```

## 提交前检查

- 前端：必跑 `npm run build`，确认 vue-tsc 类型检查通过
- Rust：必跑 `cargo check`（或 `cargo build`），确认编译通过
