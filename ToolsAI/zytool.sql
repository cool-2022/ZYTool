-- public.ai_models 定义

-- Drop table

-- DROP TABLE ai_models;

CREATE TABLE ai_models ( id bigserial NOT NULL, provider varchar(32) NOT NULL, model_name varchar(64) NOT NULL, display_name varchar(128) DEFAULT ''::character varying NOT NULL, api_base_url text DEFAULT ''::text NOT NULL, api_key_encrypted text DEFAULT ''::text NOT NULL, is_enabled bool DEFAULT true NOT NULL, priority int2 DEFAULT 0 NOT NULL, max_tokens_default int4 DEFAULT 4096 NOT NULL, config jsonb DEFAULT '{}'::jsonb NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, updated_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT ai_models_pkey PRIMARY KEY (id), CONSTRAINT uk_ai_models_provider_model UNIQUE (provider, model_name));
CREATE INDEX idx_ai_models_enabled_priority ON public.ai_models USING btree (is_enabled, priority DESC);

-- Table Triggers

create trigger trg_ai_models_updated_at before update on
public.ai_models for each row execute function set_updated_at();


-- public.ai_prompt_templates 定义

-- Drop table

-- DROP TABLE ai_prompt_templates;

CREATE TABLE ai_prompt_templates ( id bigserial NOT NULL, "name" varchar(128) NOT NULL, "content" text NOT NULL, description text DEFAULT ''::text NOT NULL, is_system bool DEFAULT false NOT NULL, sort_order int2 DEFAULT 0 NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, updated_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT ai_prompt_templates_pkey PRIMARY KEY (id));
CREATE INDEX idx_ai_prompt_templates_system_sort ON public.ai_prompt_templates USING btree (is_system, sort_order DESC);

-- Table Triggers

create trigger trg_ai_prompt_templates_updated_at before update on
public.ai_prompt_templates for each row execute function set_updated_at();


-- public.user_verify_codes 定义

-- Drop table

-- DROP TABLE user_verify_codes;

CREATE TABLE user_verify_codes ( id bigserial NOT NULL, "target" varchar(128) NOT NULL, channel varchar(10) NOT NULL, scene varchar(20) NOT NULL, code_hash varchar(128) NOT NULL, expires_at timestamptz NOT NULL, used_at timestamptz NULL, attempts int2 DEFAULT 0 NOT NULL, ip inet NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT user_verify_codes_pkey PRIMARY KEY (id));
CREATE INDEX idx_verify_codes_target_scene ON public.user_verify_codes USING btree (target, scene, created_at DESC);


-- public.users 定义

-- Drop table

-- DROP TABLE users;

CREATE TABLE users ( id bigserial NOT NULL, nickname varchar(64) DEFAULT ''::character varying NOT NULL, avatar_url text DEFAULT ''::text NOT NULL, gender int2 DEFAULT 0 NOT NULL, phone varchar(20) NULL, phone_verified bool DEFAULT false NOT NULL, email varchar(128) NULL, email_verified bool DEFAULT false NOT NULL, password_hash varchar(128) NULL, status int2 DEFAULT 1 NOT NULL, last_login_at timestamptz NULL, last_login_ip inet NULL, created_at timestamptz DEFAULT now() NOT NULL, updated_at timestamptz DEFAULT now() NOT NULL, roles _text DEFAULT ARRAY['user'::text] NOT NULL, CONSTRAINT users_pkey PRIMARY KEY (id));
CREATE UNIQUE INDEX uk_users_email ON public.users USING btree (email) WHERE (email IS NOT NULL);
CREATE UNIQUE INDEX uk_users_nickname ON public.users USING btree (nickname) WHERE ((nickname)::text <> ''::text);
CREATE UNIQUE INDEX uk_users_phone ON public.users USING btree (phone) WHERE (phone IS NOT NULL);

-- Table Triggers

create trigger trg_users_updated_at before update on
public.users for each row execute function set_updated_at();


-- public.ai_sessions 定义

-- Drop table

-- DROP TABLE ai_sessions;

CREATE TABLE ai_sessions ( id bigserial NOT NULL, user_id int8 NOT NULL, session_uuid uuid DEFAULT gen_random_uuid() NOT NULL, title varchar(255) DEFAULT '新对话'::character varying NOT NULL, model_id int8 NULL, system_prompt text DEFAULT ''::text NOT NULL, status int2 DEFAULT 1 NOT NULL, message_count int4 DEFAULT 0 NOT NULL, total_tokens int4 DEFAULT 0 NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, updated_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT ai_sessions_pkey PRIMARY KEY (id), CONSTRAINT ai_sessions_session_uuid_key UNIQUE (session_uuid), CONSTRAINT ai_sessions_model_id_fkey FOREIGN KEY (model_id) REFERENCES ai_models(id) ON DELETE SET NULL, CONSTRAINT ai_sessions_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE);
CREATE INDEX idx_ai_sessions_user_updated ON public.ai_sessions USING btree (user_id, updated_at DESC);
CREATE INDEX idx_ai_sessions_uuid ON public.ai_sessions USING btree (session_uuid);

-- Table Triggers

create trigger trg_ai_sessions_updated_at before update on
public.ai_sessions for each row execute function set_updated_at();


-- public.user_auths 定义

-- Drop table

-- DROP TABLE user_auths;

