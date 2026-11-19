> 来源：翻译自 666_AI/skills/skills/pick-ui-library/SKILL.md。用途：为一项前端任务从精选清单中挑选最合适的库（数字、OTP 输入、图表、命令菜单、虚拟列表、拖拽、Toast、状态、样式等）。

---

name: pick-ui-library
description: Pick the right library for a given frontend task from a curated, opinionated list — numbers, OTP inputs, charts, command menus, virtualization, drag and drop, toasts, state, styling, and more. Only runs when explicitly invoked; it does not trigger on its own.
disable-model-invocation: true
---

# 挑选正确的库

这是一个查阅型技能。当带着一项任务调用它时（"我需要 toast"、"拖拽该用什么？"），将任务与下方精选清单匹配并推荐对应库。这些是经过深思熟虑、由品味驱动的选择——除非用户明确要求，或任务确实不在清单覆盖范围内，否则不要替换成清单之外的替代方案。

## 如何使用本技能

1. **识别任务**，而不是用户说出的库名。"我需要显示一个下拉菜单"是一个 UI 原语任务（base-ui），即使他们问的是别的东西。
2. **检查已安装的依赖。** 先看 `package.json`。如果项目已经在使用清单中的某个库，就用它。如果用的是竞品（例如用 react-window 而不是 Virtuoso），指出推荐方案，但在未被要求的情况下不要改动依赖。
3. **只推荐一个库**，用一句话说明它的用途，并在请求包含安装/接线时完成这些工作。当清单有明确答案时，不要摆出一份选项菜单。
4. 如果任务不在清单覆盖范围内，明确说明，并基于你自己的知识推荐——但要清楚表明你已经超出了精选清单的范围。

## 清单

### UI 组件与原语

| 任务 | 库 |
| --- | --- |
| 无样式、无障碍的 UI 组件（对话框、弹出层、菜单、选择器……） | [base-ui](https://base-ui.com) |
| 命令菜单（⌘K 面板） | [cmdk](https://cmdk.paco.me) |
| Toast / 通知 | [Sonner](https://sonner.emilkowal.ski) |
| 一次性密码 / 验证码输入框 | [input-otp](https://input-otp.rodz.dev) |
| 可定制的 GUI / 控制面板 | [Leva](https://github.com/pmndrs/leva) —— 替代方案 [dialkit](https://joshpuckett.me/dialkit) |

### 动效与视觉

| 任务 | 库 |
| --- | --- |
| 通用动画（弹簧、布局动画、进入/退出） | [motion](https://motion.dev) (Framer Motion) |
| 数字动画（计数器、价格、统计数据） | [NumberFlow](https://number-flow.barvian.me) |
| 动画文本组件 | [torph](https://torph.lochie.me/) |
| 3D 地球仪 | [Cobe](https://cobe.vercel.app) |
| 动态 OG 图片（HTML/CSS → SVG/PNG） | [Satori](https://github.com/vercel/satori) |
| 语法高亮 | [shiki](https://shiki.style) |

当你需要弹簧、布局动画、退出动画或手势驱动的数值时，使用 motion。简单的 hover 或淡入淡出不需要它——纯 CSS transition 才是那里的正确工具。

### 图表

| 任务 | 库 |
| --- | --- |
| 实时 / 流式图表 | [Liveline](https://github.com/benjitaylor/liveline) |
| 通用图表（静态或交互式仪表盘） | [recharts](https://recharts.org) |

划分标准：如果数据点实时到达且图表随时间滚动，用 Liveline。其他所有情况都用 recharts。

### 交互与性能

| 任务 | 库 |
| --- | --- |
| 拖拽 | [dnd kit](https://dndkit.com) |
| 虚拟化（长列表、大表格） | [Virtuoso](https://virtuoso.dev) |

### 状态与样式

| 任务 | 库 |
| --- | --- |
| 状态管理 | [zustand](https://zustand.docs.pmnd.rs) |
| 条件拼接 `className` 字符串 | [clsx](https://github.com/lukeed/clsx) |
| Tailwind 的类型安全、变体驱动样式 | [cva](https://cva.style) |
| 主题切换 / 暗色模式（加载时不闪烁） | [next-themes](https://github.com/pacocoursey/next-themes) |

样式划分标准：临时性的条件类名用 clsx；当组件具有真正的变体（尺寸、意图、状态）且值得一个带类型的 API 时用 cva。二者可以组合——cva 内部使用 clsx 风格的输入。

## 需要警惕的常见错配

- **手写或用模态框库实现 toast** → Sonner 正是为此而生。
- **基于 `<div>` 的下拉/对话框并手动处理焦点** → base-ui，它处理无障碍、焦点捕获和关闭。
- **通过重新渲染文本来做数字动画** → NumberFlow 能正确处理数位过渡。
- **直接渲染 1,000+ 行的列表** → 在用分页 hack 之前先用 Virtuoso。
- **每个组件一个 `useState`、靠层层 props 传递共享状态** → zustand。
- **模板字符串 className 里嵌套三层三元运算符** → clsx（如果呈变体形态则用 cva）。
