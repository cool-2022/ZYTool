# Animation Vocabulary（动画术语表）

> 来源：翻译自 `666_AI/skills/skills/animation-vocabulary/SKILL.md`。用途：反向查询词典——把对 Web 动画/动效的模糊描述转换成精确术语（术语保留英文，附中文解释）。

---

```
name: animation-vocabulary
description: 反向查询术语表，把对 Web 动画或动效的模糊描述转换成精确术语（"popover 打开时弹一下的那个效果" → Pop in；"iOS 橡皮筋滚动" → Rubber-banding）。当用户问"那个叫什么……"，或描述了一个动效但不知道名字、想找正确的词去提示 AI 或设计师时使用。用于给效果命名，而不是设计或构建效果。
```

# 动画术语表

把对运动或效果的模糊描述转换成精确术语，让用户知道该要什么。

## 快速开始

用户松散地描述一个效果。你按以下格式返回匹配的术语：

```
**Stagger** — Animate several items one after another with a small delay between each, creating a cascade.
```

如果有多个术语可能匹配，把最佳匹配放最前，然后列 1–2 个备选，附一行说明它们的区别。

## 使用说明

1. **读意图，而不是读关键词。** 用户描述的是他们*看到*或*感觉到*的东西（"弹弹的"、"滑出去"、"自己画出来"），而不是技术名词。把这种感觉映射到下面的术语表。
2. **逐字引用术语表。** 术语表的描述是权威——原样使用，不要转述。
3. **区分相近术语。** 当两个术语竞争时（*Clip-path* vs *Mask*、*Pop in* vs *Bounce*、*Shared element transition* vs *Layout animation*），对比它们，让用户能选择。
4. **当没有完全匹配时，** 给出最接近的术语并直说这是近似，或者用术语表的词汇描述该效果（例如"那是一个 *scale-in* 入场的 *stagger*"）。
5. **不超出本术语表。** 如果一个术语真的不在这里，直说，而不是编造——不过你可以用这些词解释概念。
6. **保持简短。** 命名问题要的是名字，不是论文。先给术语；只在被问时才展开。

## 示例

**示例 1——基于感觉**
用户："popover 从你点的按钮里长出来、而不是从它自己中间长出来，这个叫什么？"
输出：
```
**Origin-aware animation** — An element animates out of its trigger, like a popover growing from the button that opened it instead of from its own center which is the default in CSS.
```

**示例 2——消歧**
用户："一张图变成另一张图的那个效果。"
输出：
```
**Morph** — One shape smoothly turns into another shape, e.g. Dynamic Island.

Close alternates:
- **Crossfade** — if they simply fade over each other in the same spot.
- **Shared element transition** — if an element travels and transforms from one position into another.
```

**示例 3——物理手感**
用户："iOS 那个滚动，拉过头会抵抗并弹回去的。"
输出：
```
**Rubber-banding** — Resistance and snap-back when you drag past a boundary (the iOS overscroll feel).
```

## 术语表

_这是项目 `/vocabulary` 页面的精选快照；任一方变更时保持两边同步。_

### Entrances & Exits（入场与出场）——元素如何出现和消失
- **Fade in / Fade out** — 元素通过改变透明度出现或消失。
- **Slide in** — 元素从屏幕外（左、右、上或下）滑入。
- **Scale in** — 元素出现时从较小尺寸长到完整尺寸，常与淡入搭配。
- **Pop in** — 元素出现时带轻微过冲，像弹跳着到位。
- **Reveal** — 内容被逐渐揭开，常通过动画 clip-path 或 mask。
- **Enter / Exit** — 元素被添加到屏幕或从屏幕移除时播放的动画。

### Sequencing & Timing（编排与时序）——协调多个元素或多个时刻
- **Keyframes** — 动画中定义的点（0%、50%、100%），浏览器补齐它们之间的帧。
- **Interpolation / Tween** — 在起始值和结束值之间生成所有中间帧，使运动连续。
- **Stagger** — 让多个项目一个接一个地动画，每个之间有小延迟，形成级联。
- **Orchestration** — 刻意安排多个动画的时序，让它们感觉像一次协调的运动。
- **Delay** — 动画开始前的时间。
- **Duration** — 动画持续的时间。
- **Fill mode** — 元素在动画开始前或结束后是否保留第一帧或最后一帧的样式（例如 forwards）。
- **Stepped animation** — 被分成离散步骤的动画，像倒计时器。

