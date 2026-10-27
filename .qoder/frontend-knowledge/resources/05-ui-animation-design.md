# 05 - UI 动画与设计规范（emilkowalski/skills 知识库整理）

> 来源：emilkowalski/skills（Emil Kowalski 的 10 个 Agent Skill，源自其在 Vercel/Linear 的设计工程经验与 animations.dev 课程，以及 Apple WWDC 流体界面设计演讲）。
> 用途：为 ZYTool 前端（Vue 3 + Ant Design Vue）的动效编写、评审、组件选型提供标准。库推荐以 React 生态为主，Vue 项目选型时作参考映射。
> **完整内容已全量翻译收录于 `resources/ui-animation-skills/`（00-索引.md ~ 10-ask-sonner.md），自包含、可直接移植到其他项目**；本文件为摘要速查。

## 一、10 个 Skill 速查（完整版见 `ui-animation-skills/` 目录）

| Skill | 角色 | 一句话 | 完整版文件 |
|---|---|---|---|
| emil-design-eng | 哲学母体 | 全部规则的源头：动画决策框架、组件原则、性能、无障碍 | `ui-animation-skills/01-emil-design-eng.md` |
| animate | 构建者 | 从零写动画：7 步序列 + RECIPES 配方库，闸门不过就拒写 | `ui-animation-skills/02-animate.md` |
| review-animations | 评审者 | 按十大标准审动画 diff，输出 `\| Before \| After \| Why \|` 表 + Block/Approve 裁决 | `ui-animation-skills/03-review-animations.md` |
| improve-animations | 审计+规划器 | 普查全库动画，产出优先级排序的自包含计划（写 `plans/`，不改源码） | `ui-animation-skills/07-improve-animations.md` |
| find-animation-opportunities | 机会发现器 | 找"该动未动"之处，≤5–7 条建议 + 必写拒绝清单，不改代码 | `ui-animation-skills/06-find-animation-opportunities.md` |
| animation-vocabulary | 术语词典 | 模糊描述 → 精确术语（Pop in / Rubber-banding / Stagger...），只命名不设计 | `ui-animation-skills/05-animation-vocabulary.md` |
| apple-design | 底层原理 | Apple 流体界面/弹簧/材质/排版/设计原则的 Web 映射 | `ui-animation-skills/04-apple-design.md` |
| pick-ui-library | 选型器 | 按任务推荐唯一精选库（只显式调用） | `ui-animation-skills/08-pick-ui-library.md` |
| prototype | 原型探索 | 同一 UI 做 3–5 个真正不同方向的变体 + picker 切换器供选择 | `ui-animation-skills/09-prototype.md` |
| ask-sonner | Sonner 指南 | React toast 库 Sonner 的接入/样式/排障（Vue 项目可用 vue-sonner 类比） | `ui-animation-skills/10-ask-sonner.md` |

## 二、公共动画取值标准（所有 skill 共享的硬数值，禁止近似）

### 频率门槛（动不动？）

| 频率 | 决策 |
|---|---|
| 100+ 次/天（快捷键、命令面板） | 永不动画；键盘触发一票否决（Raycast 无开关动画即最优） |
| 数十次/天（hover、列表导航） | 移除或大幅减弱 |
| 偶尔（modal、drawer、toast） | 标准动画 |
| 罕见/首次（onboarding、庆祝） | 可加 delight（愉悦预算只花在这） |

### Easing 决策树

- 进入/退出 → `ease-out`；屏上移动/变形 → `ease-in-out`；hover/颜色 → `ease`；恒定运动（marquee、进度条）→ `linear`；默认 → `ease-out`
- **UI 动画禁用 `ease-in`**（起步慢，感觉拖沓）
- 内置 CSS easing 太弱，用强自定义曲线 token：

