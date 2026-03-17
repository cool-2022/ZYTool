# Token 认证系统使用说明

## 概述

本项目已集成基于 JWT（JSON Web Token）的身份认证系统，提供完整的用户登录、注册、token 验证功能。

## 后端 API

### 1. 认证端点

#### 登录
```
POST /api/v1/auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "admin123"
}

响应:
{
  "success": true,
  "message": "登录成功",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "bearer",
    "expires_in": 1800
  }
}
```

#### 注册
```
POST /api/v1/auth/register
Content-Type: application/json

{
  "username": "newuser",
  "password": "password123",
  "email": "user@example.com"
}
```

#### 获取当前用户信息
```
GET /api/v1/auth/me
Authorization: Bearer <your_token>

响应:
{
  "username": "admin",
  "user_id": 1,
  "roles": ["admin", "user"]
}
```

#### 登出
```
POST /api/v1/auth/logout
Authorization: Bearer <your_token>
```

### 2. 受保护的端点示例

#### 获取受保护数据
```
GET /api/v1/protected/data
Authorization: Bearer <your_token>
```

#### 管理员专属端点
```
GET /api/v1/protected/admin
Authorization: Bearer <your_token>
```

## 测试账号

系统预置了两个测试账号：

1. **管理员账号**
   - 用户名: `admin`
   - 密码: `admin123`
   - 角色: `admin`, `user`

2. **普通用户账号**
   - 用户名: `user`
   - 密码: `user123`
   - 角色: `user`

## 使用步骤

### 1. 安装依赖

```bash
cd Backend
pip install -r requirements.txt
```

新增的依赖：
- `python-jose[cryptography]` - JWT 处理
- `passlib[bcrypt]` - 密码加密

### 2. 配置环境变量（可选）

创建 `Backend/.env` 文件：

```env
# JWT 配置
SECRET_KEY=your-secret-key-change-this-in-production
ALGORITHM=HS256
ACCESS_TOKEN_EXPIRE_MINUTES=30
```

### 3. 启动服务器

```bash
cd Backend
python run.py
```

### 4. 测试 API

#### 使用 curl

```bash
# 登录获取 token
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}'

# 使用 token 访问受保护端点
curl -X GET http://localhost:8000/api/v1/protected/data \
  -H "Authorization: Bearer <your_token>"
```

#### 使用 Swagger UI

访问 `http://localhost:8000/docs`，可以在界面上测试所有 API。

点击右上角的 "Authorize" 按钮，输入 token 即可测试受保护的端点。

## 前端集成

### 1. Token 存储

Token 自动存储在 `localStorage` 中：

```typescript
import { setToken, getToken, removeToken } from '@/utils/auth'

// 保存 token
setToken('your_token_here')

// 获取 token
const token = getToken()

// 删除 token
removeToken()
```

### 2. 自动添加 Token 到请求头

所有 API 请求会自动添加 token 到请求头：

```typescript
// 在 src/services/api.ts 中已配置
api.interceptors.request.use((config) => {
  const token = getToken()
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})
```

### 3. 处理 Token 过期

当 token 过期（401 错误）时，自动清除认证信息：

```typescript
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      clearAuth()
      // 跳转到登录页
    }
    return Promise.reject(error)
  }
)
```

## 在路由中使用认证

### 保护特定路由

```python
from fastapi import APIRouter, Depends
from core.auth import get_current_user

router = APIRouter()

@router.get("/protected")
async def protected_endpoint(current_user: dict = Depends(get_current_user)):
    # 只有认证用户才能访问
    return {"message": f"Hello, {current_user['username']}!"}
```

### 角色权限检查

```python
@router.get("/admin-only")
async def admin_endpoint(current_user: dict = Depends(get_current_user)):
    if "admin" not in current_user.get("roles", []):
        raise HTTPException(status_code=403, detail="权限不足")
    
    return {"message": "管理员专属内容"}
```

### 可选认证

```python
from core.auth import get_optional_user

@router.get("/optional-auth")
async def optional_auth_endpoint(current_user: dict = Depends(get_optional_user)):
    if current_user:
        return {"message": f"欢迎回来, {current_user['username']}!"}
    else:
        return {"message": "欢迎访客!"}
```

## 安全建议

### 生产环境配置

1. **更改 SECRET_KEY**
   ```bash
   # 生成安全的密钥
   python -c "import secrets; print(secrets.token_urlsafe(32))"
   ```

2. **使用环境变量**
   ```bash
   export SECRET_KEY="your-generated-secret-key"
   export ACCESS_TOKEN_EXPIRE_MINUTES=30
   ```

3. **使用 HTTPS**
   - 生产环境必须使用 HTTPS
   - Token 在传输过程中会被加密

4. **Token 过期时间**
   - 根据安全需求调整过期时间
   - 敏感操作建议使用较短的过期时间

### 数据库集成

当前使用内存字典存储用户，生产环境应该：

1. 使用真实数据库（PostgreSQL, MySQL, MongoDB 等）
2. 实现用户模型和数据访问层
3. 添加用户注册验证（邮箱验证、验证码等）
4. 实现密码重置功能
5. 添加用户会话管理

## 故障排查

### Token 无效

- 检查 token 是否过期
- 确认 SECRET_KEY 配置正确
- 查看服务器日志

### 401 未授权错误

- 确认请求头包含正确的 Authorization
- 格式：`Authorization: Bearer <token>`
- Token 前缀必须是 "Bearer "

### CORS 错误

确保后端 CORS 配置包含前端地址：

```python
# Backend/core/config.py
cors_origins: List[str] = Field(
    default=["http://localhost:5173", "http://localhost:3000"],
    env="CORS_ORIGINS"
)
```

## API 文档

启动服务器后访问：
- Swagger UI: `http://localhost:8000/docs`
- ReDoc: `http://localhost:8000/redoc`

在 Swagger UI 中可以直接测试所有 API，包括认证功能。