### Movement & Transforms（移动与变换）——改变元素的位置、大小或角度
- **Translate** — 沿 X 或 Y 轴移动元素。
- **Scale** — 让元素变大或变小。
- **Rotate** — 让元素绕一个点旋转。
- **Skew** — 沿 X 或 Y 轴倾斜元素，把它剪切成非矩形。
- **3D tilt / Flip** — 在 3D 空间旋转（rotateX / rotateY）以增加深度。
- **Perspective** — 3D 效果的强度——值越小深度越夸张，像观察者更近。
- **Transform origin** — 缩放或旋转生长/旋转所围绕的锚点。
- **Origin-aware animation** — 元素从它的触发者动画出来，比如 popover 从打开它的按钮生长，而不是从自己的中心（CSS 默认是中心）。

### Transitions Between States（状态间过渡）——把一个状态、视图或元素连接到另一个
- **Crossfade** — 一个元素淡出，同时另一个在同一位置淡入。
- **Continuity transition** — 通过在视觉上连接前后状态来保持用户方向感的变化。例如让同一个矩形变大变小。
- **Morph** — 一个形状平滑地变成另一个形状，例如 Dynamic Island。
- **Shared element transition** — 一个元素从一个位置移动并变换到另一个位置，比如缩略图展开成卡片。
- **Layout animation** — 当元素的尺寸或位置变化时，它动画到新位置而不是跳变。
- **Accordion / Collapse** — 一个区块平滑地展开和收起高度，以显示或隐藏内容。
- **Direction-aware transition** — 前进时内容向一个方向滑，后退时向反方向滑，让导航有方向感。

### Scroll（滚动）——与滚动或视图间导航绑定的运动
- **Scroll reveal** — 元素进入视口时淡入或滑入到位。
- **Scroll-driven animation** — 进度直接与滚动位置绑定的动画。
- **Parallax** — 滚动时背景和前景以不同速度移动，产生景深。
- **Page transition** — 从一个页面或路由导航到另一个时播放的动画。
- **View transition** — 浏览器在两个状态或页面之间变形过渡，连接共享元素。

### Feedback & Interaction（反馈与交互）——响应用户的动作
- **Hover effect** — 光标移到元素上时的视觉变化。
- **Press / Tap feedback** — 元素被点击时轻微缩小，让它感觉有物理实体。
- **Hold to confirm** — 用户按住按钮时逐渐填满的进度效果。
- **Drag** — 抓住元素移动它，释放时常带惯性。
- **Drag to reorder** — 拖动列表项重排，其他项移位让出空间。
- **Swipe to dismiss** — 把元素拖出屏幕来关闭它，比如抽屉或 toast。
- **Rubber-banding** — 拖过边界时的阻力和回弹（iOS 过度滚动的手感）。
- **Shake / Wiggle** — 快速的左右抖动，表示错误或输入被拒绝。
- **Ripple** — 从点按位置扩散的圆形，确认这次按压。

### Easing（缓动）——动画过程中速度如何变化
- **Easing** — 动画加速或减速的速率。
- **Ease-out** — 开始快，结束慢。大多数 UI 和任何响应用户的东西的默认值。
- **Ease-in** — 开始慢，结束快。通常应避免；会感觉迟钝。
- **Ease-in-out** — 慢、快、慢。适合已在屏幕上的元素从 A 移动到 B。
- **Linear** — 恒定速度。UI 中避免；留给 spinner 或跑马灯。
- **Cubic-bezier** — 你自定义的缓动曲线，用于精确控制。
- **Asymmetric easing** — 加速和减速速率不同的曲线。比对称曲线更有生气。

