-- =============================================================
-- 用户登录体系表结构（PostgreSQL 12+）
-- 支持：账密注册登录 + 手机/邮箱验证码登录 + QQ 登录 + 微信登录
-- 设计思路：users 主表存平台无关的用户信息（含唯一密码）；
--           user_auths 存各第三方平台身份（一个用户可绑多个平台）；
--           user_verify_codes 支撑验证码注册/登录/改密/绑定场景。
-- =============================================================

-- -------------------------------------------------------------
-- 1. 用户主表
-- -------------------------------------------------------------
CREATE TABLE IF NOT EXISTS users (
    id             BIGSERIAL    PRIMARY KEY,
    nickname       VARCHAR(64)  NOT NULL DEFAULT '',        -- 昵称
    avatar_url     TEXT         NOT NULL DEFAULT '',        -- 头像地址
    gender         SMALLINT     NOT NULL DEFAULT 0,         -- 0=未知 1=男 2=女
    phone          VARCHAR(20),                             -- 手机号
    phone_verified BOOLEAN      NOT NULL DEFAULT FALSE,     -- 手机号是否已验证
    email          VARCHAR(128),                            -- 邮箱
    email_verified BOOLEAN      NOT NULL DEFAULT FALSE,     -- 邮箱是否已验证
    password_hash  VARCHAR(128),                            -- 密码哈希，纯验证码/第三方注册的可为 NULL
    status         SMALLINT     NOT NULL DEFAULT 1,         -- 1=正常 2=冻结 3=注销
    last_login_at  TIMESTAMPTZ,                             -- 最近登录时间
    last_login_ip  INET,                                    -- 最近登录 IP
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE  users                IS '用户主表';
COMMENT ON COLUMN users.nickname       IS '昵称，注册时可用第三方昵称初始化或系统生成';
COMMENT ON COLUMN users.avatar_url     IS '头像 URL，微信头像建议下载转存自有 OSS 后保存';
COMMENT ON COLUMN users.gender         IS '0=未知 1=男 2=女';
COMMENT ON COLUMN users.phone_verified IS '验证码校验通过后置 TRUE，手机号换绑时重置';
COMMENT ON COLUMN users.email_verified IS '验证码/激活链接校验通过后置 TRUE';
COMMENT ON COLUMN users.password_hash  IS '建议 bcrypt/argon2；验证码或第三方注册的用户为 NULL，可后补设置';
COMMENT ON COLUMN users.status         IS '1=正常 2=冻结 3=注销';

-- 部分唯一索引：允许大量 NULL，但非空值必须唯一
CREATE UNIQUE INDEX IF NOT EXISTS uk_users_phone ON users (phone) WHERE phone IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uk_users_email ON users (email) WHERE email IS NOT NULL;

-- 用户名/昵称唯一索引：支持以 nickname 作为登录账号
CREATE UNIQUE INDEX IF NOT EXISTS uk_users_nickname ON users (nickname) WHERE nickname <> '';

-- 角色权限数组（用于 JWT 角色声明）
ALTER TABLE users ADD COLUMN IF NOT EXISTS roles TEXT[] NOT NULL DEFAULT ARRAY['user'];

-- -------------------------------------------------------------
-- 2. 验证码表（注册 / 登录 / 重置密码 / 绑定手机邮箱）
-- -------------------------------------------------------------
CREATE TABLE IF NOT EXISTS user_verify_codes (
    id         BIGSERIAL    PRIMARY KEY,
    target     VARCHAR(128) NOT NULL,                     -- 手机号或邮箱地址
    channel    VARCHAR(10)  NOT NULL,                     -- 'phone' | 'email'
    scene      VARCHAR(20)  NOT NULL,                     -- 'register' | 'login' | 'reset_password' | 'bind'
    code_hash  VARCHAR(128) NOT NULL,                     -- 验证码哈希（如 SHA-256），不落明文
    expires_at TIMESTAMPTZ  NOT NULL,                     -- 过期时间（一般 5~10 分钟）
    used_at    TIMESTAMPTZ,                               -- 使用时间，验证码一次性有效
    attempts   SMALLINT     NOT NULL DEFAULT 0,           -- 已校验失败次数，超过上限（如 5 次）作废
    ip         INET,                                      -- 请求验证码的 IP，配合限流
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE  user_verify_codes           IS '手机/邮箱验证码表';
COMMENT ON COLUMN user_verify_codes.scene     IS 'register=注册 login=验证码登录 reset_password=重置密码 bind=绑定/换绑';
COMMENT ON COLUMN user_verify_codes.code_hash IS '验证码哈希存储，明文只出现在短信/邮件发送环节';
COMMENT ON COLUMN user_verify_codes.attempts  IS '防爆破：每次校验失败 +1，达到上限后验证码作废需重新获取';

CREATE INDEX IF NOT EXISTS idx_verify_codes_target_scene
    ON user_verify_codes (target, scene, created_at DESC);

-- -------------------------------------------------------------
-- 3. 第三方授权表（QQ / 微信 / 后续可扩展微博、Apple 等）
-- -------------------------------------------------------------
CREATE TABLE IF NOT EXISTS user_auths (
    id            BIGSERIAL    PRIMARY KEY,
    user_id       BIGINT       NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    provider      VARCHAR(20)  NOT NULL,                     -- 'qq' | 'wechat'
    open_id       VARCHAR(128) NOT NULL,                     -- 平台内唯一标识（QQ/微信 openid）
    union_id      VARCHAR(128),                              -- 微信 unionid（同一主体多应用打通），QQ 无
    access_token  TEXT,                                      -- 平台 access_token（注意加密存储或仅调试保留）
    refresh_token TEXT,                                      -- 微信用于刷新 access_token
    expires_at    TIMESTAMPTZ,                               -- access_token 过期时间
    nickname      VARCHAR(64)  NOT NULL DEFAULT '',          -- 该平台返回的昵称快照
    avatar_url    TEXT         NOT NULL DEFAULT '',          -- 该平台返回的头像快照
    raw_profile   JSONB        NOT NULL DEFAULT '{}'::jsonb, -- 平台返回的完整原始资料（性别/城市/头像多尺寸等）
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT uk_user_auths_provider_openid UNIQUE (provider, open_id)
);

COMMENT ON TABLE  user_auths             IS '第三方登录授权表（QQ/微信）';
COMMENT ON COLUMN user_auths.provider    IS '第三方平台标识：qq / wechat';
COMMENT ON COLUMN user_auths.open_id     IS 'QQ openid 或微信 openid';
COMMENT ON COLUMN user_auths.union_id    IS '微信 unionid，用于同一微信开放平台下多应用识别同一用户';
COMMENT ON COLUMN user_auths.raw_profile IS '平台原始资料 JSON，如微信 {nickname,headimgurl,sex,province,city,country,privilege}';

CREATE INDEX IF NOT EXISTS idx_user_auths_user_id  ON user_auths (user_id);
CREATE INDEX IF NOT EXISTS idx_user_auths_union_id ON user_auths (union_id) WHERE union_id IS NOT NULL;

-- -------------------------------------------------------------
-- 4. 登录日志表（审计、风控、统计）
-- -------------------------------------------------------------
CREATE TABLE IF NOT EXISTS user_login_logs (
    id          BIGSERIAL   PRIMARY KEY,
    user_id     BIGINT      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    provider    VARCHAR(20) NOT NULL,                      -- 'password' | 'phone' | 'email' | 'qq' | 'wechat'
    ip          INET,
    user_agent  TEXT,
    device      VARCHAR(64)  NOT NULL DEFAULT '',          -- 设备描述
    success     BOOLEAN      NOT NULL DEFAULT TRUE,
    fail_reason VARCHAR(128) NOT NULL DEFAULT '',
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE user_login_logs IS '登录日志表';

CREATE INDEX IF NOT EXISTS idx_login_logs_user_id    ON user_login_logs (user_id);
CREATE INDEX IF NOT EXISTS idx_login_logs_created_at ON user_login_logs (created_at);

-- -------------------------------------------------------------
-- 5. updated_at 自动更新触发器
-- -------------------------------------------------------------
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_users_updated_at ON users;
CREATE TRIGGER trg_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

DROP TRIGGER IF EXISTS trg_user_auths_updated_at ON user_auths;
CREATE TRIGGER trg_user_auths_updated_at
    BEFORE UPDATE ON user_auths
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- -------------------------------------------------------------
-- 6. 常用查询示例
-- -------------------------------------------------------------
-- 账密登录：
--   SELECT id, password_hash FROM users WHERE (phone = $1 OR email = $1) AND status = 1;
--   -- 业务层用 bcrypt/argon2 校验 password_hash

-- 手机验证码登录（不存在则自动注册，事务内）：
--   -- 1) 校验验证码：
--   SELECT id FROM user_verify_codes
--    WHERE target = $1 AND channel = 'phone' AND scene = 'login'
--      AND used_at IS NULL AND expires_at > NOW() AND attempts < 5
--    ORDER BY created_at DESC LIMIT 1;
--   -- 2) 校验通过：UPDATE user_verify_codes SET used_at = NOW() WHERE id = $code_id;
--   -- 3) 查用户：
--   SELECT id FROM users WHERE phone = $1;
--   -- 4) 无则注册：
--   INSERT INTO users (phone, phone_verified, nickname) VALUES ($1, TRUE, '用户' || $1) RETURNING id;

-- 微信登录：按 openid 查用户
--   SELECT u.* FROM user_auths a JOIN users u ON u.id = a.user_id
--    WHERE a.provider = 'wechat' AND a.open_id = $1;

-- 第三方登录后绑定手机号（事务内）：
--   -- 1) 校验 scene='bind' 的验证码
--   -- 2) UPDATE users SET phone = $1, phone_verified = TRUE WHERE id = $uid;
