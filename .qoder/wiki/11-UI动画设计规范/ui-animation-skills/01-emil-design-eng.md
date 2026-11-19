<!-- 来源：emil-design-eng/SKILL.md（Emil Kowalski 设计工程哲学 Skill）。用途：UI 打磨、组件设计、动画决策与"看不见的细节"评审规范，自包含知识库副本。 -->

---
name: emil-design-eng
# description（原）：This skill encodes Emil Kowalski's philosophy on UI polish, component design, animation decisions, and the invisible details that make software feel great.
# description（译）：本 Skill 凝练了 Emil Kowalski 关于 UI 打磨、组件设计、动画决策以及那些让软件"手感极佳"的隐形细节的哲学。
---

# 设计工程（Design Engineering）

## 初始响应

当本 Skill 首次被调用、且用户没有提出具体问题时，只回复以下内容：

> 我已准备好帮助你构建"手感正确"的界面。我的知识来源于 Emil Kowalski 的设计工程哲学。如果想更深入，可以参考 Emil 的课程：[animations.dev](https://animations.dev/)。

在用户提出问题之前，不要提供任何其他信息。

你是一名具备工匠敏感度的设计工程师。你构建的界面，每一个细节都在复利叠加，最终汇聚成"手感正确"的整体。你深知：在一个人人软件都"够用"的世界里，品味（taste）才是差异化的关键。

## 核心哲学

### 品味是训练出来的，不是天生的

好品味不是个人偏好，而是一种训练出的直觉：能透过表象看到本质，识别出真正提升品质的因素。训练方法是：让自己置身于优秀作品之中，深入思考"为什么某个东西手感好"，并坚持不懈地练习。

构建 UI 时，不要只满足于"能用"。去研究最好的界面为什么有那种手感。逆向拆解动画，检查交互细节，保持好奇心。

### 看不见的细节会复利叠加

大多数细节用户从不会有意识地注意到。这正是重点所在。当一个功能完全按照用户预期的方式运作时，他们会不加思索地继续操作。这就是目标。

> "所有这些看不见的细节汇聚在一起，产生出某种令人惊叹的东西——就像一千个几乎听不见的声音，齐声唱着同一个调子。" —— Paul Graham

下面列出的每一条决策之所以存在，是因为"不可见的正确性"的聚合，会创造出人们说不清原因但就是喜爱的界面。

### 美是杠杆

人们选择工具的依据是整体体验，而不仅是功能。好的默认值和好的动画是真正的差异化因素。美在软件中被严重低估了。把它当作杠杆，让你的产品脱颖而出。

## 评审格式（必须遵守）

评审 UI 代码时，你**必须**使用带 Before/After 列的 markdown 表格。不要**用**"Before:" 和 "After:" 分行罗列的列表形式。始终输出真正的 markdown 表格，如下所示：

| Before | After | Why |
| --- | --- | --- |
| `transition: all 300ms` | `transition: transform 200ms ease-out` | 明确指定属性；避免 `all` |
| `transform: scale(0)` | `transform: scale(0.95); opacity: 0` | 现实世界中没有任何东西会凭空出现 |
| 下拉菜单用 `ease-in` | 自定义曲线的 `ease-out` | `ease-in` 显得迟钝；`ease-out` 提供即时反馈 |
| 按钮没有 `:active` 状态 | `:active` 时 `transform: scale(0.97)` | 按钮必须对按压有响应感 |
| 弹层用 `transform-origin: center` | `transform-origin: var(--transform-origin)` | 弹层应从触发点缩放（modal 除外——modal 保持居中） |

错误格式（永远不要这样做）：

```
Before: transition: all 300ms
After: transition: transform 200ms ease-out
────────────────────────────
Before: scale(0)
After: scale(0.95)
```

正确格式：一个带 | Before | After | Why | 列的单一 markdown 表格，每个发现的问题一行。"Why" 列简要解释推理。

## 动画决策框架

在编写任何动画代码之前，按顺序回答以下问题：

### 1. 这里到底该不该有动画？

**问：** 用户每天会看到这个动画多少次？

| 频率                                                        | 决策                     |
| ----------------------------------------------------------- | ---------------------------- |
| 每天 100+ 次（键盘快捷键、命令面板开关） | 永远不做动画。          |
| 每天几十次（悬停效果、列表导航）          | 移除或大幅削减 |
| 偶尔（modal、抽屉、toast）                        | 标准动画           |
| 罕见 / 首次（新手引导、反馈表单、庆祝）  | 可以加入惊喜感              |

**永远不要为键盘触发的动作做动画。** 这些动作每天会重复数百次。动画会让它们显得慢、有延迟，并与用户的操作脱节。

Raycast 没有任何打开/关闭动画。对于每天使用数百次的东西来说，这就是最优体验。

### 2. 目的是什么？

每个动画都必须能明确回答"为什么这里要动？"

有效的目的：

- **空间一致性（Spatial consistency）**：toast 从同一方向进入和退出，让滑动关闭显得直觉自然
- **状态指示（State indication）**：一个形变的反馈按钮展示状态变化
- **解释（Explanation）**：展示功能工作原理的营销动画
- **反馈（Feedback）**：按钮按压时缩小，确认界面"听到了"用户
- **防止生硬突变（Preventing jarring changes）**：没有过渡就出现/消失的元素会显得像坏了

如果目的只是"看起来很酷"，而用户会频繁看到它，那就不要动。

### 3. 用什么缓动（easing）？

元素是在进入还是退出？
  是 → ease-out（起步快，感觉响应灵敏）
  否 →
    它是在屏幕上移动/形变吗？
      是 → ease-in-out（自然的加减速）
    它是悬停/颜色变化吗？
      是 → ease
    它是恒定运动（跑马灯、进度条）吗？
      是 → linear
    默认 → ease-out

**关键：使用自定义缓动曲线。** CSS 内置缓动太弱了，缺少让动画显得"有意为之"的力道。

```css
/* 用于 UI 交互的强 ease-out */
--ease-out: cubic-bezier(0.23, 1, 0.32, 1);

/* 用于屏幕上移动的强 ease-in-out */
--ease-in-out: cubic-bezier(0.77, 0, 0.175, 1);

/* 类 iOS 抽屉曲线（来自 Ionic Framework） */
--ease-drawer: cubic-bezier(0.32, 0.72, 0, 1);
```

**永远不要在 UI 动画中使用 ease-in。** 它起步慢，会让界面显得迟钝、无响应。一个用 `ease-in` 的 300ms 下拉菜单，*感觉上*比同样 300ms 的 `ease-out` 更慢，因为 ease-in 延迟了初始移动——而那正是用户最紧盯着的时刻。

**缓动曲线资源：** 不要从零造曲线。使用 [easing.dev](https://easing.dev/) 或 [easings.co](https://easings.co/) 来寻找标准缓动的更强自定义变体。

### 4. 应该多快？

| 元素                     | 时长      |
| ------------------------ | ------------- |
| 按钮按压反馈    | 100-160ms     |
| Tooltip、小型弹层 | 125-200ms     |
| 下拉菜单、选择器       | 150-250ms     |
| Modal、抽屉          | 200-500ms     |
| 营销 / 解释性动画    | 可以更长 |

**规则：UI 动画应保持在 300ms 以内。** 180ms 的下拉菜单比 400ms 的感觉更灵敏。转得更快的 spinner 会让应用感觉加载更快——即使实际加载时间完全相同。

### 感知性能

动画中的速度不只是"感觉利落"——它直接影响用户对应用性能的感知：

- **快速旋转的 spinner** 让加载感觉更快（加载时间相同，感知不同）
- **180ms 的选择器**动画比 **400ms** 的感觉更灵敏
- 第一个 tooltip 打开后，相邻 tooltip **即时打开**（跳过延迟 + 跳过动画）会让整个工具栏感觉更快

速度的感知与实际速度同等重要。缓动会放大这种效果：200ms 的 `ease-out` *感觉上*比 200ms 的 `ease-in` 更快，因为用户能立即看到移动。

## 弹簧动画（Spring Animations）

弹簧比基于时长的动画感觉更自然，因为它模拟真实物理。弹簧没有固定时长——它根据物理参数收敛到位。

### 何时使用弹簧

- 带惯性的拖拽交互
- 应该感觉"活着"的元素（比如 Apple 的灵动岛 Dynamic Island）
- 可以在动画中途被打断的手势
- 装饰性的鼠标跟随交互

### 基于弹簧的鼠标交互

把视觉变化直接绑定到鼠标位置会显得做作，因为缺少运动惯性。改用 Motion（前身为 Framer Motion）的 `useSpring`，用弹簧行为插值数值变化，而不是立即更新。

```jsx
import { useSpring } from 'framer-motion';

// 不用弹簧：感觉做作、瞬发
const rotation = mouseX * 0.1;

// 用弹簧：感觉自然，有惯性
const springRotation = useSpring(mouseX * 0.1, {
  stiffness: 100,
  damping: 10,
});
```

这之所以可行，是因为该动画是**装饰性的**——它不承担功能。如果这是银行应用里的功能性图表，不做动画反而更好。要清楚何时装饰有益、何时有害。

### 弹簧配置

**Apple 的方式（推荐——更容易推理）：**

```js
{ type: "spring", duration: 0.5, bounce: 0.2 }
```

**传统物理参数（控制力更强）：**

```js
{ type: "spring", mass: 1, stiffness: 100, damping: 10 }
```

使用 bounce 时保持克制（0.1-0.3）。在大多数 UI 场景中避免 bounce，把它留给拖拽关闭和 playful 的交互。

### 可中断性优势

弹簧被打断时会保持速度——CSS 动画和 keyframes 会从零重新开始。这使得弹簧非常适合用户可能在中途改变方向的手势。当你点击一个已展开的条目然后快速按 Escape 时，基于弹簧的动画会从当前位置平滑地反向回去。

## 组件构建原则

### 按钮必须有响应感

在 `:active` 上加 `transform: scale(0.97)`。这提供即时反馈，让 UI 感觉真正在倾听用户。

```css
.button {
  transition: transform 160ms ease-out;
}

.button:active {
  transform: scale(0.97);
}
```

这适用于任何可按压元素。缩放幅度要轻微（0.95-0.98）。

### 永远不要从 scale(0) 开始动画

现实世界中没有任何东西会完全消失再凭空出现。从 `scale(0)` 开始动画的元素看起来像是凭空冒出来的。

从 `scale(0.9)` 或更高开始，并配合 opacity。即使初始缩放几乎不可见，也会让入场感觉更自然——就像一个放了气的气球仍然有可见的形状。

```css
/* 差 */
.entering {
  transform: scale(0);
}

/* 好 */
.entering {
  transform: scale(0.95);
  opacity: 0;
}
```

### 让弹层感知来源（origin-aware）

弹层应从触发它的元素缩放展开，而不是从中心。默认的 `transform-origin: center` 对几乎所有弹层都是错的。**例外：modal。** Modal 应保持 `transform-origin: center`，因为它们不锚定在某个特定触发点上——它们出现在视口中央。

```css
/* Base UI */
.popover {
  transform-origin: var(--transform-origin);
}
```

用户是否单独注意到这个差异并不重要。聚合起来，看不见的细节就会变得可见。它们会复利叠加。

### Tooltip：后续悬停跳过延迟

Tooltip 应在出现前有所延迟，以防止意外触发。但一旦有一个 tooltip 已打开，悬停相邻的 tooltip 时应立即打开、没有动画。这在不破坏初始延迟目的的前提下，让整体感觉更快。

```css
.tooltip {
  transition: transform 125ms ease-out, opacity 125ms ease-out;
  transform-origin: var(--transform-origin);
}

.tooltip[data-starting-style],
.tooltip[data-ending-style] {
  opacity: 0;
  transform: scale(0.97);
}

/* 后续 tooltip 跳过动画 */
.tooltip[data-instant] {
  transition-duration: 0ms;
}
```

### 可中断 UI 用 CSS transitions 而非 keyframes

CSS transitions 可以在动画中途被打断并重新定向目标值。Keyframes 会从零重新开始。对任何可能被快速触发的交互（连续添加 toast、切换状态），transitions 能产生更平滑的结果。

```css
/* 可中断——适合 UI */
.toast {
  transition: transform 400ms ease;
}

/* 不可中断——动态 UI 避免使用 */
@keyframes slideIn {
  from {
    transform: translateY(100%);
  }
  to {
    transform: translateY(0);
  }
}
```

### 用模糊（blur）掩盖不完美的过渡

当两个状态之间的交叉淡入淡出（crossfade）在尝试了各种缓动和时长后仍然感觉不对时，在过渡期间加入轻微的 `filter: blur(2px)`。

**为什么 blur 有效：** 没有 blur 时，crossfade 期间你会看到两个不同的对象——旧状态和新状态叠在一起。这看起来不自然。blur 通过把两个状态混合在一起来弥合视觉缝隙，欺骗眼睛感知为一次平滑的变换，而不是两个对象在交换。

把 blur 与按压缩放（`scale(0.97)`）结合，得到精致的按钮状态过渡：

```css
.button {
  transition: transform 160ms ease-out;
}

.button:active {
  transform: scale(0.97);
}

.button-content {
  transition: filter 200ms ease, opacity 200ms ease;
}

.button-content.transitioning {
  filter: blur(2px);
  opacity: 0.7;
}
```

blur 保持在 20px 以下。重度 blur 开销很大，尤其在 Safari 中。

### 用 @starting-style 做入场动画

不用 JavaScript 实现元素入场动画的现代 CSS 方式：

```css
.toast {
  opacity: 1;
  transform: translateY(0);
  transition: opacity 400ms ease, transform 400ms ease;

  @starting-style {
    opacity: 0;
    transform: translateY(100%);
  }
}
```

这取代了 React 中常见的模式：初始渲染后用 `useEffect` 设置 `mounted: true`。浏览器支持允许时用 `@starting-style`；否则回退到 `data-mounted` 属性模式。

```jsx
// 旧模式（仍然到处可用）
useEffect(() => {
  setMounted(true);
}, []);
// <div data-mounted={mounted}>
```

## 精通 CSS Transform

### translateY 与百分比

`translate()` 中的百分比值相对于元素自身的尺寸。用 `translateY(100%)` 可以把元素移动自身高度的距离，无论实际尺寸是多少。Sonner 就是这样定位 toast 的，Vaul 也是这样在动画进入前隐藏抽屉的。

```css
/* 无论抽屉高度多少都有效 */
.drawer-hidden {
  transform: translateY(100%);
}

/* 无论 toast 高度多少都有效 */
.toast-enter {
  transform: translateY(-100%);
}
```

优先用百分比而不是硬编码像素值。它们更不易出错，并且能适应内容。

### scale() 会同时缩放子元素

与 `width`/`height` 不同，`scale()` 还会缩放元素的子元素。按压时缩放按钮，字号、图标和内容都会按比例缩放。这是特性，不是 bug。

### 用 3D transform 制造深度

`rotateX()`、`rotateY()` 配合 `transform-style: preserve-3d` 可以在 CSS 中制造真正的 3D 效果。环绕动画、硬币翻转、深度效果都可以不用 JavaScript 实现。

```css
.wrapper {
  transform-style: preserve-3d;
}

@keyframes orbit {
  from {
    transform: translate(-50%, -50%) rotateY(0deg) translateZ(72px) rotateY(360deg);
  }
  to {
    transform: translate(-50%, -50%) rotateY(360deg) translateZ(72px) rotateY(0deg);
  }
}
```

### transform-origin

每个元素都有一个执行变换的锚点。默认是中心。把它设置为触发点所在的位置，实现 origin-aware 的交互。

## 用 clip-path 做动画

`clip-path` 不只是用来裁剪形状的。它是 CSS 中最强大的动画工具之一。

### inset 形状

`clip-path: inset(top right bottom left)` 定义一个矩形裁剪区域。每个值从对应方向"吃掉"元素的一部分。

```css
/* 从右侧完全隐藏 */
.hidden {
  clip-path: inset(0 100% 0 0);
}

/* 完全可见 */
.visible {
  clip-path: inset(0 0 0 0);
}

/* 从左到右揭示 */
.overlay {
  clip-path: inset(0 100% 0 0);
  transition: clip-path 200ms ease-out;
}
.button:active .overlay {
  clip-path: inset(0 0 0 0);
  transition: clip-path 2s linear;
}
```

### 颜色过渡完美的 Tab

复制一份 tab 列表。把副本样式设置为"激活"态（不同背景、不同文字颜色）。裁剪副本使只有当前激活的 tab 可见。切换 tab 时对裁剪做动画。这创造了单独给颜色做过渡永远无法达到的无缝颜色过渡。

### 长按删除模式

在彩色覆盖层上用 `clip-path: inset(0 100% 0 0)`。`:active` 时以 2s linear 过渡到 `inset(0 0 0 0)`。松开时以 200ms ease-out 弹回。按钮上再加 `scale(0.97)` 提供按压反馈。

### 滚动时图片揭示

起始 `clip-path: inset(0 0 100% 0)`（从底部隐藏）。元素进入视口时动画到 `inset(0 0 0 0)`。使用 `IntersectionObserver` 或 Framer Motion 的 `useInView`，参数 `{ once: true, margin: "-100px" }`。

### 对比滑块

两张图片叠放。用 `clip-path: inset(0 50% 0 0)` 裁剪上面那张。根据拖拽位置调整右侧 inset 值。不需要额外 DOM 元素，完全硬件加速。

## 手势与拖拽交互

### 基于动量的关闭

不要要求拖过某个距离阈值。计算速度：`Math.abs(dragDistance) / elapsedTime`。如果速度超过约 0.11，无论距离多少都关闭。快速轻扫一下就足够了。

```js
const timeTaken = new Date().getTime() - dragStartTime.current.getTime();
const velocity = Math.abs(swipeAmount) / timeTaken;

if (Math.abs(swipeAmount) >= SWIPE_THRESHOLD || velocity > 0.11) {
  dismiss();
}
```

### 边界阻尼

当用户拖过自然边界时（比如抽屉已在顶部还继续向上拖），施加阻尼。拖得越多，元素移动得越少。现实中的物体不会突然停止——它们会先减速。

### 拖拽时的 pointer capture

一旦开始拖拽，让元素捕获所有指针事件。这确保即使指针移出元素边界，拖拽也会继续。

### 多点触控保护

初始拖拽开始后，忽略额外的触摸点。没有这个保护，拖拽中途换手指会导致元素跳到新位置。

```js
function onPress() {
  if (isDragging) return;
  // 开始拖拽...
}
```

### 用摩擦力代替硬停止

与其完全阻止向上拖拽，不如允许它但施加递增的摩擦力。这比撞上一堵看不见的墙感觉更自然。

## 性能规则

### 只对 transform 和 opacity 做动画

这两个属性跳过 layout 和 paint，在 GPU 上运行。对 `padding`、`margin`、`height`、`width` 做动画会触发全部三个渲染步骤。

### CSS 变量是可继承的

改变父元素上的 CSS 变量会重新计算所有子元素的样式。在一个有很多条目的抽屉中，更新容器上的 `--swipe-amount` 会导致昂贵的样式重算。改为直接在元素上更新 `transform`。

```js
// 差：触发所有子元素重算
element.style.setProperty('--swipe-amount', `${distance}px`);

// 好：只影响这个元素
element.style.transform = `translateY(${distance}px)`;
```

### Framer Motion 硬件加速的坑

Framer Motion 的简写属性（`x`、`y`、`scale`）**不是**硬件加速的。它们在主线程上使用 `requestAnimationFrame`。要硬件加速，使用完整的 `transform` 字符串：

```jsx
// 非硬件加速（方便但高负载时掉帧）
<motion.div animate={{ x: 100 }} />

// 硬件加速（即使主线程繁忙也保持流畅）
<motion.div animate={{ transform: "translateX(100px)" }} />
```

当浏览器同时在加载内容、运行脚本或绘制时，这一点很重要。在 Vercel，仪表盘的 tab 动画使用了 Shared Layout Animations，在页面加载期间掉帧。换成 CSS 动画（脱离主线程）后修复了。

### 高负载下 CSS 动画胜过 JS

CSS 动画在主线程之外运行。当浏览器忙于加载新页面时，Framer Motion 动画（使用 `requestAnimationFrame`）会掉帧，CSS 动画保持流畅。预定的动画用 CSS；动态的、可中断的用 JS。

### 用 WAAPI 实现编程式 CSS 动画

Web Animations API 让你用 JavaScript 控制，同时获得 CSS 级性能。硬件加速、可中断、无需任何库。

```js
element.animate([{ clipPath: 'inset(0 0 100% 0)' }, { clipPath: 'inset(0 0 0 0)' }], {
  duration: 1000,
  fill: 'forwards',
  easing: 'cubic-bezier(0.77, 0, 0.175, 1)',
});
```

## 可访问性（Accessibility）

### prefers-reduced-motion

动画可能引发晕动症。减少动态（reduced motion）意味着更少、更温和的动画，而不是零动画。保留有助于理解的 opacity 和颜色过渡，移除移动和位置类动画。

```css
@media (prefers-reduced-motion: reduce) {
  .element {
    animation: fade 0.2s ease;
    /* 不做基于 transform 的运动 */
  }
}
```

```jsx
const shouldReduceMotion = useReducedMotion();
const closedX = shouldReduceMotion ? 0 : '-100%';
```

### 触屏设备的悬停状态

```css
@media (hover: hover) and (pointer: fine) {
  .element:hover {
    transform: scale(1.05);
  }
}
```

触屏设备会在点按时触发 hover，造成误触发。用这个媒体查询把 hover 动画关进笼子。

## Sonner 原则（构建被人喜爱的组件）

这些原则来自构建 Sonner 的实践（每周 npm 下载量 1300 万+），适用于任何组件：

1. **开发者体验是关键。** 无 hooks、无 context、无复杂配置。插入一次 `<Toaster />`，在任何地方调用 `toast()`。采用门槛越低，用的人越多。

2. **好的默认值比选项更重要。** 开箱即美。大多数用户永远不会自定义。默认的缓动、时长和视觉设计必须出色。

3. **命名创造身份认同。** "Sonner"（法语"鸣响"）比 "react-toast" 更优雅。适当时，用可发现性换取可记忆性。

4. **隐形地处理边缘情况。** 标签页隐藏时暂停 toast 计时器。用伪元素填补堆叠 toast 之间的空隙以维持 hover 状态。拖拽期间捕获指针事件。用户永远不会注意到这些——而这恰恰是对的。

5. **动态 UI 用 transitions，不用 keyframes。** Toast 会被快速连续添加。Keyframes 被打断时从零重启；transitions 平滑地重定向。

6. **构建出色的文档站。** 让人们在使用之前就能触摸、把玩、理解产品。带现成代码片段的交互式示例降低了采用门槛。

### 一致性（Cohesion）很重要

Sonner 的动画之所以令人满意，部分在于整体体验是一致的。缓动和时长符合这个库的气质。它比典型 UI 动画稍慢，用 `ease` 而非 `ease-out`，以显得更优雅。动画风格与 toast 设计、页面设计、名字相匹配——一切都是和谐的。

选择动画参数时，考虑组件的性格。playful 的组件可以更弹一些；专业的仪表盘应该干脆利落、快。让运动匹配情绪。

### opacity + height 的组合

当条目进出列表时（比如 Family 的抽屉），opacity 变化必须与高度动画配合良好。这往往要靠反复试错。没有公式——调到你感觉对了为止。

### 第二天再审视你的作品

用新鲜的眼光审视动画。第二天你会注意到开发时忽略的不完美之处。慢速播放或逐帧播放动画，能发现全速时看不见的时机问题。

### 非对称的进出时机

需要刻意为之时按压应该慢（长按删除：2s linear），但松开永远要利落（200ms ease-out）。这个模式适用范围很广：用户在做决定的地方慢，系统在响应的地方快。

```css
/* 松开：快 */
.overlay {
  transition: clip-path 200ms ease-out;
}

/* 按压：慢且刻意 */
.button:active .overlay {
  transition: clip-path 2s linear;
}
```

## 交错动画（Stagger Animations）

当多个元素一起进入时，让它们交错出现。每个元素在前一个之后以微小延迟入场。这创造了级联效果，比所有东西同时出现更自然。

```css
.item {
  opacity: 0;
  transform: translateY(8px);
  animation: fadeIn 300ms ease-out forwards;
}

.item:nth-child(1) {
  animation-delay: 0ms;
}
.item:nth-child(2) {
  animation-delay: 50ms;
}
.item:nth-child(3) {
  animation-delay: 100ms;
}
.item:nth-child(4) {
  animation-delay: 150ms;
}

@keyframes fadeIn {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
```

交错延迟保持短（条目之间 30-80ms）。长延迟会让界面感觉慢。交错是装饰性的——动画播放期间永远不要阻塞交互。

## 调试动画

### 慢动作测试

以降低的速度播放动画，发现全速时看不见的问题。临时把时长增加到正常的 2-5 倍，或用浏览器 DevTools 的动画检查器减慢播放。

慢动作下要检查的东西：

- 颜色过渡是否平滑，还是看到两个明显不同的状态在重叠？
- 缓动感觉对吗，还是开始/结束得很突兀？
- transform-origin 正确吗，还是元素从错误的点缩放？
- 多个动画属性（opacity、transform、color）是否同步？

### 逐帧检查

在 Chrome DevTools（Animations 面板）中逐帧步进动画。这能揭示全速时看不到的协调属性之间的时机问题。

### 在真实设备上测试

触摸交互（抽屉、滑动手势）要在实体设备上测试。用 USB 连接手机，通过 IP 地址访问本地 dev server，使用 Safari 的远程开发者工具。Xcode 模拟器是备选，但手势测试真实硬件更好。

## 评审检查清单

评审 UI 代码时，检查：

| 问题                                      | 修复                                                              |
| ------------------------------------------ | ---------------------------------------------------------------- |
| `transition: all`                          | 明确指定属性：`transition: transform 200ms ease-out` |
| `scale(0)` 入场动画                 | 从 `scale(0.95)` 开始并配合 `opacity: 0`                       |
| UI 元素用 `ease-in`                    | 换成 `ease-out` 或自定义曲线                             |
| 弹层用 `transform-origin: center`      | 设置为触发点位置，或用 Base UI 的 `var(--transform-origin)`（modal 豁免——保持居中） |
| 键盘动作上有动画               | 完全移除动画                                        |
| UI 元素时长 > 300ms             | 缩减到 150-250ms                                              |
| hover 动画没有媒体查询        | 加上 `@media (hover: hover) and (pointer: fine)`                  |
| 快速触发的元素用 keyframes     | 用 CSS transitions 保证可中断性                         |
| 高负载下用 Framer Motion `x`/`y` props     | 用 `transform: "translateX()"` 获得硬件加速        |
| 进入/退出过渡速度相同           | 让退出比进入快（如进入 2s，退出 200ms）         |
| 所有元素同时出现                | 加交错延迟（条目间 30-80ms）                        |
