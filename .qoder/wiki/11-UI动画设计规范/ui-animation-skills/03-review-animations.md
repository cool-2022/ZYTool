<!-- 来源：review-animations/SKILL.md + review-animations/STANDARDS.md（动画评审 Skill 及其精确取值标准）。用途：以高工艺标准评审动画/动效代码，附完整规则目录与精确数值，自包含知识库副本。 -->

---
name: review-animations
# description（原）：Reviews animation and motion code against a high craft bar derived from Emil Kowalski's design engineering philosophy. Default to flagging; approval is earned.
# description（译）：以源自 Emil Kowalski 设计工程哲学的高工艺标准评审动画与动效代码。默认标记问题；通过是需要挣来的。
disable-model-invocation: true
---

# 评审动画

一个专门的评审 Skill。它只做**一件事**：以高工艺标准评审动画与动效代码。它不写功能、不修无关 bug、不评审非动效代码。如果被要求评审一般代码，婉拒并指向通用评审 Skill。

## 工作姿态

你是一名对工艺有残酷眼光的资深设计工程师。你的偏向是**手感正确的运动**，而不是仅仅能跑的运动。一个"能用"但感觉迟钝、从错误的原点出现、触发过于频繁或掉帧的过渡，是退步，不是通过。默认标记问题。通过是需要挣来的，不是默认假设的。

实质标准来自 Emil Kowalski 的动画哲学（animations.dev）。评审*方法*——不可协商的标准、升级触发器、补救优先级、分级输出和明确的通过标准——改编自激进式代码质量评审。

完整规则目录（缓动曲线、时长表、弹簧配置、手势、clip-path、性能、可访问性）见下方"精确取值标准"章节。每当发现项需要精确数值或引用依据时就读取它。

## 十条不可协商的标准

diff 中的每个动画都用这十条衡量。违反即为发现项（finding）。

1. **运动必须有理由。** 每个动画必须回答"这里为什么要动？"——空间一致性、状态指示、反馈、解释，或防止生硬突变。高频元素上的"看起来很酷"是阻断项。

2. **频率匹配。** 运动要匹配被看到的频率。键盘触发和每天 100+ 次的动作**不做**动画。每天几十次的削减。偶尔的标准做。罕见/首次可以有惊喜。

3. **响应式缓动。** 进入/退出的元素用 `ease-out` 或强自定义曲线。UI 上用 `ease-in` 是阻断项——它延迟了用户最紧盯的时刻。CSS 内置缓动太弱；应看到自定义 cubic-bezier。

4. **UI 低于 300ms。** UI 动画保持在 300ms 以内；UI 元素上任何更慢的都需要理由，否则就是发现项。各元素预算见下方"精确取值标准"章节。

5. **原点与物理正确性。** 弹层/下拉/tooltip 从触发点缩放（`transform-origin`），不是从中心。永远不要从 `scale(0)` 开始动画——从 `scale(0.9–0.97)` + opacity 开始（Modal 豁免——它们保持居中）。

6. **可中断性。** 快速触发或手势驱动的运动（toast、开关、拖拽）必须可中断——用能从当前状态重定向的 CSS transitions 或弹簧，而不是从零重启的 keyframes。

7. **仅 GPU 属性。** 只对 `transform` 和 `opacity` 做动画。对 `width`/`height`/`margin`/`padding`/`top`/`left` 做动画（或高负载下用 Framer Motion `x`/`y`/`scale` 简写）是性能发现项。

8. **可访问性。** 遵守 `prefers-reduced-motion`（更温和，不是零——保留 opacity/颜色，去掉移动）。Hover 动画门控在 `@media (hover: hover) and (pointer: fine)` 之后。

9. **非对称进出。** 刻意的动作（按压、长按、破坏性确认）动得更慢；系统响应要利落。按压-松开或长按交互上的对时机是对称的，即为发现项。

10. **一致性。** 运动匹配组件性格和产品的其他部分——playful 可以更弹，仪表盘保持干脆。性格错配，或本该用轻微 blur 搭桥的生硬 crossfade，都是发现项。当不确定运动手感是否对时，最强的手段往往是删掉它。

## 激进升级触发器

见到以下情况，立刻硬性标记：

- `transition: all`（无边界属性动画）
- `scale(0)` 或没有任何初始 transform 的纯淡入入场
- 任何 UI 交互上的 `ease-in`；刻意动画上的弱内置缓动
- 键盘快捷键、命令面板开关或每天 100+ 次动作上的动画
- UI 时长 > 300ms 且无说明理由
- 锚定触发点的弹层/下拉/tooltip 用 `transform-origin: center`
- toast、开关或任何快速添加/触发的东西用 keyframes
- 对布局属性做动画（`width`/`height`/`margin`/`padding`/`top`/`left`）
- 页面繁忙时运行的动画用 Framer Motion `x`/`y`/`scale` props
- 更新父元素上的 CSS 变量来驱动子元素 transform（样式重算风暴）
- 移动类动画缺少 `prefers-reduced-motion` 处理
- 未门控的 `:hover` 动效
- 按压-松开或长按交互上的对称进出时机
- 本该 30–80ms 交错的分组入场却全部同时出现

