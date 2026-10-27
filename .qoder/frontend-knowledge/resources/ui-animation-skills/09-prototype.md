> 来源：翻译自 666_AI/skills/skills/prototype/SKILL.md 与 PICKER.md。用途：为你描述的一块 UI 构建多个真正不同的版本，放在可视化切换器后面实时翻页比较，并把你选中的版本提升进代码库。

---

name: prototype
description: Build multiple genuinely different versions of a UI piece you describe, rendered behind a visual picker so you can flip through them live and promote the one that feels right. Only runs when explicitly invoked; it does not trigger on its own.
disable-model-invocation: true
---

# 原型变体

这是一个发散型技能。它只做一件事：拿到一段被描述的 UI（"一个 toast"、"定价卡片"、"一个长按删除按钮"），构建几个真正不同的版本，并把它们放到一个可视化切换器后面，让用户实时翻页比较、选出胜者。它不评审现有 UI（那是 `review-animations` 的事）、不为其规划修复（那是 `improve-animations`）、也不选择依赖（那是 `pick-ui-library`）。

## 工作姿态

你是一位资深设计工程师，正在进行一场设计探索。本技能的全部价值在于**发散**：同一想法的三个色调只会浪费切换器——用户在它们之间翻页学不到任何东西。每个变体都必须是一个你能为其独立上线辩护的方向，探索同一份需求简报下真正不同的答案。

发散不是降低工艺标准的借口。每个变体单独都要达到 Emil Kowalski 的标准——正确的缓动（进入用 `ease-out`，绝不用 `ease-in`）、UI 动效低于 300ms、正确的 `transform-origin`、只用 `transform`/`opacity`、处理 reduced-motion。一个粗糙的变体不会拓宽探索；它只是输在执行力上，且无法教给我们任何关于其所代表方向的东西。

## 硬性规则

1. **探索期间绝不动生产代码。** 所有东西都放在隔离的原型表面中（见阶段 4）。集成只发生在阶段 6，且只针对用户选中的那个变体。
2. **变体沿一条具名的轴发散**——布局、密度、个性、动效、交互模型。动手前，你必须能用一句话说出每个变体的轴。共享项目的 token 不算收敛；变体*本来就应该*看起来与产品浑然一体。
3. **每个变体都完整可用。** 真实的交互、真实的动效、逼真的内容——真正产品形态的文案、可信的名字和数字。不要 lorem ipsum、不要死按钮、不要"想象这一部分"。
4. **切换器是脚手架，不是参赛者。** 它的确切标记、样式和行为在本文“切换器规格”章节中规定——逐字复制。它的外观不是设计决策，永远不去适配项目。
5. **选定后清理干净。** 当胜者被提升后，删除原型表面，除非用户要求保留。

## 工作流程

### 阶段 1 — 圈定范围

每次运行只做一件事。如果描述横跨多个组件（"这个仪表盘"），收窄它：挑出杠杆效应最高的单一件，说明是哪件以及为什么，并把其余的作为后续运行提供。用一句话复述需求简报——这个东西是什么、它将放在哪里、它必须做什么。

### 阶段 2 — 侦察

在设计任何东西之前，摸清变体必须立足的地面：

- **技术栈**：框架、样式系统（Tailwind、CSS modules、原生 CSS）、动效库（如有）。
- **Token**：颜色、圆角、间距、字体、缓动/时长变量。变体使用这些 token——每个变体看起来都应该像明天就能在这个产品里上线。
- **个性**：活泼的消费级应用还是干练的仪表盘？这决定了最大胆的变体能走多远。
- **上下文**：这件 UI 渲染在哪里——什么背景之上、旁边有什么邻居、什么尺寸。

如果没有项目（空目录，或用户只是在探索），跳到阶段 4 的独立分支，选择一个克制的默认外观：中性灰、一个强调色、系统字体栈。

### 阶段 3 — 选择方向

默认 **3 个变体**；当用户要求或设计空间确实宽广时最多 5 个。超过 5 个会稀释比较。

在写任何代码之前，列出这个集合：每个变体一个名字和一条轴。名字描述方向——"Quiet"、"Editorial"、"Playful"、"Dense"——绝不要"选项 A/B/C"。如果两个提议的方向只在强调色或文案上不同，它们是同一个方向；用真正的替代方案替换掉其中一个（不同布局、不同交互模型、不同动效叙事）。

**完成标准：** 每个变体都有名字和明确陈述的轴，且没有任何两个变体共享同一个轴位置。

### 阶段 4 — 构建切换器载体

按现有条件分两个分支：

- **在有 dev server 的项目中**——一条隔离的路由或页面（`/prototypes/<slug>`，或框架的等价物），每个变体一个文件，外加一个小的载体文件。任何东西都不从原型表面导入到生产代码。
- **没有项目 / 静态上下文**——单个自包含的 HTML 文件（内联 CSS/JS），用户可以直接在浏览器中打开。

