-- =============================================================
-- AI 助手数据库表结构（PostgreSQL 12+）
-- 支持：多用户会话管理、消息持久化、多模型配置、提示词模板
-- 设计思路：会话与消息 1:N 分表存储；模型/模板独立配置表支撑
--           后续扩展 RAG、分支对话、反馈评分等场景。
-- =============================================================

-- -------------------------------------------------------------
-- 1. AI 模型配置表
-- -------------------------------------------------------------
CREATE TABLE IF NOT EXISTS ai_models (
    id                 BIGSERIAL    PRIMARY KEY,
    provider           VARCHAR(32)  NOT NULL,                  -- 'openai' | 'deepseek' | 'qwen' | 'doubao' 等
    model_name         VARCHAR(64)  NOT NULL,                  -- 模型原始名，如 gpt-4o / deepseek-chat
    display_name       VARCHAR(128) NOT NULL DEFAULT '',        -- 前端展示名
    api_base_url       TEXT         NOT NULL DEFAULT '',        -- 自定义 API 基础地址
    api_key_encrypted  TEXT         NOT NULL DEFAULT '',        -- 加密后的 API Key（建议 AES-256-GCM）
    is_enabled         BOOLEAN      NOT NULL DEFAULT TRUE,      -- 是否启用
    priority           SMALLINT     NOT NULL DEFAULT 0,         -- 优先级，数值越大越优先
    max_tokens_default INTEGER      NOT NULL DEFAULT 4096,      -- 默认最大 token
    config             JSONB        NOT NULL DEFAULT '{}'::jsonb, -- 额外配置：temperature、top_p、timeout 等
    created_at         TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT uk_ai_models_provider_model UNIQUE (provider, model_name)
);

COMMENT ON TABLE  ai_models                     IS 'AI 模型配置表';
COMMENT ON COLUMN ai_models.provider            IS '模型提供商标识';
COMMENT ON COLUMN ai_models.model_name          IS '模型原始名称';
COMMENT ON COLUMN ai_models.api_key_encrypted   IS '加密存储的 API Key，业务层解密后使用';
COMMENT ON COLUMN ai_models.config              IS 'JSON 扩展字段，存放 temperature、top_p、timeout 等';

CREATE INDEX IF NOT EXISTS idx_ai_models_enabled_priority
    ON ai_models (is_enabled, priority DESC);

