<script setup lang="ts">
// App.vue - 主应用组件
</script>

<template>
  <div id="app">
    <!-- 导航栏 -->
    <nav class="navbar">
      <div class="nav-container">
        <router-link to="/" class="nav-logo">
          <span class="logo-icon">⚡</span>
          <span class="logo-text">ZYTool</span>
        </router-link>
        <div class="nav-menu">
          <router-link to="/home" class="nav-link">首页</router-link>
          <router-link to="/tools" class="nav-link">工具</router-link>
          <router-link to="/" class="nav-link nav-link-primary">登录</router-link>
        </div>
      </div>
    </nav>

    <!-- 路由视图 -->
    <router-view />
  </div>
</template>

<style scoped>
#app {
  min-height: 100vh;
  background: var(--gradient-bg);
}

.navbar {
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border-bottom: 1px solid var(--glass-border);
  padding: 0.875rem 0;
  position: sticky;
  top: 0;
  z-index: 1000;
  transition: all var(--transition-speed);
}

.nav-container {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 24px;
}

.nav-logo {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  text-decoration: none;
  transition: transform 0.3s ease;
}

.nav-logo:hover {
  transform: scale(1.02);
}

.logo-icon {
  font-size: 1.5rem;
  filter: drop-shadow(0 2px 4px rgba(59, 130, 246, 0.3));
}

.logo-text {
  font-size: 1.6rem;
  font-weight: 800;
  background: var(--gradient-primary);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: -0.5px;
}

.nav-menu {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.nav-link {
  color: var(--text-secondary);
  text-decoration: none;
  padding: 0.55rem 1.1rem;
  border-radius: var(--border-radius);
  transition: all var(--transition-speed);
  font-weight: 500;
  font-size: 0.95rem;
}

.nav-link:hover {
  color: var(--primary-color);
  background: rgba(59, 130, 246, 0.08);
}

.nav-link.router-link-active {
  color: var(--primary-color);
  background: rgba(59, 130, 246, 0.12);
  font-weight: 600;
}

.nav-link-primary {
  background: var(--gradient-primary);
  color: white !important;
  box-shadow: var(--shadow-primary);
  margin-left: 0.5rem;
}

.nav-link-primary:hover {
  background: var(--gradient-primary-hover);
  box-shadow: var(--shadow-primary-hover);
  transform: translateY(-1px);
}

@media (max-width: 768px) {
  .nav-container {
    padding: 0 16px;
  }

  .logo-text {
    font-size: 1.4rem;
  }

  .nav-link {
    padding: 0.45rem 0.8rem;
    font-size: 0.9rem;
  }

  .nav-link-primary {
    margin-left: 0.25rem;
  }
}
</style>