切换器的标记、样式、键盘接线和摆放位置来自"切换器规格"章节，逐字复制——现在就加载它并严格按它构建。除切换器本身外，载体必须**一次渲染一个变体、全尺寸、置于逼真的周边上下文中**——toast 需要一个页面在它后面，卡片需要兄弟元素，按钮需要一个表单。并排缩略图会扭曲间距和尺度；绝不在邮票大小下评判 UI。切换是**瞬时的**——翻页是每会话 100+ 次的动作；按频率规则，变体切换不配动画。

### 阶段 5 — 验证并交付

运行载体。确认每个变体都能渲染、每个交互都有响应、控制台干净——在向用户展示之前自己先把所有变体翻一遍。如果浏览器工具可用，给每个变体截图。

然后呈现这个集合，并**停手——选择权属于用户**：

| # | 变体 | 轴 | 何时它是正确选择 | 它的代价 |
| --- | --- | --- | --- | --- |
| 1 | Quiet | 极简动效，边框优于阴影 | 产品是日常高频工具 | 最不令人难忘 |
| 2 | Editorial | 大字重排版，充裕留白 | 这个时刻值得庄重感 | 吞噬纵向空间 |

最后说明切换器运行在哪里（URL 或文件路径）以及翻页按键。

**完成标准：** 每个变体都能从切换器到达且行为正确；无控制台错误；表格诚实地写出了每个变体的取舍。

### 阶段 6 — 选定后提升

当用户做出选择：把该变体集成到它应在的位置，遵循项目既有约定（文件布局、命名、token 使用），然后按硬性规则 5 删除原型表面。如果用户想再来一轮，保留载体并重新运行阶段 3，围绕他们所倾向的方向*向外*发散。

## 调用变体

| 调用 | 行为 |
| --- | --- |
| `<description>` | 完整流程：圈定范围 → 侦察 → 3 个变体 → 切换器 → 等待选择 |
| `<description> x5` | 同上，但生成指定数量的变体（上限 5） |
| `riff <variant>` | 新一轮：保留载体，围绕指定变体的方向生成一组全新变体 |
| `keep <variant>` | 把该变体提升进代码库并删除原型表面 |
| `keep <variant>, leave the picker` | 提升该变体，但保留原型表面 |

## 语气

诚实地推销每个变体——一行说它在何时胜出，一行说它的代价。绝不在表格里预先钦定最爱；如果用户问你会选哪个，用一个根植于产品个性和使用频率的理由回答，而非仅凭美学。如果两个变体在你构建过程中收敛了，砍掉一个并明说：一个真正只有两个方向的切换器胜过凑数到三个的。

---

# 切换器规格（PICKER）

切换器的外观**不是设计决策**——它就是这份规格。逐字复制下面的标记、CSS 和接线；每次运行中唯一变化的值是变体名称和数量。它在每个项目中保持完全相同，以便它始终被读作载体脚手架，而绝不被读作被评判设计的一部分。不要用项目的 token、字体或颜色给它重新换肤。

它是一个悬浮的深色胶囊，位于底部居中。深色玻璃在任何页面之上都有效——无论浅色还是深色——这正是它不做主题感知的原因。

## 标记

先是滑动高亮 span，然后每个变体一个按钮、一条发丝分割线，最后是重播按钮（仅当至少一个变体有值得重新触发的动效时）：

```html
<nav class="proto-picker" aria-label="Prototype variants">
  <span class="proto-picker-highlight" aria-hidden="true"></span>
  <button class="proto-picker-item" data-active aria-current="true">Quiet</button>
  <button class="proto-picker-item">Editorial</button>
  <button class="proto-picker-item">Playful</button>
  <span class="proto-picker-divider" aria-hidden="true"></span>
  <button class="proto-picker-item proto-picker-replay" aria-label="Replay animation (R)">↻</button>
</nav>
```

在框架中，保留类名和结构；只有渲染语法变化。

## 样式

```css
.proto-picker {
  position: fixed;
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 2147483647;
  display: flex;
  align-items: center;
  gap: 2px;
  padding: 4px;
  border-radius: 999px;
  background: rgba(10, 10, 10, 0.82);
  -webkit-backdrop-filter: blur(12px) saturate(1.4);
  backdrop-filter: blur(12px) saturate(1.4);
  box-shadow:
    0 0 0 1px rgba(255, 255, 255, 0.08) inset,
    0 8px 24px rgba(0, 0, 0, 0.24),
    0 2px 6px rgba(0, 0, 0, 0.12);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  font-size: 13px;
  line-height: 1;
  -webkit-font-smoothing: antialiased;
  user-select: none;
  -webkit-user-select: none;
}

.proto-picker-highlight {
  position: absolute;
  top: 4px;
  left: 0;
  height: 28px;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.12);
  will-change: transform;
}

/* The slide is enabled only after first paint (data-ready), so load doesn't animate. */
.proto-picker[data-ready] .proto-picker-highlight {
  transition:
    transform 250ms cubic-bezier(0.23, 1, 0.32, 1),
    width 250ms cubic-bezier(0.23, 1, 0.32, 1);
}

@media (prefers-reduced-motion: reduce) {
  .proto-picker[data-ready] .proto-picker-highlight { transition: none; }
}

.proto-picker-item {
  position: relative; /* sits above the highlight */
  display: flex;
  align-items: center;
  height: 28px;
  padding: 0 12px;
  border: 0;
  border-radius: 999px;
  background: transparent;
  color: rgba(255, 255, 255, 0.55);
  font: inherit;
  cursor: pointer;
  transition: color 150ms ease-out;
}

.proto-picker-item:hover {
  color: rgba(255, 255, 255, 0.85);
}

.proto-picker-item:active {
  transform: scale(0.97);
}

.proto-picker-item:focus-visible {
  outline: 2px solid rgba(255, 255, 255, 0.4);
  outline-offset: 2px;
}

.proto-picker-item[data-active] {
  color: #fff;
}

.proto-picker-divider {
  width: 1px;
  height: 16px;
  margin: 0 4px;
  background: rgba(255, 255, 255, 0.12);
}

.proto-picker-replay {
  padding: 0 10px;
  font-size: 14px;
}

.proto-picker[data-position="top"] {
  bottom: auto;
  top: 24px;
}
```

