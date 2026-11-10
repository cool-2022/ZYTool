<template>
  <main class="home warm-theme">
    <div class="home-shell">
      <section class="hero-section" aria-labelledby="home-title">
        <div class="hero-copy">
          <span class="hero-badge"><ThunderboltOutlined /> ZYTool · 日常效率工具箱</span>
          <h1 id="home-title">把常用的事，<br />做得<span>简单一点。</span></h1>
          <p>工具、AI 助手和个人记账，集中在一个轻盈的工作台。</p>
        </div>
        <div class="hero-actions">
          <router-link to="/tools" class="home-btn home-btn-primary">打开工具箱 <ArrowRightOutlined /></router-link>
          <router-link to="/agents/chat" class="home-btn home-btn-secondary"><RobotOutlined /> 开始对话</router-link>
        </div>
      </section>

      <section class="entry-section" aria-labelledby="entry-title">
        <div class="section-heading"><h2 id="entry-title">从这里开始</h2><p>选择你此刻要做的事</p></div>
        <div class="entry-grid">
          <router-link to="/tools" class="entry-card entry-tools">
            <span class="entry-icon"><ToolOutlined /></span>
            <div class="entry-copy"><span class="entry-label">实用工具</span><h3>处理数据，更快一点</h3><p>JSON、编码转换、文本对比等日常工具，打开就能用。</p></div>
            <span class="entry-link">浏览工具 <ArrowRightOutlined /></span>
          </router-link>
          <router-link to="/agents/chat" class="entry-card entry-ai">
            <span class="entry-icon"><RobotOutlined /></span>
            <div class="entry-copy"><span class="entry-label">AI 助手</span><h3>聊聊想法，理清思路</h3><p>写作、编程或灵感探索，让对话帮助你找到下一步。</p></div>
            <span class="entry-link">开始对话 <ArrowRightOutlined /></span>
          </router-link>
        </div>
        <router-link to="/finance/ledger" class="ledger-entry">
          <span class="ledger-icon"><WalletOutlined /></span>
          <span class="ledger-copy"><strong>小账本</strong><span>记下日常收支，让生活心里有数</span></span>
          <span class="ledger-tag">本地保存</span>
          <span class="ledger-link">去记账 <ArrowRightOutlined /></span>
        </router-link>
      </section>

      <footer class="home-footer"><span><CheckCircleOutlined /> 简单上手，专注眼前的事</span><span>电脑与手机，都能轻松使用</span></footer>
    </div>
  </main>
</template>

<script setup lang="ts">
import {
  ArrowRightOutlined, CheckCircleOutlined, RobotOutlined,
  ThunderboltOutlined, ToolOutlined, WalletOutlined,
} from '@ant-design/icons-vue'
</script>