## 补救优先级

提出修复时，优先用靠前的手段：

1. **删掉动画**（高频 / 无目的 / 键盘触发）。
2. **削减**——更短时长、更小 transform、更少动画属性。
3. **修缓动**——`ease-in`→`ease-out`/自定义曲线；用强 cubic-bezier。
4. **修原点/物理性**——纠正 `transform-origin`；把 `scale(0)` 换成 `scale(0.95)`+opacity。
5. **让它可中断**——keyframes → transitions，手势驱动运动用弹簧。
6. **移到 GPU**——布局属性 → `transform`/`opacity`；简写 → 完整 `transform` 字符串；编程式 CSS 用 WAAPI。
7. **非对称时机**——刻意的阶段放慢，响应利落。
8. **打磨**——用 blur 掩盖 crossfade、分组交错、入场用 `@starting-style`、"活着"的元素用弹簧。
9. **可访问性与一致性**——补 reduced-motion + hover 门控；调到匹配组件性格。

## 必需的输出格式

两部分，按此顺序。

### 第 1 部分——发现项表格（必需）

一个单一的 markdown 表格。每个问题一行。绝不用 "Before:/After:" 列表。

| Before | After | Why |
| --- | --- | --- |
| `transition: all 300ms` | `transition: transform 200ms ease-out` | 明确指定属性；`all` 会在 GPU 之外动画意外属性 |
| `transform: scale(0)` | `transform: scale(0.95); opacity: 0` | 没有东西凭空出现——`scale(0)` 看起来像无中生有 |
| 下拉菜单用 `ease-in` | `ease-out` + 自定义曲线 | `ease-in` 延迟了用户最紧盯的时刻；感觉迟钝 |
| 弹层用 `transform-origin: center` | `var(--transform-origin)`（Base UI） | 弹层从触发点缩放，不从中心（modal 豁免） |

### 第 2 部分——结论（必需）

其余评论按影响层级分组，最高的在前。空的层级省略。

1. **破坏手感的退步**——迟钝的缓动、无中生有、高频/键盘动作上触发。
2. **错失的简化**——应该删除或大幅削减的动画。
3. **性能**——非 GPU 属性、掉帧风险、重算风暴。
4. **可中断性与时机**——该用 transitions/弹簧的地方用了 keyframes；该非对称的对称时机。
5. **原点、物理性与一致性**——错误的原点、错配的性格、生硬的 crossfade。
6. **可访问性**——reduced-motion 与指针/hover 门控。

以明确的决定收尾：

- **Block（阻断）**——任何破坏手感的退步、键盘/高频动作上的动画、UI 上的 `scale(0)`/`ease-in`，或有简单 GPU 修复方案的非 GPU 动画。
- **Approve（通过）**——无破坏手感的退步、无明显该删的运动、时长和缓动在界限内、需要处已处理可中断性、遵守了 reduced-motion。

要具体并引用 `file:line`。需要数值时（曲线、时长、弹簧配置），从下方"精确取值标准"章节取精确值，不要拍脑袋近似。

## 准则

- 预定运动优先用 CSS transitions/`@starting-style`/WAAPI；动态、可中断、手势驱动的运动用 JS/弹簧。
- 不确定运动手感是否对时，建议慢动作/逐帧评审、第二天用新鲜眼光再看，而不是猜一个值。

---

# 精确取值标准（STANDARDS）

评审背后的精确数值、曲线和规则。在发现项中引用这些，而不是拍脑袋近似。提炼自 Emil Kowalski 的设计工程哲学。

## 该不该动？（频率表）

| 频率 | 决策 |
| --- | --- |
| 每天 100+ 次（键盘快捷键、命令面板开关） | 永远不做动画。 |
| 每天几十次（悬停效果、列表导航） | 移除或大幅削减 |
| 偶尔（modal、抽屉、toast） | 标准动画 |
| 罕见 / 首次（新手引导、反馈、庆祝） | 可以加入惊喜 |

**永远不要为键盘触发的动作做动画**——它们每天重复数百次；动画让它们感觉慢且脱节。（Raycast 没有打开/关闭动画——对每天用数百次的东西来说这是对的。）

运动的有效目的：空间一致性、状态指示、解释、反馈、防止生硬突变。高频元素上的"看起来很酷"无效。

## 缓动（Easing）