## 规则

- **逐字。** 这些值就是规格。不要项目字体、不要品牌色、不要主题切换、不要额外的阴影或边框。
- **高亮滑动；变体切换保持瞬时。** 激活胶囊在按钮之间做动画（250ms，强 ease-out），作为切换器自身的空间反馈——但被预览的变体仍然无过渡地切换。`width` 过渡是 transform/opacity 规则的一次刻意例外：该元素高 28px、绝对定位、没有布局依赖者，因此绘制成本可忽略。
- **唯一允许的修改：** 如果某个变体占据屏幕底部居中位置（toast 堆叠、底部抽屉、dock），设置 `data-position="top"`，使切换器永远不遮挡作品。其他任何东西都不得移动或改变。
- **重播是有条件的。** 仅当至少一个变体有值得重新触发的入场或状态动画时，才渲染重播按钮及其分割线；纯静态对比用更短的胶囊。

## 行为契约

无论载体如何渲染，契约是固定的：

- 数字键 `1–N` 和 `←`/`→` 切换变体；`R` 重播。当焦点在 input、textarea、select 或 contenteditable 中，或按住修饰键时，忽略按键事件。
- 点击某项切换到该项；任何时候恰有一项携带 `data-active` 和 `aria-current="true"`，且高亮滑动到它。
- 选择通过 URL 参数（`?v=2`）跨刷新持久化，回退到变体 1。高亮无动画地到达初始位置（`data-ready` 在首次绘制后才添加）。
- 切换会重新挂载变体（因此入场动画重新运行）；重播键在不切换的情况下重新挂载。

## 参考接线

独立 HTML 分支下逐字使用；在框架中，保持相同行为但用地道的方式表达（用 state 代替 `innerHTML`，用带 key 的重新挂载代替 `requestAnimationFrame`，用 refs + 布局效应做高亮测量）。

```js
// `variants` is an array of render functions, one per variant, in picker order.
const stage = document.getElementById('stage');
const picker = document.querySelector('.proto-picker');
const highlight = picker.querySelector('.proto-picker-highlight');
const items = [...picker.querySelectorAll('.proto-picker-item:not(.proto-picker-replay)')];
const replay = picker.querySelector('.proto-picker-replay');
let current = 0;

function moveHighlight() {
  const el = items[current];
  highlight.style.width = el.offsetWidth + 'px';
  highlight.style.transform = `translateX(${el.offsetLeft}px)`;
}

function mount(i) {
  stage.innerHTML = '';
  // Clear first, render next frame, so entrance animations re-run.
  requestAnimationFrame(() => { stage.innerHTML = variants[i](); });
}

function setActive(i) {
  if (i < 0 || i >= variants.length) return;
  current = i;
  items.forEach((el, j) => {
    el.toggleAttribute('data-active', j === i);
    if (j === i) el.setAttribute('aria-current', 'true');
    else el.removeAttribute('aria-current');
  });
  moveHighlight();
  const url = new URL(location);
  url.searchParams.set('v', i + 1);
  history.replaceState(null, '', url);
  mount(i);
}

items.forEach((el, i) => el.addEventListener('click', () => setActive(i)));
replay?.addEventListener('click', () => mount(current));
window.addEventListener('resize', moveHighlight);

document.addEventListener('keydown', (e) => {
  if (/^(INPUT|TEXTAREA|SELECT)$/.test(e.target.tagName) || e.target.isContentEditable) return;
  if (e.metaKey || e.ctrlKey || e.altKey) return;
  const num = parseInt(e.key, 10);
  if (num >= 1 && num <= variants.length) setActive(num - 1);
  else if (e.key === 'ArrowRight') setActive((current + 1) % variants.length);
  else if (e.key === 'ArrowLeft') setActive((current - 1 + variants.length) % variants.length);
  else if (e.key === 'r' || e.key === 'R') mount(current);
});

setActive((parseInt(new URLSearchParams(location.search).get('v'), 10) || 1) - 1);
// Enable the slide only after first paint, so load doesn't animate.
requestAnimationFrame(() => requestAnimationFrame(() => picker.setAttribute('data-ready', '')));
```