### Spring Animations（弹簧动画）——基于物理的运动，作为固定时长缓动的替代
- **Spring** — 由物理（张力、质量、阻尼）而非设定时长驱动的运动。
- **Stiffness / Tension** — 弹簧向目标拉的强度。越高越利落。
- **Damping** — 弹簧落定的快慢。阻尼越低回弹和振荡越多。
- **Mass** — 被动画的元素感觉多重。质量越大越慢、越钝。
- **Bounce** — 过冲后落定的弹簧，增加俏皮感。
- **Perceptual duration** — 弹簧*感觉*完成的时长，尽管它底下仍在微落定。
- **Momentum** — 携带速度的运动，尤其在拖拽或中断之后。
- **Velocity** — 元素运动的速度和方向。弹簧被中断时会把它带入下一个动画，所以被甩动的元素保持它的速度。
- **Interruptible animation** — 可以在飞行途中被平滑转向、而不是先播完的动画。

### Looping & Ambient Motion（循环与环境运动）——自己运行的动画
- **Marquee** — 连续循环滚动的文本或内容。
- **Loop** — 重复的动画，固定次数或无限。
- **Alternate (yoyo)** — 每次迭代正向播放再反向的循环，而不是跳回开头。
- **Orbit** — 一个元素沿连续路径绕另一个元素转圈。
- **Pulse** — 温和的重复缩放或透明度变化，用来吸引注意。
- **Float** — 温和、连续的上下漂浮，让静态元素感觉活着、失重。
- **Idle animation** — 元素只是待在那里、等待交互时播放的细微运动。

### Polish & Effects（润色与效果）——区分好与卓越的小细节
- **Blur** — 模糊滤镜，用于柔化元素或掩盖微小瑕疵。
- **Clip-path** — 把元素裁剪成某个形状，用于揭示、遮罩和前后对比滑块。
- **Mask** — 用形状或渐变隐藏或显示元素的一部分——像 clip-path，但有柔和、可淡变的边缘。
- **Before / after slider** — 可拖动的分隔条，在两张叠放的图片之间擦动以对比。
- **Line drawing** — 自己画出来的 SVG 路径，像一支隐形的笔在描。
- **Text morph** — 文本变化时逐字符动画，把注意力引向新值。
- **Skeleton / Shimmer** — 内容加载时显示的带流动光泽的占位符。
- **Number ticker** — 数字滚动或计数到某个值。
- **Tabular numbers** — 等宽数字，数字变化时不移位。ticker、计时器、计数器的必需品。
- **Typewriter** — 文本逐字符出现，像正在被打字。

### Performance（性能）——让运动平滑而非卡顿的东西
- **Frame rate (FPS)** — 每秒绘制的帧数。60fps 是平滑运动的基线；新显示器 120fps。
- **Jank** — 浏览器跟不上动画而丢帧时的可见卡顿。
- **Dropped frame** — 浏览器错过绘制截止时间的一帧，造成运动中的小顿挫。
- **Compositing** — 让 GPU 在自己的图层上移动或淡变元素，而不重做布局或绘制。
- **will-change** — 一个 CSS 提示，告诉浏览器元素即将动画，让它提前把元素提升到自己的图层。
- **Layout thrashing** — 动画 width、height、top、left 这类属性，迫使浏览器每帧重算布局，导致 jank。

### Principles to Know（应知原则）——指导何时以及如何动画的概念
- **Purposeful animation** — 运动应服务于功能——定向、给反馈、展示关系——而不只是装饰。
- **Anticipation** — 移动前向反方向的小蓄力，暗示即将发生什么。
- **Follow-through** — 主体运动停止后，元素的某些部分继续运动并略微落定，增加重量感。
- **Squash & stretch** — 元素运动时变形，以传达重量、速度和柔韧性。
- **Perceived performance** — 正确的动画让界面感觉更快，即使它并没有。
- **Frequency of use** — 用户越常看到一个动画，它就应该越短、越微妙。
- **Spatial consistency** — 让元素跨状态保持身份和位置的动画方式，让用户永远不会跟丢东西去了哪。
- **Hardware acceleration** — 动画 transform 和 opacity 让 GPU 保持运动平滑。
- **Reduced motion** — 尊重用户的 prefers-reduced-motion 设置，调低或移除运动。