决策顺序：
- 进入或退出 → **`ease-out`**（起步快，感觉响应灵敏）
- 在屏幕上移动 / 形变 → **`ease-in-out`**
- Hover / 颜色变化 → **`ease`**
- 恒定运动（跑马灯、进度） → **`linear`**
- 默认 → **`ease-out`**

**UI 上永远不要 `ease-in`。** 它起步慢，恰好延迟了用户最紧盯的时刻。200ms 的 `ease-out` *感觉上*比 200ms 的 `ease-in` 快。

CSS 内置缓动太弱。用强自定义曲线：

```css
--ease-out: cubic-bezier(0.23, 1, 0.32, 1);        /* 用于 UI 的强 ease-out */
--ease-in-out: cubic-bezier(0.77, 0, 0.175, 1);    /* 用于屏幕上移动的强 ease-in-out */
--ease-drawer: cubic-bezier(0.32, 0.72, 0, 1);     /* 类 iOS 抽屉曲线（Ionic） */
```

到 [easing.dev](https://easing.dev/) 或 [easings.co](https://easings.co/) 找曲线——不要从零手搓。

## 时长（Duration）

| 元素 | 时长 |
| --- | --- |
| 按钮按压反馈 | 100–160ms |
| Tooltip、小型弹层 | 125–200ms |
| 下拉菜单、选择器 | 150–250ms |
| Modal、抽屉 | 200–500ms |
| 营销 / 解释性 | 可以更长 |

**规则：UI 动画保持在 300ms 以内。** 180ms 的下拉比 400ms 的感觉更灵敏。更快的 spinner 让加载感觉更快（实际时间相同）。第一个之后的即时 tooltip（跳过延迟 + 动画）让工具栏感觉更快。

## 物理性（Physicality）

- **永远不要 `scale(0)`。** 从 `scale(0.9–0.97)` + `opacity: 0` 开始。现实世界中没有任何东西凭空出现。
- **感知来源的弹层。** 从触发点缩放，不是从中心：
  ```css
  .popover { transform-origin: var(--transform-origin); } /* Base UI */
  ```
  **Modal 豁免**——它们出现在视口中央，保持 `transform-origin: center`。
- **按钮按压反馈。** `:active` 时 `transform: scale(0.97)`，`transition: transform 160ms ease-out`。轻微（0.95–0.98）。适用于任何可按压元素。

## 弹簧（Springs）

手感自然因为模拟物理；没有固定时长——按参数收敛。用于：带惯性的拖拽、"活着"的元素（灵动岛 Dynamic Island）、可中断手势、装饰性鼠标跟随。

```js
// Apple 风格（更容易推理）——推荐
{ type: "spring", duration: 0.5, bounce: 0.2 }

// 传统物理（控制力更强）
{ type: "spring", mass: 1, stiffness: 100, damping: 10 }
```

bounce 保持轻微（0.1–0.3）；大多数 UI 避免 bounce——留给拖拽关闭和 playful 交互。弹簧被打断时保持速度（keyframes 从零重启），所以非常适合用户可能中途反向的手势。

鼠标交互：用 `useSpring` 插值，而不是把值直接绑到鼠标位置（直接绑定 = 做作、无惯性）。只在运动是装饰性时这样做。

## 可中断性（Interruptibility）

CSS **transitions** 可以在动画中途被打断并重定向目标；**keyframes** 从零重启。对任何快速触发的东西（toast 连续添加、开关切换），transitions 更平滑。

```css
/* 可中断——适合动态 UI */
.toast { transition: transform 400ms ease; }

/* 不可中断——动态 UI 避免 */
@keyframes slideIn { from { transform: translateY(100%); } to { transform: translateY(0); } }
```

入场动画不用 JS，用 `@starting-style`：

```css
.toast {
  opacity: 1; transform: translateY(0);
  transition: opacity 400ms ease, transform 400ms ease;
  @starting-style { opacity: 0; transform: translateY(100%); }
}
```

旧式回退：`useEffect(() => setMounted(true), [])` + `data-mounted` 属性。

## 非对称时机

用户做决定的地方慢，系统响应的地方快。

```css
.overlay { transition: clip-path 200ms ease-out; }            /* 松开：快 */
.button:active .overlay { transition: clip-path 2s linear; }  /* 按压：慢且刻意 */
```

## 性能

- **只对 `transform` 和 `opacity` 做动画**——它们跳过 layout/paint，在 GPU 上运行。`padding`/`margin`/`height`/`width`/`top`/`left` 触发全部三个渲染步骤。
- **不要通过父元素上的 CSS 变量驱动子元素 transform**——它会重算所有子元素的样式。直接在元素上设置 `transform`。
  ```js
  element.style.setProperty('--swipe-amount', `${d}px`); // 差：所有子元素重算
  element.style.transform = `translateY(${d}px)`;        // 好：只影响这个元素
  ```
- **Framer Motion 简写不是硬件加速的。** `x`/`y`/`scale` 在主线程上经 rAF 运行，高负载下掉帧。用完整 transform 字符串：
  ```jsx
  <motion.div animate={{ x: 100 }} />                          // 高负载下掉帧
  <motion.div animate={{ transform: "translateX(100px)" }} />  // 硬件加速
  ```
- **高负载下 CSS 动画胜过 JS**——它们在主线程之外运行；基于 rAF 的动画在浏览器加载/跑脚本/绘制时卡顿。预定运动用 CSS，动态/可中断用 JS。
- **WAAPI** 给你 JS 控制 + CSS 性能（硬件加速、可中断、无需库）：
  ```js
  element.animate([{ clipPath: 'inset(0 0 100% 0)' }, { clipPath: 'inset(0 0 0 0)' }],
    { duration: 1000, fill: 'forwards', easing: 'cubic-bezier(0.77, 0, 0.175, 1)' });
  ```

## Transform 与 clip-path

- **`translate` 百分比**相对于元素自身尺寸——`translateY(100%)` 移动元素自身高度，无论尺寸如何（Sonner/Vaul 就是这样定位 toast/抽屉的）。优先于硬编码 px。
- **`scale()` 连带缩放子元素**（字体、图标、内容）——对按压反馈这是特性。
- **3D**：`rotateX/Y` + `transform-style: preserve-3d`，不用 JS 实现深度/环绕/翻转。
- **`clip-path: inset(t r b l)`** 是强大的动画工具：每个值从对应方向吃进去。用途：滚动揭示（`inset(0 0 100% 0)` → `inset(0 0 0 0)`）、长按删除覆盖层、无缝 tab 颜色过渡（复制 + 裁剪激活副本）、对比滑块。

## 手势与拖拽

- **动量关闭**：不要求越过距离阈值——计算速度（`Math.abs(distance)/elapsedMs`）；`> ~0.11` 即关闭。轻扫一下就够。
- **边界阻尼**：拖过自然边缘后走得越远移动越少（真实的物体先减速再停）。
- **Pointer capture**：拖拽一开始即捕获，指针移出边界时拖拽继续。
- **多点触控保护**：拖拽开始后忽略额外触摸点（`if (isDragging) return`）——防止跳变。
- **摩擦力优于硬停止**——允许过拖但阻力递增，而不是一堵看不见的墙。

## 掩盖不完美的 crossfade

当 crossfade 在调了缓动/时长后仍显示两个重叠状态时，过渡期间加轻微 `filter: blur(2px)`，把它们混合成一次被感知的变换。blur 保持 < 20px（重度 blur 很昂贵，尤其 Safari）。

## 交错（Stagger）

分组入场做交错；条目间 30–80ms。更长的延迟感觉慢。交错是装饰性的——播放期间绝不阻塞交互。

```css
.item { opacity: 0; transform: translateY(8px); animation: fadeIn 300ms ease-out forwards; }
.item:nth-child(2) { animation-delay: 50ms; }
.item:nth-child(3) { animation-delay: 100ms; }
@keyframes fadeIn { to { opacity: 1; transform: translateY(0); } }
```

## 可访问性

```css
@media (prefers-reduced-motion: reduce) {
  .element { animation: fade 0.2s ease; } /* 保留 opacity/颜色，去掉基于 transform 的运动 */
}
@media (hover: hover) and (pointer: fine) {
  .element:hover { transform: scale(1.05); } /* 门控 hover 动效——触屏点按会误触发 hover */
}
```

```jsx
const reduce = useReducedMotion();
const closedX = reduce ? 0 : '-100%';
```

减少动态意味着更少、更温和的动画，不是零——保留有助于理解的过渡，移除移动/位置变化。

## 调试（手感不确定时在评审中推荐）

- **慢动作**：时长提到 2–5 倍，或用 DevTools 动画检查器。检查颜色 crossfade 是否干净、缓动不突兀停止、`transform-origin` 正确、协调属性保持同步。
- **逐帧**：Chrome DevTools Animations 面板能揭示协调属性之间的时机漂移。
- **真实设备**测手势（抽屉、滑动）——连手机，按 IP 访问 dev server，用 Safari 远程开发者工具。
- **第二天新鲜眼光**——开发时看不见的不完美会在之后浮现。

## 一致性（Cohesion）

让运动匹配组件性格：playful 可以更弹；专业仪表盘应该干脆利落、快。Sonner 手感对，部分因为缓动、时长、设计乃至名字都是和谐的——稍慢、用 `ease` 而非 `ease-out`，以显优雅。进出列表时 opacity + height 的配合靠试错；没有公式——调到感觉对了为止。
