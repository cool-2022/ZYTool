<!-- 来源：animate/SKILL.md + animate/RECIPES.md（动画构建 Skill 及其配方库）。用途：从零构建动画的决策序列与现成实现配方，自包含知识库副本。 -->

---
name: animate
# description（原）：Build an animation from scratch, making the decisions in the order that determines whether it feels right — should it animate at all, what purpose, which tool, which properties, which curve and duration, how it interrupts, how it exits. Writes the implementation. Use when asked to animate something, add motion, make a component feel alive, or build a transition. For critiquing existing motion use review-animations; for auditing a whole codebase use improve-animations.
# description（译）：从零构建动画，按"决定手感是否正确"的顺序做决策——该不该动、目的是什么、用什么工具、哪些属性、哪条曲线和时长、如何被打断、如何退出。产出实现代码。当被要求做动画、加动效、让组件活起来或构建过渡时使用。评审已有动效用 review-animations；审计整个代码库用 improve-animations。
---

# 构建动画

一个构建型 Skill。它只做**一件事**：把一个动效请求变成能通过严格评审的实现。它不审计代码库（那是 `improve-animations` 的活）、不评审 diff（那是 `review-animations` 的活）、不寻找可以加动画的地方（那是 `find-animation-opportunities` 的活）。

## 工作姿态

你是一名资深设计工程师，亲手构建这个动画。标准是 Emil Kowalski 的动画哲学——与 `review-animations` 执行的标准相同。写到第一次就能通过那个评审。

两种失败模式，第一种更严重：

1. **给不该动的东西做了动画。** 下面的门槛存在的目的，就是有时产出零行代码。那是成功，不是逃避。
2. **给对的东西用了错的配料**——入场用 `ease-in`、`scale(0)`、toast 用 keyframes、让下拉菜单显得迟钝的时长。

永远不要把动效选项列成菜单让用户挑。自己做决定，用一行话说明理由，然后写代码。

## 硬性规则

1. **按顺序执行序列。** 第 1、2 步是后面一切的门槛。在确定动不动之前，不要伸手去拿曲线。
2. **不许用拍脑袋的近似值。** 每条曲线、时长、弹簧配置都来自下面的表格。永远不要因为它眼熟就发明 `cubic-bezier(0.4, 0, 0.2, 1)`。
3. **扩展代码库已有的 token，不要另起炉灶。** 如果 `--ease-out` 或一套时长阶梯已经存在，用它。加一套平行系统就是缺陷。
4. **减少动态（reduced motion）和 hover 门控随动画一起交付**，不是事后补。
5. **用最便宜的够用工具。** 不要为了一个淡入安装动效库。

## 构建序列

### 1. 这里到底该不该有动画？

| 频率 | 决策 |
| --- | --- |
| 每天 100+ 次（键盘快捷键、命令面板开关） | **永远不做动画。**到此为止。 |
| 每天几十次（悬停效果、列表导航） | 只做几乎不可察觉的——快而轻微，或者不做 |
| 偶尔（modal、抽屉、toast） | 标准动画 |
| 罕见 / 首次（新手引导、成功、庆祝） | 惊喜预算用在这里 |

**键盘触发的动作是取消资格项，不是判断题。** Raycast 没有打开/关闭动画——对每天打开数百次的东西来说这是对的。

如果请求没有通过这道门槛，直说，不要写动画。改为提供非动效替代方案（即时状态变化、静态可供性暗示）。

### 2. 目的是什么？

用下面这些词之一给它命名，然后才能继续：

- **反馈（Feedback）**——确认界面听到了用户
- **空间一致性（Spatial consistency）**——展示某物从哪里来、到哪里去
- **状态指示（State indication）**——让状态变化可读
- **防止生硬突变（Preventing a jarring change）**——为否则会瞬间传送的内容搭桥
- **解释（Explanation）**——演示某物如何工作（仅限营销/新手引导）
- **惊喜（Delight）**——*只允许*出现在罕见/首次层级

命不了名？就不要做。高频元素上的"看起来很酷"是停下的理由。

还要检查**功能**：用户正在阅读或操作的数据不应为了风格而移动。装饰性的鼠标跟随效果属于营销页，不属于银行应用里的图表。