```css
--ease-out: cubic-bezier(0.23, 1, 0.32, 1);        /* UI 强 ease-out */
--ease-in-out: cubic-bezier(0.77, 0, 0.175, 1);    /* 屏上移动 */
--ease-drawer: cubic-bezier(0.32, 0.72, 0, 1);     /* iOS 抽屉曲线 */
```

### 时长表（UI 动画一律 < 300ms）

| 元素 | 时长 |
|---|---|
| 按钮按压反馈 | 100–160ms |
| Tooltip、小 popover | 125–200ms |
| Dropdown、select | 150–250ms |
| Modal、drawer | 200–500ms |
| 营销/解释型 | 可更长 |

### 弹簧（Spring）

- 适用：拖拽惯性、"活"的元素、可中断手势、装饰性鼠标跟随
- Apple 风格推荐：`{ type: "spring", duration: 0.5, bounce: 0.2 }`；传统物理：`{ mass: 1, stiffness: 100, damping: 10 }`
- bounce 保持 0.1–0.3，多数 UI 用临界阻尼（无回弹）；回弹只留给带动量的手势
- Apple 发货参数：移动/重定位 damping 1.0 / response 0.4；旋转 0.8/0.4；抽屉 Sheet 0.8/0.3
- 关键优势：**可中断性**——弹簧被中断保持速度，keyframes 从零重启

### 性能铁律

- **只动 `transform` 和 `opacity`**（GPU）；禁动 width/height/margin/padding/top/left
- `transition: all` 永远是 finding——指明确切属性
- Framer Motion 简写 `x`/`y`/`scale` 不走硬件加速，写全 `transform: "translateX(100px)"`
- 不要用父级 CSS 变量驱动子元素 transform（子元素全部重算样式）
- 页面繁忙时 CSS 动画优于 JS（离主线程）；预设轨迹用 CSS，动态可中断用 JS/WAAPI
- `filter: blur()` 保持 < 20px（Safari 上重模糊很贵）

### 无障碍（随动画一起交付，不是后补）

```css
@media (prefers-reduced-motion: reduce) {
  /* 保留 opacity/颜色过渡，去掉位移；是降级不是归零 */
}
@media (hover: hover) and (pointer: fine) {
  /* hover 动效必须门控，触屏 tap 会误触发 hover */
}
```

## 三、常用配方（animate/RECIPES.md）

- **按钮按压**：`:active { transform: scale(0.97) }` + `transition: transform 160ms var(--ease-out)`；scale 区间 0.95–0.98
- **入场**：禁 `scale(0)`，用 `scale(0.95)` + `opacity: 0`；免 JS 用 `@starting-style`（旧浏览器降级：useEffect + `data-mounted`）
- **Popover/Dropdown**：`transform-origin: var(--transform-origin)` 从触发器缩出；**modal 豁免**保持 center；200ms ease-out，起止 `scale(0.95)`
- **Tooltip**：125ms、初始 `scale(0.97)`；首个延迟防误触，相邻 tooltip `[data-instant] { transition-duration: 0ms }`
- **Modal**：`transform-origin: center`，250ms，`scale(0.96)` 起，backdrop 同步 250ms opacity
- **Drawer**：`transition: transform 500ms var(--ease-drawer)`，关闭态 `translateY(100%)`（百分比相对自身尺寸，优于硬编码 px）
- **Toast**：`opacity + transform 400ms ease` + `@starting-style { translateY(100%) }`；进出路径对称（底部滑入底部滑出）
- **Accordion**：`overflow: hidden` + height/opacity 200ms ease-out；务必短（每帧走 layout）
- **Stagger 群体入场**：30–80ms/项；`translateY(8px)` + fadeIn 300ms ease-out，delay 50/100/150ms；纯装饰不阻塞交互
- **Hold to confirm**：`clip-path: inset(0 100% 0 0)` → 按住 `inset(0 0 0 0)` 2s linear 填满，松开 200ms ease-out 弹回（非对称时序：蓄意慢、响应快）
- **Tab 指示器**：复制 tab 列表裁剪激活副本，`clip-path: inset(...)` 250ms ease-in-out
- **Scroll reveal**（仅营销面）：`inset(0 0 100% 0)` → `inset(0 0 0 0)` 600ms；`useInView({ once: true, margin: "-100px" })`
- **Crossfade 遮缝**：双态叠影加 `filter: blur(2px); opacity: 0.7`，200ms ease
- **WAAPI 无库动画**：`element.animate([...], { duration: 1000, fill: 'forwards', easing: 'cubic-bezier(0.77, 0, 0.175, 1)' })`