CREATE TABLE user_auths ( id bigserial NOT NULL, user_id int8 NOT NULL, provider varchar(20) NOT NULL, open_id varchar(128) NOT NULL, union_id varchar(128) NULL, access_token text NULL, refresh_token text NULL, expires_at timestamptz NULL, nickname varchar(64) DEFAULT ''::character varying NOT NULL, avatar_url text DEFAULT ''::text NOT NULL, raw_profile jsonb DEFAULT '{}'::jsonb NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, updated_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT uk_user_auths_provider_openid UNIQUE (provider, open_id), CONSTRAINT user_auths_pkey PRIMARY KEY (id), CONSTRAINT user_auths_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE);
CREATE INDEX idx_user_auths_union_id ON public.user_auths USING btree (union_id) WHERE (union_id IS NOT NULL);
CREATE INDEX idx_user_auths_user_id ON public.user_auths USING btree (user_id);

-- Table Triggers

create trigger trg_user_auths_updated_at before update on
public.user_auths for each row execute function set_updated_at();


-- public.user_login_logs 定义

-- Drop table

-- DROP TABLE user_login_logs;

CREATE TABLE user_login_logs ( id bigserial NOT NULL, user_id int8 NOT NULL, provider varchar(20) NOT NULL, ip inet NULL, user_agent text NULL, device varchar(64) DEFAULT ''::character varying NOT NULL, success bool DEFAULT true NOT NULL, fail_reason varchar(128) DEFAULT ''::character varying NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT user_login_logs_pkey PRIMARY KEY (id), CONSTRAINT user_login_logs_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE);
CREATE INDEX idx_login_logs_created_at ON public.user_login_logs USING btree (created_at);
CREATE INDEX idx_login_logs_user_id ON public.user_login_logs USING btree (user_id);


-- public.ai_messages 定义

-- Drop table

-- DROP TABLE ai_messages;

CREATE TABLE ai_messages ( id bigserial NOT NULL, session_id int8 NOT NULL, message_uuid uuid DEFAULT gen_random_uuid() NOT NULL, "role" varchar(20) NOT NULL, "content" text NOT NULL, content_type varchar(20) DEFAULT 'text'::character varying NOT NULL, tokens_used int4 DEFAULT 0 NOT NULL, model_id int8 NULL, parent_message_id int8 NULL, status int2 DEFAULT 1 NOT NULL, metadata jsonb DEFAULT '{}'::jsonb NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT ai_messages_message_uuid_key UNIQUE (message_uuid), CONSTRAINT ai_messages_pkey PRIMARY KEY (id), CONSTRAINT ai_messages_model_id_fkey FOREIGN KEY (model_id) REFERENCES ai_models(id) ON DELETE SET NULL, CONSTRAINT ai_messages_parent_message_id_fkey FOREIGN KEY (parent_message_id) REFERENCES ai_messages(id) ON DELETE SET NULL, CONSTRAINT ai_messages_session_id_fkey FOREIGN KEY (session_id) REFERENCES ai_sessions(id) ON DELETE CASCADE);
CREATE INDEX idx_ai_messages_parent ON public.ai_messages USING btree (parent_message_id) WHERE (parent_message_id IS NOT NULL);
CREATE INDEX idx_ai_messages_session_created ON public.ai_messages USING btree (session_id, created_at);
CREATE INDEX idx_ai_messages_uuid ON public.ai_messages USING btree (message_uuid);


-- public.ai_session_settings 定义

-- Drop table

-- DROP TABLE ai_session_settings;

CREATE TABLE ai_session_settings ( id bigserial NOT NULL, session_id int8 NOT NULL, temperature float4 DEFAULT 0.7 NOT NULL, max_tokens int4 DEFAULT 4096 NOT NULL, top_p float4 DEFAULT 1.0 NOT NULL, extra jsonb DEFAULT '{}'::jsonb NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, updated_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT ai_session_settings_pkey PRIMARY KEY (id), CONSTRAINT uk_ai_session_settings_session UNIQUE (session_id), CONSTRAINT ai_session_settings_session_id_fkey FOREIGN KEY (session_id) REFERENCES ai_sessions(id) ON DELETE CASCADE);

-- Table Triggers

create trigger trg_ai_session_settings_updated_at before update on
public.ai_session_settings for each row execute function set_updated_at();


-- public.ai_message_feedbacks 定义

-- Drop table

-- DROP TABLE ai_message_feedbacks;

CREATE TABLE ai_message_feedbacks ( id bigserial NOT NULL, message_id int8 NOT NULL, user_id int8 NOT NULL, rating int2 NOT NULL, "comment" text DEFAULT ''::text NOT NULL, created_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT ai_message_feedbacks_pkey PRIMARY KEY (id), CONSTRAINT uk_ai_message_feedbacks_user_msg UNIQUE (message_id, user_id), CONSTRAINT ai_message_feedbacks_message_id_fkey FOREIGN KEY (message_id) REFERENCES ai_messages(id) ON DELETE CASCADE, CONSTRAINT ai_message_feedbacks_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE);
CREATE INDEX idx_ai_message_feedbacks_message ON public.ai_message_feedbacks USING btree (message_id);

-- public.ai_session_archives 定义（会话删除备份履历表，2026-08-07 新增）

CREATE TABLE ai_session_archives ( id bigserial NOT NULL, session_id int8 NOT NULL, session_uuid uuid NOT NULL, user_id int8 NOT NULL, title varchar(255) DEFAULT ''::character varying NOT NULL, message_count int4 DEFAULT 0 NOT NULL, total_tokens int4 DEFAULT 0 NOT NULL, messages jsonb DEFAULT '[]'::jsonb NOT NULL, deleted_at timestamptz DEFAULT now() NOT NULL, CONSTRAINT ai_session_archives_pkey PRIMARY KEY (id));
CREATE INDEX idx_ai_session_archives_user ON public.ai_session_archives USING btree (user_id, deleted_at DESC);