### 3. 选工具——最便宜的够用方案

从上往下走，在第一个匹配处停下。

| 需求 | 工具 |
| --- | --- |
| Hover、按压、颜色、用 class 或属性控制的状态切换 | **CSS transition** |
| 挂载时的入场动画，无 JS 状态 | **CSS `@starting-style`** |
| 页面忙于加载时必须保持流畅的预定运动 | **CSS animation**（在主线程之外运行） |
| 要编程控制又要 CSS 性能，不要库 | **WAAPI**（`element.animate()`） |
| 弹簧、布局动画、退出动画、手势驱动的值 | **Motion**（`motion.dev`） |

高负载下 CSS 动画胜过 JS——它们在主线程之外运行，而基于 `requestAnimationFrame` 的动画在浏览器加载、跑脚本或绘制时会掉帧。预定运动用 CSS，动态、可中断的运动用 JS。

如果任务需要的是一个*组件*而不是一个动画——toast、抽屉、命令菜单、下拉——停下来，调用 `pick-ui-library`。手搓这些组件的下场就是一个 `<div>` 下拉框加上零焦点管理。

### 4. 选属性

- **只用 `transform` 和 `opacity`。** 它们跳过 layout 和 paint，在 GPU 上运行。`width`/`height`/`margin`/`padding`/`top`/`left` 会触发全部三个渲染步骤。（`clip-path` 是被认可的第四个——见下文"配方库"章节。`height` 只在手风琴中被容忍，因为没有等价的 transform 方案。）
- **永远不要 `scale(0)`。** 从 `scale(0.9–0.97)` + `opacity: 0` 开始。现实世界中没有任何东西凭空出现。
- **弹层、下拉、菜单、tooltip 的 `transform-origin` 设在触发点**——Base UI 中用 `var(--transform-origin)`。**Modal 豁免**：它们不锚定在触发点上，所以保持居中。
- **`translate()` 中的百分比**相对于元素自身尺寸——`translateY(100%)` 移动自身高度，无论内容如何。优先于硬编码像素。
- **在 Motion 中用完整的 transform 字符串。** `x`/`y`/`scale` 简写不是硬件加速的，高负载下会掉帧：

```jsx
<motion.div animate={{ x: 100 }} />                          // 高负载下掉帧
<motion.div animate={{ transform: "translateX(100px)" }} />  // 硬件加速
```

- **永远不要通过父元素上的 CSS 变量驱动子元素的 transform**——它会重算每个子元素的样式。直接在元素上设置 `transform`。

### 5. 缓动和时长——或者弹簧

**缓动**，按决策顺序：

| 情形 | 缓动 |
| --- | --- |
| 进入或退出 | `ease-out` |
| 在屏幕上移动 / 形变 | `ease-in-out` |
| Hover / 颜色变化 | `ease` |
| 恒定运动（跑马灯、进度） | `linear` |
| 默认 | `ease-out` |

**UI 上永远不要 `ease-in`。** 它起步慢，恰好延迟了用户最紧盯的时刻。200ms 的 `ease-out` *感觉上*比 200ms 的 `ease-in` 快。

CSS 内置缓动太弱。用这些：

```css
--ease-out: cubic-bezier(0.23, 1, 0.32, 1);        /* 用于 UI 的强 ease-out */
--ease-in-out: cubic-bezier(0.77, 0, 0.175, 1);    /* 用于屏幕上移动的强 ease-in-out */
--ease-drawer: cubic-bezier(0.32, 0.72, 0, 1);     /* 类 iOS 抽屉曲线（Ionic） */
```

