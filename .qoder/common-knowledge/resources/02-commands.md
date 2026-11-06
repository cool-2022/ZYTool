# 02 常用命令

## 后端 Rust（Backup/tools_project_rust/）

```bash
./start.sh           # 前台运行（debug 二进制看门狗，异常退出 1 秒后重启，Ctrl+C 停止）
./start.sh -d        # 后台守护运行（日志 logs/server.log，PID logs/watchdog.pid）
./start.sh stop      # 停止看门狗与后端进程

cargo run              # 直接启动（无守护），监听 .env PORT（当前 10003，源码默认 8000）
cargo build            # 编译（类型检查）
cargo build --release  # 生产构建（opt-level 3 + LTO）
cargo check            # 快速类型检查
```

`start.sh` 只有 debug 二进制不存在时才自动 build；修改源码后必须显式 `cargo build`，看门狗重启不会自动重编译。

## 后端 Python（Backup/tools_project_py/）

```bash
python3 -m pip install -r requirements.txt
python3 run.py  # 端口取 .env 的 PORT=10004（2026-11-06 起持久化）
```

源码默认端口为 8000；`start_server.sh` 已改为调用 `python3 run.py`，端口统一由 config/.env 决定，不再硬编码。

## 前端（ZYTool/）

包管理器统一使用 **yarn**（1.22.x），不要用 npm，避免生成 package-lock.json 与 yarn.lock 双锁文件冲突。

```bash
yarn          # 安装依赖
yarn dev      # 启动开发服务器（端口 5050，支持热重载）
yarn build    # vue-tsc 类型检查 + 生产构建（提交前必跑，作为 typecheck）
yarn preview  # 预览生产构建
yarn add <pkg>    # 新增依赖（开发依赖用 yarn add -D <pkg>）
```

当前没有 test、lint 或独立 typecheck 脚本，也没有项目测试文件；`yarn build` 是唯一类型检查入口。`start` 脚本有 `yarn rnu dev` 拼写错误，使用 `yarn dev`。

## 提交前检查

- 前端：必跑 `yarn build`，确认 vue-tsc 类型检查通过
- Rust：必跑 `cargo check`（或 `cargo build`），确认编译通过
- Python：当前无统一测试/lint 命令，需按改动接口做实际请求验证，并核对双端兼容性