### 手势/拖拽

- 动量 dismiss：`velocity = Math.abs(swipeAmount) / timeTaken`，**velocity > 0.11** 即 dismiss（不强制距离阈值）
- 1:1 跟随手指、尊重抓取偏移；Pointer Events + `setPointerCapture`；多点触控保护 `if (isDragging) return`
- 橡皮筋软边界：`(overshoot * dim * 0.55) / (dim + 0.55 * |overshoot|)`，不硬停
- Apple 动量投影落点：`project(v) = (v/1000) * d / (1 - d)`，`d ≈ 0.998`（Vaul/Embla 的做法；不是物理课本的 v²/2a）
- 速度交接：弹簧初速度 = 释放速度；需归一化时 `v / (target - current)`
- 2D 运动分解为独立 X、Y 两个弹簧（单弹簧两轴失步）

## 四、评审标准（review-animations 十条 + 触发器）

十条不可协商：① 目的正当（反馈/空间一致性/状态指示/解释/防突兀，"看着酷"不算）② 频率匹配 ③ ease-out 系曲线 ④ UI < 300ms ⑤ 原点与物理正确（popover 从触发器、禁 scale(0)）⑥ 可中断（高频用 transition/弹簧不用 keyframes）⑦ 只动 GPU 属性 ⑧ 无障碍两媒体查询 ⑨ 非对称进出时序 ⑩ 协调性（动效人格匹配组件）。

见即重判的触发器：`transition: all`、`scale(0)`、UI 上 `ease-in`、键盘/高频动作动画、>300ms、浮层 center origin、toast 用 keyframes、动 layout 属性、FM `x/y` 简写、父变量驱动子 transform、缺 reduced-motion、未门控 hover、对称按压时序、群体同现无 stagger。

补救偏好层级：删动画 → 减弱 → 修 easing → 修原点 → 可中断化 → 上 GPU → 非对称时序 → 打磨（blur/stagger/@starting-style）→ 无障碍补齐。**最强的修复往往是删掉动画。**

## 五、Apple 设计原则要点（apple-design）

- **响应性**：pointer-down 即反馈，不等 click/释放；按压 `:active scale(0.97)` 100ms ease-out；清除输入路径上的 debounce/定时器延迟
- **可中断性**（最重要单一原则）：过渡期间不锁输入；永远从 presentation（当前屏幕）值开始动画；反转时混合速度不硬切
- **空间一致性**：进出同路径（右进右出）；弹层从触发元素生长；可逆过渡镜像缓动
- **材质与景深**：半透明浮层 `backdrop-filter: blur(20px) saturate(180%)` + 半透明背景；绝不浅半透明叠浅半透明；模糊面上文字提高对比/字重；粘性头下用模糊淡出替代 1px 硬分隔线；玻璃面进出场时模糊半径和缩放一起动
- **多模态反馈三规则**：因果性、和谐性（视/声/触同帧）、效用性（只在有意义时刻）
- **排版**：字距随字号变化（大标题 `-0.02em` 负字距，正文近 0）；行距与字号反向（标题 1.05、正文 1.5）；层级靠字重+字号+行距成套；间距用 rem/em 尊重用户字号；默认系统字体
- **设计八原则**：目的、掌控（容错易撤销）、责任、熟悉、灵活、简单≠极简、匠心、愉悦（是结果不是装饰）
- 确认对话框只给真正破坏性不可逆操作（滥用训练用户盲点通过）；行内校验优于提交时报错