需要这里没有列出的曲线？从 [easing.dev](https://easing.dev/) 或 [easings.co](https://easings.co/) 取。不要手搓。

**时长：**

| 元素 | 时长 |
| --- | --- |
| 按钮按压反馈 | 100–160ms |
| Tooltip、小型弹层 | 125–200ms |
| 下拉菜单、选择器 | 150–250ms |
| Modal、抽屉 | 200–500ms |
| 营销 / 解释性 | 可以更长 |

**UI 动画保持在 300ms 以内。** 180ms 的下拉比 400ms 的感觉更灵敏。

**以下情况改用弹簧**：带惯性的拖拽、应该感觉活着的元素、用户可以打断或反向的手势、装饰性鼠标跟随：

```js
{ type: "spring", duration: 0.5, bounce: 0.2 }        // Apple 风格——更容易推理
{ type: "spring", mass: 1, stiffness: 100, damping: 10 }  // 传统物理——控制力更强
```

bounce 保持在 0.1–0.3，大多数 UI 中避免 bounce——留给拖拽关闭和 playful 交互。

### 6. 中断与退出

- **任何快速触发的东西用 transitions，不用 keyframes**——toast、开关切换、任何用户一秒内可能触发两次的东西。Transitions 从当前值重定向；keyframes 从零重启。
- **手势用弹簧**，因为它们被打断时带着速度走。
- **怎么进来就怎么出去。** 从底部滑入的 toast 从底部离开。对称的路径让滑动关闭显得不言自明。
- **用户做决定的地方用非对称时机。** 刻意的阶段慢（长按确认的按压：2s linear），系统响应利落（松开：200ms ease-out）。

### 7. 减少动态与指针门控

每次都随动画一起交付。

```css
@media (prefers-reduced-motion: reduce) {
  .element { animation: fade 0.2s ease; } /* 保留 opacity/颜色，去掉基于 transform 的运动 */
}

@media (hover: hover) and (pointer: fine) {
  .element:hover { transform: scale(1.05); } /* 触屏点按会误触发 hover */
}
```

```jsx
const reduce = useReducedMotion();
const closedX = reduce ? 0 : '-100%';
```

减少动态意味着**更少、更温和**的动画，不是零——保留有助于理解的过渡，移除移动和位置变化。

## 配方库

常见场景的现成实现——按钮按压、下拉、tooltip、modal、抽屉、toast、手风琴、交错、长按确认、tab 指示器、滚动揭示、拖拽关闭——见下方"配方库"章节。请求命中其中某个组件时就读取它；从配方出发，而不是从空白文件出发。

## 永不交付清单（Never Ship）

完成前自查。下面每一条在 `review-animations` 中都是自动阻断项：

| 永远不要 | 改为 |
| --- | --- |
| `transition: all` | 点名确切的属性 |
| `transform: scale(0)` 入场 | `scale(0.95)` + `opacity: 0` |
| UI 元素用 `ease-in` | `ease-out` 或强自定义曲线 |
| 刻意的动画用内置 `ease-out` | `cubic-bezier(0.23, 1, 0.32, 1)` |
| 键盘快捷键或每天 100+ 次动作上的动画 | 不做动画 |
| 无理由超过 300ms 的 UI 时长 | 150–250ms |
| 锚定触发点的弹层用 `transform-origin: center` | `var(--transform-origin)`（modal 豁免） |
| toast、开关、快速触发元素用 keyframes | CSS transitions |
| 对 `width`/`height`/`margin`/`padding`/`top`/`left` 做动画 | `transform` / `opacity` |
| 高负载下用 Motion `x`/`y`/`scale` props | 完整 `transform` 字符串 |
| 未门控的 `:hover` 动效 | `@media (hover: hover) and (pointer: fine)` |
| 缺少 `prefers-reduced-motion` | 更温和的变体，不是零 |
| 所有东西同时入场 | 30–80ms 交错 |

## 输出

写代码。然后，最多用几行：

- **门槛结果**——频率层级和命名的目的。如果请求中有被否决的部分，说清是哪个、为什么。
- **配料**——工具、属性、曲线、时长或弹簧配置，每项一行。
- **需要手感确认的点**——如果结果取决于代码无法判断的手感（一个 crossfade、弹簧的 bounce、入场列表中 opacity/height 的平衡），说明并指出验证方法：以 2–5 倍时长播放或用 DevTools 动画检查器、逐帧步进、在真实设备上测手势、第二天用新鲜眼光再看一遍。

不要把这个填充成报告。代码才是交付物。

## 语气

有主见、简短。当诚实的答案是"这里不该动"时，就给这个答案——这个答案正是本 Skill 存在的理由。当手感确实无法从代码判定时，直说，而不是猜一个值。

---

# 配方库（RECIPES）

常见场景的现成实现。从配方出发再做适配——不要从零重建。

曲线即 SKILL 部分定义的 `--ease-out`、`--ease-in-out`、`--ease-drawer` token。

---

## 按钮按压

任何可按压元素。确认界面听到了用户的即时反馈。

```css
.button {
  transition: transform 160ms var(--ease-out);
}

.button:active {
  transform: scale(0.97);
}
```

`scale()` 会连带缩放子元素——文字和图标一起缩，这正是它读起来像物理按压的原因。

这里不需要 hover 门控：`:active` 在触屏上就是真实按压。任何 `:hover` 样式单独门控。

---

## 下拉、弹层、菜单、选择器

从触发点缩放展开，而不是凭空出现。

```css
.popover {
  transform-origin: var(--transform-origin); /* Base UI 提供此变量 */
  transition:
    opacity 200ms var(--ease-out),
    transform 200ms var(--ease-out);
}

.popover[data-starting-style],
.popover[data-ending-style] {
  opacity: 0;
  transform: scale(0.95);
}
```

`transform-origin` 是全部要点——面板应该看起来像从你点击的东西里长出来的。

---

## Tooltip

形状与弹层相同，更快，外加大多数实现漏掉的那个细节。

```css
.tooltip {
  transform-origin: var(--transform-origin);
  transition:
    transform 125ms var(--ease-out),
    opacity 125ms var(--ease-out);
}

.tooltip[data-starting-style],
.tooltip[data-ending-style] {
  opacity: 0;
  transform: scale(0.97);
}

/* 一个 tooltip 打开后，相邻的即时打开 */
.tooltip[data-instant] {
  transition-duration: 0ms;
}
```

初始延迟防止意外触发。之后，同时跳过延迟和动画会让整个工具栏感觉更快。

---

## Modal

唯一保持居中的弹层。

```css
.modal {
  transform-origin: center; /* 豁免——不锚定在触发点上 */
  transition:
    opacity 250ms var(--ease-out),
    transform 250ms var(--ease-out);
}

.modal[data-starting-style],
.modal[data-ending-style] {
  opacity: 0;
  transform: scale(0.96);
}

.backdrop {
  transition: opacity 250ms var(--ease-out);
}
```

同时给背板的 opacity 做动画，让它们读起来像一个整体表面。

---

## 抽屉 / 底部面板（Drawer / sheet）

```css
.drawer {
  transform: translateY(0);
  transition: transform 500ms var(--ease-drawer);
}

.drawer[data-closed] {
  transform: translateY(100%);
}
```

Vaul 就是这样在动画进入前隐藏抽屉的。

加上拖拽就变成了手势问题——见下方**拖拽关闭**。

---

## Toast

```css
.toast {
  opacity: 1;
  transform: translateY(0);
  transition:
    opacity 400ms ease,
    transform 400ms ease;

  @starting-style {
    opacity: 0;
    transform: translateY(100%);
  }
}
```

- 用 `ease` 而非 `ease-out`，比典型 UI 稍慢：Sonner 读起来优雅，部分因为它的运动是调到匹配组件性格的，而不是匹配通用 UI 预算。
- 如果 `@starting-style` 不可用，回退到挂载标记：

```jsx
useEffect(() => { setMounted(true); }, []);
// <div data-mounted={mounted}>
```

当 toast 堆叠、列表重排时，opacity 变化必须与高度变化配合。这一对没有公式——调到感觉对了为止，然后第二天再检查一遍。

---

## 手风琴 / 折叠

```css
.content {
  overflow: hidden;
  transition:
    height 200ms var(--ease-out),
    opacity 200ms var(--ease-out);
}
```

保持短——这是少数每一帧都花 layout 的动画之一，所以长时长既昂贵又迟钝。用 JS 测量内容高度（或用提供该值的 headless 原语），而不是动画到 `auto`。

---

## 分组交错入场

用于用户偶尔看到的列表或网格——不要用于他们整天滚动浏览的列表。

```css
.item {
  opacity: 0;
  transform: translateY(8px);
  animation: fadeIn 300ms var(--ease-out) forwards;
}

.item:nth-child(2) { animation-delay: 50ms; }
.item:nth-child(3) { animation-delay: 100ms; }
.item:nth-child(4) { animation-delay: 150ms; }

@keyframes fadeIn {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
```

交错是装饰性的——播放期间绝不能阻塞交互。

---

## 长按确认

用于普通点击太容易误触的破坏性操作。

```css
.overlay {
  clip-path: inset(0 100% 0 0);
  transition: clip-path 200ms var(--ease-out); /* 松开：利落 */
}

.button:active .overlay {
  clip-path: inset(0 0 0 0);
  transition: clip-path 2s linear;             /* 按压：慢且刻意 */
}

.button:active {
  transform: scale(0.97);
}
```

这里用 `linear` 是对的——填充是进度指示器，而进度不该有缓动。

---

## 带颜色过渡的 Tab 指示器

跨 tab 列表给单个颜色做过渡的时机永远调不准。改用裁剪。

复制一份 tab 列表。把副本样式设为激活态——不同背景、不同文字颜色。裁剪副本使只有激活的 tab 显示，切换时对裁剪做动画：

```css
.tabs-active-copy {
  clip-path: inset(0 60% 0 20%); /* 由激活 tab 的位置驱动 */
  transition: clip-path 250ms var(--ease-in-out);
}
```

文字和背景一起变化、完美同步，因为它们是被揭示的同一个元素，而不是被插值的两种颜色。

---

## 滚动揭示

仅限营销页面。不要对用户每天访问的功能性 UI 做这个。

```css
.reveal {
  clip-path: inset(0 0 100% 0);
  transition: clip-path 600ms var(--ease-in-out);
}

.reveal[data-visible] {
  clip-path: inset(0 0 0 0);
}
```

用 `IntersectionObserver` 触发，或 Motion 的 `useInView` 配 `{ once: true, margin: "-100px" }`。只触发一次——每次滚过都重新动画，就是界面在和读者打架。

---

## 拖拽关闭

手势配方。用弹簧而非时长，因为用户可能中途反向。

```js
// 轻扫即关闭，不只看距离
const timeTaken = Date.now() - dragStartTime.current;
const velocity = Math.abs(swipeAmount) / timeTaken;

if (Math.abs(swipeAmount) >= SWIPE_THRESHOLD || velocity > 0.11) {
  dismiss();
}
```

```js
// 直接在被拖拽元素上设置 transform。
// 通过父元素上的 CSS 变量驱动会重算每个子元素的样式。
element.style.transform = `translateY(${distance}px)`;
```

区分好拖拽和坏拖拽的四个细节：

- **Pointer capture**：拖拽一开始即捕获指针，指针移出元素边界时拖拽继续。
- **多点触控保护**——新触摸点上 `if (isDragging) return`，否则拖拽中途换手指会让元素跳变。
- **边界阻尼**——拖过自然边缘后，走得越远移动越少。真实的物体先减速再停下。
- **摩擦力，不是墙**——允许过拖但阻力递增，而不是直接拒绝。

用弹簧收尾，被打断的拖拽能保持速度：

```js
{ type: "spring", duration: 0.5, bounce: 0.2 }
```

---

## 掩盖调不好的 crossfade

当两个状态在过渡期间可见地重叠，怎么调缓动和时长都修不好时，模糊接缝：

```css
.content {
  transition:
    filter 200ms ease,
    opacity 200ms ease;
}

.content.transitioning {
  filter: blur(2px);
  opacity: 0.7;
}
```

没有 blur 时，眼睛读出的是两个不同的对象在交换。blur 把它们混合成一次被感知的变换。保持在 20px 以下——重度 blur 很昂贵，尤其在 Safari。

---

## 不用库的编程式动画

当运动需要 JS 控制但不需要依赖时，WAAPI 给你 CSS 级性能：

```js
element.animate(
  [{ clipPath: 'inset(0 0 100% 0)' }, { clipPath: 'inset(0 0 0 0)' }],
  { duration: 1000, fill: 'forwards', easing: 'cubic-bezier(0.77, 0, 0.175, 1)' }
);
```

硬件加速、可中断、零打包成本。
