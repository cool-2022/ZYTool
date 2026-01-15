<script setup lang="ts">
// App.vue - 主应用组件
</script>

<template>
  <div id="app">
    <!-- 导航栏 -->
    <nav class="navbar">
      <div class="nav-container">
        <router-link to="/" class="nav-logo">
          ZYTool
        </router-link>
        <div class="nav-menu">
          <router-link to="/" class="nav-link">首页</router-link>
          <router-link to="/tools" class="nav-link">工具</router-link>
          <router-link to="/login" class="nav-link">登陆</router-link>
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
  background-color: var(--bg-primary);
}

.navbar {
  background: var(--gradient-primary);
  padding: 1rem 0;
  box-shadow: var(--shadow-md);
  position: sticky;
  top: 0;
  z-index: 1000;
  backdrop-filter: blur(10px);
  transition: all var(--transition-speed);
}

.nav-container {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 20px;
}

.nav-logo {
  color: white;
  text-decoration: none;
  font-size: 1.8rem;
  font-weight: 700;
  letter-spacing: -0.5px;
  transition: all 0.3s ease;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
}

.nav-logo:hover {
  transform: scale(1.05);
  text-shadow: 3px 3px 6px rgba(0, 0, 0, 0.3);
  color: white;
}

.nav-menu {
  display: flex;
  gap: 1rem;
}

.nav-link {
  color: rgba(255, 255, 255, 0.9);
  text-decoration: none;
  padding: 0.6rem 1.2rem;
  border-radius: var(--border-radius);
  transition: all var(--transition-speed);
  font-weight: 500;
  position: relative;
  overflow: hidden;
}

.nav-link::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: rgba(255, 255, 255, 0.2);
  transition: left var(--transition-speed);
}

.nav-link:hover::before {
  left: 0;
}

.nav-link:hover {
  background-color: rgba(255, 255, 255, 0.15);
  transform: translateY(-2px);
  color: white;
}

.nav-link.router-link-active {
  background-color: rgba(255, 255, 255, 0.25);
  color: white;
  box-shadow: var(--shadow-sm);
}

@media (max-width: 768px) {
  .nav-container {
    flex-direction: column;
    gap: 1rem;
  }

  .nav-menu {
    gap: 0.5rem;
  }

  .nav-logo {
    font-size: 1.5rem;
  }
}
</style>