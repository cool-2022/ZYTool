<template>
  <div class="home">
    <!-- Hero 区域 -->
    <section class="hero-section">
      <div class="hero-grid">
        <div class="hero-glow hero-glow-1"></div>
        <div class="hero-glow hero-glow-2"></div>
      </div>
      <div class="hero-content">
        <div class="hero-badge">
          <span class="badge-dot"></span>
          一站式在线工具平台
        </div>
        <h1 class="hero-title">
          <span class="title-main">ZYTool</span>
          <span class="title-sub">效率工具集合</span>
        </h1>
        <p class="hero-subtitle">
          简洁、强大、开箱即用的在线工具箱，让开发和工作更高效
        </p>
        <div class="hero-actions">
          <router-link to="/tools" class="btn btn-primary">
            开始使用
            <span class="btn-icon">→</span>
          </router-link>
          <router-link to="/agents/chat" class="btn btn-secondary">
            AI 助手
          </router-link>
        </div>
      </div>
    </section>

    <!-- 特性介绍 -->
    <section class="features-section">
      <div class="container">
        <div class="features-grid">
          <div class="feature-card">
            <div class="feature-icon">🔧</div>
            <h3>丰富工具</h3>
            <p>覆盖前后端开发常用工具，持续更新扩展</p>
          </div>
          <div class="feature-card">
            <div class="feature-icon">🤖</div>
            <h3>AI 助手</h3>
            <p>集成 DeepSeek 智能对话，支持流式响应</p>
          </div>
          <div class="feature-card">
            <div class="feature-icon">🚀</div>
            <h3>开箱即用</h3>
            <p>无需安装，浏览器打开即可使用</p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
// HomeView - 首页仅展示 Hero 和特性介绍
</script>

<style scoped>
.home {
  width: 100%;
  min-height: 100vh;
}

/* Hero 区域 */
.hero-section {
  position: relative;
  padding: 60px 24px 80px;
  background: var(--gradient-hero);
  overflow: hidden;
  text-align: center;
}

.hero-grid {
  position: absolute;
  inset: 0;
  pointer-events: none;
}

.hero-glow {
  position: absolute;
  border-radius: 50%;
  filter: blur(80px);
  opacity: 0.35;
}

.hero-glow-1 {
  width: 500px;
  height: 500px;
  top: -150px;
  right: 10%;
  background: var(--primary-color);
  animation: float 8s ease-in-out infinite;
}

.hero-glow-2 {
  width: 400px;
  height: 400px;
  bottom: -100px;
  left: 5%;
  background: var(--accent-color);
  animation: float 10s ease-in-out infinite reverse;
}

.hero-content {
  position: relative;
  z-index: 2;
  max-width: 800px;
  margin: 0 auto;
}

.hero-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: var(--gradient-glass);
  border: 1px solid rgba(255, 255, 255, 0.15);
  border-radius: 50px;
  color: rgba(255, 255, 255, 0.9);
  font-size: 0.9rem;
  font-weight: 500;
  margin-bottom: 1.5rem;
  backdrop-filter: blur(10px);
}

.badge-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #22c55e;
  box-shadow: 0 0 8px #22c55e;
}

.hero-title {
  margin: 0 0 1.25rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.title-main {
  font-size: 4.5rem;
  font-weight: 900;
  background: linear-gradient(135deg, #ffffff 0%, #bfdbfe 50%, #ddd6fe 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: -2px;
  line-height: 1.1;
}

.title-sub {
  font-size: 1.75rem;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.75);
  letter-spacing: 4px;
}

.hero-subtitle {
  font-size: 1.25rem;
  color: rgba(255, 255, 255, 0.65);
  margin: 0 auto 2rem;
  max-width: 560px;
  line-height: 1.7;
}

.hero-actions {
  display: flex;
  justify-content: center;
  gap: 1rem;
  flex-wrap: wrap;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.85rem 1.75rem;
  border-radius: var(--border-radius);
  font-weight: 600;
  font-size: 1rem;
  text-decoration: none;
  transition: all 0.3s ease;
  cursor: pointer;
  border: none;
}

.btn-primary {
  background: var(--gradient-primary);
  color: white;
  box-shadow: 0 4px 16px rgba(59, 130, 246, 0.4);
}

.btn-primary:hover {
  background: var(--gradient-primary-hover);
  transform: translateY(-2px);
  box-shadow: 0 6px 24px rgba(59, 130, 246, 0.5);
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.1);
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.18);
  transform: translateY(-2px);
}

.btn-icon {
  transition: transform 0.3s ease;
}

.btn-primary:hover .btn-icon {
  transform: translateX(3px);
}

/* 特性介绍区域 */
.features-section {
  padding: 100px 24px 120px;
  background: var(--gradient-bg);
}

.container {
  max-width: 1200px;
  margin: 0 auto;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 2rem;
}

.feature-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius-xl);
  padding: 2.5rem 2rem;
  text-align: center;
  box-shadow: var(--shadow-md);
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.feature-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: var(--gradient-primary);
  opacity: 0;
  transition: opacity 0.3s ease;
}

.feature-card:hover {
  transform: translateY(-8px);
  box-shadow: var(--shadow-lg);
}

.feature-card:hover::before {
  opacity: 1;
}

.feature-icon {
  font-size: 3rem;
  margin-bottom: 1.25rem;
  display: inline-block;
  transition: transform 0.3s ease;
}

.feature-card:hover .feature-icon {
  transform: scale(1.15);
}

.feature-card h3 {
  font-size: 1.35rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 0.75rem;
}

.feature-card p {
  font-size: 0.95rem;
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.6;
}

@keyframes float {

  0%,
  100% {
    transform: translateY(0) scale(1);
  }

  50% {
    transform: translateY(-20px) scale(1.05);
  }
}

@media (max-width: 768px) {
  .hero-section {
    padding: 70px 20px 90px;
  }

  .title-main {
    font-size: 3rem;
  }

  .title-sub {
    font-size: 1.25rem;
    letter-spacing: 2px;
  }

  .hero-subtitle {
    font-size: 1.05rem;
  }

  .btn {
    padding: 0.75rem 1.5rem;
  }

  .features-section {
    padding: 60px 20px 80px;
  }

  .features-grid {
    grid-template-columns: 1fr;
    gap: 1.25rem;
  }

  .feature-card {
    padding: 2rem 1.5rem;
  }
}
</style>