<style scoped>
.home {
  min-height: calc(100vh - 60px);
  padding: 20px var(--page-gutter) 36px;
  background: var(--workspace-bg);
  color: var(--workspace-text);
}
.home-shell { max-width: 1280px; margin: 0 auto; }
.hero-section {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: 32px;
  padding: 30px 0;
  border-bottom: 1px solid var(--workspace-line);
}
.hero-badge {
  display: inline-flex;
  align-items: center;
  gap: 7px;
  padding: 5px 9px;
  color: var(--workspace-accent);
  background: var(--workspace-accent-soft);
  border: 1px solid var(--workspace-accent-line);
  border-radius: 6px;
  font-size: 11px;
  letter-spacing: 0.3px;
}
.hero-copy h1 { margin: 18px 0 12px; color: var(--workspace-text); font-size: clamp(30px, 3.4vw, 44px); font-weight: 650; line-height: 1.32; letter-spacing: -1px; }
.hero-copy h1 span { color: var(--workspace-accent); }
.hero-copy p { margin: 0; color: var(--workspace-muted); font-size: 14px; line-height: 1.8; }
.hero-actions { display: flex; flex-shrink: 0; align-items: center; gap: 10px; padding-bottom: 3px; }
.home-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 9px;
  min-height: 42px;
  padding: 9px 17px;
  border: 1px solid transparent;
  border-radius: 9px;
  font-size: 13px;
  font-weight: 500;
  white-space: nowrap;
  transition: background-color var(--duration-fast) var(--ease-out), border-color var(--duration-fast) var(--ease-out), transform var(--duration-press) var(--ease-out);
}
.home-btn-primary { color: var(--workspace-bg); background: var(--workspace-accent); }
.home-btn-secondary { color: var(--workspace-text); background: var(--workspace-surface); border-color: var(--workspace-line); }
.entry-section { padding-top: 26px; }
.section-heading { display: flex; align-items: baseline; gap: 14px; margin-bottom: 16px; }
.section-heading h2 { margin: 0; color: var(--workspace-text); font-size: 21px; font-weight: 600; }
.section-heading p { margin: 0; color: var(--workspace-caption); font-size: 12px; }
.entry-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 16px; }
.entry-card {
  --entry-color: var(--workspace-accent);
  --entry-soft: var(--workspace-accent-soft);
  position: relative;
  display: flex;
  flex-direction: column;
  min-width: 0;
  padding: 24px;
  background: var(--workspace-surface);
  border: 1px solid var(--workspace-line);
  border-radius: var(--workspace-radius);
  transition: border-color var(--duration-base) var(--ease-out), box-shadow var(--duration-base) var(--ease-out), transform var(--duration-press) var(--ease-out);
}
.entry-ai { --entry-color: var(--workspace-green); --entry-soft: var(--workspace-green-soft); }
.entry-icon { display: grid; place-items: center; width: 43px; height: 43px; margin-bottom: 18px; color: var(--entry-color); background: var(--entry-soft); border-radius: 11px; font-size: 21px; }
.entry-label { display: block; margin-bottom: 5px; color: var(--entry-color); font-size: 11px; font-weight: 600; letter-spacing: 0.5px; }
.entry-copy h3 { margin: 0 0 8px; color: var(--workspace-text); font-size: 20px; font-weight: 600; line-height: 1.5; }
.entry-copy p { max-width: 440px; margin: 0 0 22px; color: var(--workspace-muted); font-size: 13px; line-height: 1.8; }
.entry-link { display: inline-flex; align-items: center; align-self: flex-start; gap: 8px; margin-top: auto; color: var(--entry-color); font-size: 12px; font-weight: 500; }
.entry-link :deep(.anticon) { transition: transform var(--duration-fast) var(--ease-out); }
.ledger-entry { display: flex; align-items: center; gap: 13px; margin-top: 14px; padding: 16px 20px; color: var(--workspace-text); background: var(--workspace-surface); border: 1px solid var(--workspace-line); border-radius: var(--workspace-radius); transition: border-color var(--duration-base) var(--ease-out), box-shadow var(--duration-base) var(--ease-out), transform var(--duration-press) var(--ease-out); }
.ledger-icon { display: grid; place-items: center; flex-shrink: 0; width: 37px; height: 37px; color: var(--workspace-sand); background: var(--workspace-sand-soft); border-radius: 9px; font-size: 18px; }
.ledger-copy { display: flex; align-items: baseline; gap: 12px; min-width: 0; }
.ledger-copy strong { flex-shrink: 0; color: var(--workspace-text); font-size: 14px; font-weight: 600; }
.ledger-copy > span { color: var(--workspace-muted); font-size: 12px; }
.ledger-tag { flex-shrink: 0; margin-left: auto; padding: 3px 8px; color: var(--workspace-green); background: var(--workspace-green-soft); border-radius: 5px; font-size: 10px; }
.ledger-link { display: inline-flex; align-items: center; flex-shrink: 0; gap: 7px; margin-left: 10px; color: var(--workspace-accent); font-size: 12px; }
.home-footer { display: flex; justify-content: space-between; gap: 16px; margin-top: 24px; padding-top: 16px; color: var(--workspace-caption); border-top: 1px solid var(--workspace-line); font-size: 11px; }
.home-footer > span:first-child { display: inline-flex; align-items: center; gap: 6px; }
.home-footer .anticon { color: var(--workspace-green); }
.home-btn:focus-visible, .entry-card:focus-visible, .ledger-entry:focus-visible { outline: 2px solid var(--workspace-accent); outline-offset: 4px; }
.home-btn:active, .entry-card:active, .ledger-entry:active { transform: scale(0.97); }
@media (hover: hover) and (pointer: fine) {
  .home-btn-primary:hover { color: var(--workspace-bg); background: var(--workspace-accent-hover); }
  .home-btn-secondary:hover { color: var(--workspace-text); background: var(--workspace-soft); border-color: var(--workspace-accent-line); }
  .entry-card:hover { border-color: var(--workspace-accent-line); box-shadow: var(--workspace-shadow-hover); }
  .entry-card:hover .entry-link { text-decoration: underline; text-underline-offset: 4px; }
  .entry-card:hover .entry-link :deep(.anticon) { transform: translateX(3px); }
  .ledger-entry:hover { border-color: var(--workspace-accent-line); box-shadow: var(--workspace-shadow-hover); }
  .ledger-entry:hover .ledger-link { text-decoration: underline; text-underline-offset: 4px; }
}
@media (min-width: 1440px) {
  .hero-copy h1 br { display: none; }
  .hero-copy h1 span { margin-left: 0.2em; }
}
@media (max-width: 900px) {
  .hero-section { gap: 24px; padding: 26px 0; }
  .hero-copy h1 { font-size: 34px; }
  .hero-copy p { max-width: 340px; }
  .hero-actions { flex-direction: column; align-items: stretch; gap: 8px; }
  .home-btn { min-height: 38px; padding: 7px 13px; }
  .entry-card { padding: 21px; }
  .ledger-copy { flex-direction: column; gap: 3px; }
}
@media (max-width: 640px) {
  .hero-section { align-items: flex-start; flex-direction: column; gap: 22px; }
  .hero-copy p { max-width: none; }
  .hero-actions { flex-direction: row; width: 100%; }
  .home-btn { flex: 1; }
  .ledger-tag { display: none; }
  .ledger-link { margin-left: auto; }
}
@media (max-width: 480px) {
  .home { padding: 16px var(--page-gutter) 26px; }
  .hero-section { gap: 18px; padding: 18px 0 22px; }
  .hero-badge { font-size: 10px; }
  .hero-copy h1 { margin: 14px 0 10px; font-size: 30px; }
  .hero-copy p { font-size: 12px; }
  .home-btn { gap: 6px; min-height: 39px; padding: 8px 10px; font-size: 12px; }
  .entry-section { padding-top: 21px; }
  .section-heading { gap: 10px; margin-bottom: 13px; }
  .section-heading h2 { font-size: 19px; }
  .section-heading p { font-size: 11px; }
  .entry-grid { grid-template-columns: 1fr; gap: 12px; }
  .entry-card { display: grid; grid-template-columns: 38px minmax(0, 1fr) auto; align-items: center; gap: 12px; padding: 17px 15px; }
  .entry-icon { width: 38px; height: 40px; margin: 0; font-size: 19px; border-radius: 9px; }
  .entry-label { margin-bottom: 2px; font-size: 10px; }
  .entry-copy h3 { margin: 0; font-size: 16px; }
  .entry-copy p { margin: 5px 0 0; font-size: 11px; line-height: 1.7; }
  .entry-link { margin: 0; font-size: 0; }
  .entry-link :deep(.anticon) { font-size: 15px; }
  .ledger-entry { gap: 10px; margin-top: 12px; padding: 14px; }
  .ledger-icon { width: 34px; height: 34px; font-size: 17px; }
  .ledger-copy { gap: 3px; }
  .ledger-copy strong { font-size: 13px; }
  .ledger-copy > span { font-size: 10px; }
  .ledger-link { gap: 4px; font-size: 11px; }
  .home-footer { align-items: flex-start; flex-direction: column; gap: 6px; margin-top: 20px; padding-top: 13px; font-size: 10px; }
}
@media (max-width: 360px) {
  .ledger-copy > span { max-width: 130px; }
  .ledger-link { font-size: 0; }
  .ledger-link :deep(.anticon) { font-size: 15px; }
}
@media (prefers-reduced-motion: reduce) {
  .home-btn, .entry-card, .ledger-entry { transition-property: background-color, border-color, box-shadow; }
  .entry-link :deep(.anticon) { transition: none; }
  .home-btn:active, .entry-card:active, .ledger-entry:active { transform: none; }
}
</style>
