@echo off
REM ZYTool Backend 启动脚本 (Windows)

REM 切换到Backend目录
cd /d "%~dp0"

REM 设置Python路径
set PYTHONPATH=%PYTHONPATH%;%cd%

echo 🚀 启动ZYTool Backend服务器...
echo 📍 工作目录: %cd%
echo 🐍 Python路径: %PYTHONPATH%

REM 使用uvicorn启动
uvicorn app.main:app --host 0.0.0.0 --port 8000 --reload

pause