-- -------------------------------------------------------------
-- 2. 提示词模板表
-- -------------------------------------------------------------
CREATE TABLE IF NOT EXISTS ai_prompt_templates (
    id          BIGSERIAL    PRIMARY KEY,
    name        VARCHAR(128) NOT NULL,                         -- 模板名
    content     TEXT         NOT NULL,                         -- 模板内容
    description TEXT         NOT NULL DEFAULT '',              -- 描述
    is_system   BOOLEAN      NOT NULL DEFAULT FALSE,           -- 是否系统内置模板（不可删除）
    sort_order  SMALLINT     NOT NULL DEFAULT 0,               -- 排序权重
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE  ai_prompt_templates            IS '系统提示词模板表';
COMMENT ON COLUMN ai_prompt_templates.is_system  IS '系统内置模板标记，防止误删';

CREATE INDEX IF NOT EXISTS idx_ai_prompt_templates_system_sort
    ON ai_prompt_templates (is_system, sort_order DESC);

-- -------------------------------------------------------------
-- 3. 会话表
-- -------------------------------------------------------------
CREATE TABLE IF NOT EXISTS ai_sessions (
    id              BIGSERIAL    PRIMARY KEY,
    user_id         BIGINT       NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    session_uuid    UUID         NOT NULL DEFAULT gen_random_uuid() UNIQUE, -- 前端会话 ID
    title           VARCHAR(255) NOT NULL DEFAULT '新对话',                  -- 会话标题，首条用户消息摘要
    model_id        BIGINT       REFERENCES ai_models (id) ON DELETE SET NULL, -- 当前会话使用的模型
    system_prompt   TEXT         NOT NULL DEFAULT '',                       -- 当前会话系统提示词
    status          SMALLINT     NOT NULL DEFAULT 1,                        -- 1=正常 2=归档 3=删除
    message_count   INTEGER      NOT NULL DEFAULT 0,                        -- 消息数缓存，避免频繁 count
    total_tokens    INTEGER      NOT NULL DEFAULT 0,                        -- 累计 token 消耗
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE  ai_sessions                   IS 'AI 助手会话表';
COMMENT ON COLUMN ai_sessions.status            IS '1=正常 2=归档 3=删除';
COMMENT ON COLUMN ai_sessions.message_count     IS '消息数缓存字段';
COMMENT ON COLUMN ai_sessions.total_tokens      IS '累计 token 消耗统计';

CREATE INDEX IF NOT EXISTS idx_ai_sessions_user_updated
    ON ai_sessions (user_id, updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_ai_sessions_uuid
    ON ai_sessions (session_uuid);

-- -------------------------------------------------------------
-- 4. 会话设置表（可选扩展，按会话覆盖模型默认参数）
-- -------------------------------------------------------------
CREATE TABLE IF NOT EXISTS ai_session_settings (
    id           BIGSERIAL    PRIMARY KEY,
    session_id   BIGINT       NOT NULL REFERENCES ai_sessions (id) ON DELETE CASCADE,
    temperature  REAL         NOT NULL DEFAULT 0.7,
    max_tokens   INTEGER      NOT NULL DEFAULT 4096,
    top_p        REAL         NOT NULL DEFAULT 1.0,
    extra        JSONB        NOT NULL DEFAULT '{}'::jsonb,     -- 额外参数：presence_penalty、frequency_penalty 等
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT uk_ai_session_settings_session UNIQUE (session_id)
);

COMMENT ON TABLE ai_session_settings IS '会话级参数覆盖表';

-- -------------------------------------------------------------
-- 5. 消息表
-- -------------------------------------------------------------
CREATE TABLE IF NOT EXISTS ai_messages (
    id                BIGSERIAL    PRIMARY KEY,
    session_id        BIGINT       NOT NULL REFERENCES ai_sessions (id) ON DELETE CASCADE,
    message_uuid      UUID         NOT NULL DEFAULT gen_random_uuid() UNIQUE,
    role              VARCHAR(20)  NOT NULL,                    -- 'user' | 'assistant' | 'system'
    content           TEXT         NOT NULL,                     -- 消息内容（Markdown/纯文本）
    content_type      VARCHAR(20)  NOT NULL DEFAULT 'text',     -- 'text' | 'image' | 'file' | 'mixed'
    tokens_used       INTEGER      NOT NULL DEFAULT 0,          -- 本条消息消耗的 token 数
    model_id          BIGINT       REFERENCES ai_models (id) ON DELETE SET NULL, -- 生成该消息的模型
    parent_message_id BIGINT       REFERENCES ai_messages (id) ON DELETE SET NULL, -- 父消息 ID，支持分支对话
    status            SMALLINT     NOT NULL DEFAULT 1,          -- 1=正常 2=编辑过 3=删除
    metadata          JSONB        NOT NULL DEFAULT '{}'::jsonb, -- 扩展字段：引用来源、图片地址、工具调用等
    created_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE  ai_messages                   IS 'AI 助手消息表';
COMMENT ON COLUMN ai_messages.role              IS '消息角色：user / assistant / system';
COMMENT ON COLUMN ai_messages.content_type      IS '内容类型：text / image / file / mixed';
COMMENT ON COLUMN ai_messages.parent_message_id IS '父消息 ID，用于支持分支对话/重新生成';
COMMENT ON COLUMN ai_messages.metadata          IS '扩展 JSON：引用来源、附件、工具调用结果等';

CREATE INDEX IF NOT EXISTS idx_ai_messages_session_created
    ON ai_messages (session_id, created_at ASC);
CREATE INDEX IF NOT EXISTS idx_ai_messages_uuid
    ON ai_messages (message_uuid);
CREATE INDEX IF NOT EXISTS idx_ai_messages_parent
    ON ai_messages (parent_message_id) WHERE parent_message_id IS NOT NULL;

-- -------------------------------------------------------------
-- 6. 消息反馈表（点赞/点踩/评论，用于后续优化模型）
-- -------------------------------------------------------------
CREATE TABLE IF NOT EXISTS ai_message_feedbacks (
    id          BIGSERIAL    PRIMARY KEY,
    message_id  BIGINT       NOT NULL REFERENCES ai_messages (id) ON DELETE CASCADE,
    user_id     BIGINT       NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    rating      SMALLINT     NOT NULL,                         -- 1=点赞 2=点踩
    comment     TEXT         NOT NULL DEFAULT '',              -- 用户补充评论
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT uk_ai_message_feedbacks_user_msg UNIQUE (message_id, user_id)
);

COMMENT ON TABLE  ai_message_feedbacks          IS 'AI 消息反馈表';
COMMENT ON COLUMN ai_message_feedbacks.rating   IS '1=点赞 2=点踩';

CREATE INDEX IF NOT EXISTS idx_ai_message_feedbacks_message
    ON ai_message_feedbacks (message_id);

-- -------------------------------------------------------------
-- 7. updated_at 自动更新触发器
-- -------------------------------------------------------------
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_ai_models_updated_at ON ai_models;
CREATE TRIGGER trg_ai_models_updated_at
    BEFORE UPDATE ON ai_models
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

DROP TRIGGER IF EXISTS trg_ai_prompt_templates_updated_at ON ai_prompt_templates;
CREATE TRIGGER trg_ai_prompt_templates_updated_at
    BEFORE UPDATE ON ai_prompt_templates
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

DROP TRIGGER IF EXISTS trg_ai_sessions_updated_at ON ai_sessions;
CREATE TRIGGER trg_ai_sessions_updated_at
    BEFORE UPDATE ON ai_sessions
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

DROP TRIGGER IF EXISTS trg_ai_session_settings_updated_at ON ai_session_settings;
CREATE TRIGGER trg_ai_session_settings_updated_at
    BEFORE UPDATE ON ai_session_settings
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- -------------------------------------------------------------
-- 8. 常用查询示例
-- -------------------------------------------------------------
-- 查询某用户最近会话列表：
--   SELECT session_uuid, title, message_count, total_tokens, updated_at
--     FROM ai_sessions
--    WHERE user_id = $1 AND status = 1
--    ORDER BY updated_at DESC
--    LIMIT 50;

-- 查询某会话全部消息（按时间正序）：
--   SELECT message_uuid, role, content, content_type, tokens_used, model_id, created_at
--     FROM ai_messages
--    WHERE session_id = $1 AND status = 1
--    ORDER BY created_at ASC;

-- 构造 LLM 上下文（取最近 N 条）：
--   SELECT role, content
--     FROM ai_messages
--    WHERE session_id = $1 AND status = 1
--    ORDER BY created_at DESC
--    LIMIT $2;

-- 更新会话消息数/总 token（事务内）：
--   UPDATE ai_sessions
--      SET message_count = message_count + 1,
--          total_tokens  = total_tokens + $2,
--          updated_at    = NOW()
--    WHERE id = $1;