## 六、库选型清单（pick-ui-library，React 生态为主）

| 任务 | 推荐库 | Vue 生态参考映射 |
|---|---|---|
| 无样式可访问组件原语 | base-ui | radix-vue / Reka UI |
| 命令菜单（⌘K） | cmdk | —（可用 cmdk 思路自研） |
| Toast 通知 | Sonner | vue-sonner |
| 通用动画/弹簧/手势 | motion（Framer Motion） | @vueuse/motion / motion-v |
| 数字滚动 | NumberFlow | — |
| 拖拽 | dnd kit | vue-draggable-plus |
| 虚拟化长列表 | Virtuoso | vue-virtual-scroller |
| 实时流式图表 | Liveline | — |
| 通用图表 | recharts | ECharts（本项目已定） |
| 状态管理 | zustand | Pinia（本项目已定） |
| 条件 className | clsx | clsx（通用） |
| 语法高亮 | shiki | shiki（通用） |
| 主题切换无闪烁 | next-themes | @vueuse/core useDark |

选型规则：识别任务而非用户点名的库；先看 package.json 已装什么（已装竞品不擅换）；只推荐一个；简单 hover/淡入淡出不需要动画库，纯 CSS transition 即可。

## 七、Sonner 要点速查（ask-sonner，用 vue-sonner 时类比）

- `<Toaster />` 只挂一次在根部，绝不按页面/条件渲染（两个 Toaster = toast 重复）；`toast()` 在客户端任意处调用
- 更新 toast：相同 `id` 再调；loading→success：`const id = toast.loading('…'); toast.success('…', { id })`
- 持久 `{ duration: Infinity }`；`toast.dismiss(id)`；`toast.promise(promise, { loading, success, error })`
- 样式阶梯：默认 + `richColors` → `toastOptions.style` → `classNames`（需 `!important`）→ headless `toast.custom()`
- 主题默认 `'light'` 不跟系统 → `theme="system"`；被 modal 遮挡 → Toaster 移到 portal 容器外
- 默认：`duration: 4000`、`position: 'bottom-right'`、`visibleToasts: 3`、`offset: 32px`、`mobileOffset: 16px`

## 八、原型探索流程（prototype）

1. 范围：一次只做一件 UI；2. 侦察：技术栈/token/产品性格；3. 选方向：默认 3 个变体（上限 5），沿可命名的轴发散（布局/密度/性格/动效/交互模型），禁换色变体；4. picker 脚手架：固定规格的底部玻璃拟态切换器（数字键/←→切换、瞬时无动画、一次全尺寸渲染一个变体）；5. 表格呈现取舍后停下等用户选；6. 胜者按项目约定集成并删除原型文件。
硬性规则：探索期不碰生产代码；变体必须完全可用（禁 lorem ipsum/死按钮）；切换必须瞬时。

## 九、反模式速查清单（Never Ship）

| 禁止 | 改为 |
|---|---|
| `transition: all` | 指明确切属性 |
| `scale(0)` 入场 | `scale(0.95)` + `opacity: 0` |
| UI 上 `ease-in` | `ease-out` 或强自定义曲线 |
| 快捷键/高频动作动画 | 无动画 |
| UI 时长 > 300ms | 150–250ms |
| popover 用 center origin | `var(--transform-origin)`（modal 豁免） |
| toast/高频元素用 keyframes | CSS transition |
| 动 width/height/margin 等 | `transform`/`opacity` |
| 未门控 `:hover` 动效 | `@media (hover: hover) and (pointer: fine)` |
| 缺 reduced-motion | 更柔的变体（非归零） |
| 元素同时入场 | 30–80ms stagger |
| 拖拽只按距离阈值 dismiss | 速度阈值 > 0.11 |
| 边界硬停 | 橡皮筋渐进阻力 |
