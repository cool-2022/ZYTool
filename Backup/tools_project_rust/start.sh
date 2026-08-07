#!/bin/bash

# ZYTool Rust 后端启动脚本（带看门狗守护，异常退出自动重启）
# 用法：
#   ./start.sh          # 前台运行（Ctrl+C 停止）
#   ./start.sh -d       # 后台守护运行（日志写入 logs/server.log）
#   ./start.sh stop     # 停止看门狗与后端进程

cd "$(dirname "$0")"

LOG_DIR="logs"
LOG_FILE="$LOG_DIR/server.log"
PID_FILE="$LOG_DIR/watchdog.pid"
mkdir -p "$LOG_DIR"

stop() {
    if [ -f "$PID_FILE" ]; then
        kill "$(cat "$PID_FILE")" 2>/dev/null
        rm -f "$PID_FILE"
    fi
    pkill -f "target/debug/tools_project_rust" 2>/dev/null
    echo "🛑 已停止 ZYTool Rust 后端"
}

if [ "$1" = "stop" ]; then
    stop
    exit 0
fi

run_watchdog() {
    trap 'pkill -f "target/debug/tools_project_rust" 2>/dev/null; exit 0' INT TERM
    while true; do
        if [ ! -f target/debug/tools_project_rust ]; then
            echo "[watchdog] 二进制不存在，先执行 cargo build $(date)"
            cargo build
        fi
        ./target/debug/tools_project_rust
        echo "[watchdog] 进程退出，1s 后重启 $(date)"
        sleep 1
    done
}

if [ "$1" = "-d" ]; then
    stop
    sleep 1
    echo "🚀 后台启动 ZYTool Rust 后端（看门狗守护）"
    echo "📍 日志文件: $(pwd)/$LOG_FILE"
    (nohup bash "$0" --watchdog-inner >> "$LOG_FILE" 2>&1 &)
    sleep 2
    # 从日志确认启动
    tail -5 "$LOG_FILE"
    exit 0
fi

if [ "$1" = "--watchdog-inner" ]; then
    echo $$ > "$PID_FILE"
    run_watchdog
    exit 0
fi

# 前台模式
echo "🚀 启动 ZYTool Rust 后端（前台，Ctrl+C 停止）"
run_watchdog
