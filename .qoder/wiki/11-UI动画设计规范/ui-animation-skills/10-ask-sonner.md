> 来源：翻译自 666_AI/skills/skills/ask-sonner/SKILL.md 与 API.md。用途：React Toast 库 Sonner 的完整使用与排障指南——安装接线、选择正确的 toast() 调用、promise/loading、更新/关闭/持久化、样式、主题、图标、定位与多 Toaster。

---

name: ask-sonner
description: Guide to Sonner, the React toast library — install and wire up the Toaster, pick the right toast() call, promise and loading toasts, updating, dismissing and persisting toasts, styling, theming and icons, positioning and multiple toasters. Use when working with Sonner or troubleshooting it — toasts that don't appear, appear twice, lose their styles, ignore Tailwind classes, sit behind a modal, or don't follow dark mode.
---

# 使用 Sonner

这是 [Sonner](https://sonner.emilkowal.ski)（Toast 库）的指南型技能。当任务涉及 Sonner——接线、渲染 toast、样式或修复——先从本文件回答。`<Toaster />` 和 `toast()` 的完整 prop 表在本文"API 参考"章节中；当你需要精确的 prop 名、类型或默认值时查阅它。

## 安装接线

两个部分，且只有两个：

1. **一个 `<Toaster />`，只挂载一次**，尽可能靠近根部（在 Next.js 中：`layout.tsx`——它在 server component 内部也能工作）。绝不要按页面渲染它或条件渲染它；第二个被挂载的 Toaster 会重复每个 toast。
2. **从客户端代码调用 `toast()`**——事件处理器、effect、回调。它是一个普通函数，不需要 hook 或 provider，但在服务端它什么都不做：在 server action 中，返回结果并在接收结果的客户端代码中调用 `toast()`。

```jsx
import { Toaster } from 'sonner'; // once, in layout
import { toast } from 'sonner';   // anywhere client-side
```

## 选择正确的调用

| 你想要 | 调用 |
| --- | --- |
| 纯消息 | `toast('Title')` —— 加 `{ description }` 获得第二行 |
| 成功 / 错误 / 信息 / 警告图标 | `toast.success('…')`、`toast.error('…')` 等 |
| 自己管理状态时的转圈 | `toast.loading('…')`，然后按 id 更新它 |
| 绑定 promise 的 loading → success/error | `toast.promise(promise, { loading, success, error })` —— success/error 接受接收解析值/错误的函数 |
| 做事的按钮 | `{ action: { label, onClick } }` —— 点击会关闭 toast，除非 `onClick` 调用 `event.preventDefault()`；`cancel` 是次要按钮变体 |
| 自定义 JSX，保留默认 toast 外壳 | `toast(<jsx />)` |
| 自定义 JSX，完全无样式 | `toast.custom((t) => <jsx />)` —— 无头模式，`t` 给你用于关闭的 id |

## 配方

**更新一个 toast**——用相同的 `id` 再次调用 `toast()`；只有你传入的 props 会改变。切换到 `toast.success(…, { id })` 会改变类型。这就是不用 `toast.promise` 实现 loading → success 流程的方式：

```jsx
const id = toast.loading('Uploading…');
toast.success('Uploaded', { id });
```

**持久化**——`{ duration: Infinity }`。**关闭**——`toast.dismiss(id)`，或 `toast.dismiss()` 关闭全部。**读取活动 toast**——在 React 中用 `useSonner()`，在 React 之外用 `toast.getActiveToasts()`。

**文本中的链接或组件**——为 title 或 description 传入一个函数：`toast(() => <a href="…">View</a>)`。

**多个 Toaster**——给每个一个 `id`，并用 `toast('…', { toasterId: 'canvas' })` 定向。没有 `toasterId` 时，每个 Toaster 都会渲染该 toast。

**关闭回调**——`onDismiss` 在点击关闭按钮或滑动关闭时触发；`onAutoClose` 在超时自动关闭时触发。它们是分开的；没有单一的 "closed" 回调。

## 样式 —— 升级阶梯

只爬到改动所需的层级；太早跳到顶级没问题（那是推荐的终态），停留在中间层级则不可取。

1. **默认值**——加上 Toaster 的 `richColors` 让 success/error 更鲜艳，`invert` 用于相对主题反转。
2. **内联微调**——Toaster 上的 `toastOptions={{ style: {…} }}` 作用于所有 toast，或每次 `toast()` 调用传 `style`。
3. **各部分的类名**——`toastOptions={{ classNames: { toast, title, description, actionButton, cancelButton, closeButton } }}`。Sonner 注入的样式在层叠中胜出，所以每个类都需要 `!important`（Tailwind：`!text-red-900`）。如果你标记 important 的地方超过寥寥几处，停手——改走无头模式。
4. **无头（headless）**——`toast.custom()` 配你自己的 JSX，保留 Sonner 的定位、堆叠和滑动。设计系统级 toast 的推荐做法：把它包进你自己的 `toast()` 抽象里。（`unstyled: true` 作为折中方案存在，但同样的投入下无头模式给你更多控制。）

**图标**——用 Toaster 的 `icons` prop 按类型替换默认图标，单个 toast 用 `icon`，传 `null` 移除。

**主题**——`theme` 默认为 `'light'`，且不跟随操作系统。传 `theme="system"`，或接上你的主题 provider：来自 `next-themes` 的 `<Toaster theme={resolvedTheme} />`。

## 排障

| 症状 | 原因 → 修复 |
| --- | --- |
| Toast 从不出现 | 没有挂载 `<Toaster />`，或它卸载了（条件渲染、按页面放置）。在根部挂载一个。如果从 server action 调用：`toast()` 仅限客户端——在客户端用 action 的结果调用它。 |
| 同一个 toast 出现两次 | 挂载了两个 Toaster（layout **和** page）——只保留一个。或者在 React StrictMode 开发环境双重调用下于 effect 中触发了 `toast()`——改从事件处理器触发，或传入稳定的 `id` 使第二次调用是更新而非重复。 |
| Tailwind/CSS 类无效 | 默认样式覆盖了它们。标记 `!important`，或使用 `unstyled` / 无头模式（见上方阶梯）。 |
| Toast 完全无样式渲染（常见于 Astro、view transitions） | Sonner 注入的样式表丢失——在 layout 中显式导入：`import 'sonner/dist/styles.css'`。 |
| Shadow DOM 内无样式 | 样式落在 `document.head`，而非 shadow root。把文本包含 `[data-sonner-toaster]` 的 style 标签复制进 shadow root。 |
| Toast 在模态框/遮罩后面，或被裁剪 | 某个祖先创建了层叠上下文（`transform`、`filter`、`overflow`），或遮罩的 z-index 高过 Toaster。把 `<Toaster />` 移到文档根部，移出任何 dialog/portal 容器。 |
| 暗色模式被忽略 | `theme` 默认为 `'light'`——设置 `theme="system"` 或传入解析后的主题（见上方主题）。 |
| Success/error 看起来是灰色而非绿色/红色 | 那是默认外观。给 Toaster 加 `richColors`。 |
| Toast 永不关闭 | `duration: Infinity`、`dismissible: false`，或一个 promise 永不 settle 的 `toast.promise`——loading toast 会一直等下去。 |
| `toast.promise` 卡在 loading | 它的第一个参数需要一个 promise（或返回 promise 的函数），且该 promise 必须真正 resolve/reject。 |
| 滑动关闭方向错误 / 不工作 | 方向由 `position` 推导。用 Toaster 的 `swipeDirections` 覆盖。 |
| Toast 出现在每个 Toaster 里 | 多 Toaster 需要定向：给每个 Toaster 一个 `id`，并在 `toast()` 调用中传 `toasterId`。 |
| 移动端 toast 离屏幕边缘太近 | `offset`（桌面端，默认 32px）和 `mobileOffset`（<600px，默认 16px）——接受数字、CSS 字符串或按边的对象。 |

---

# API 参考（API.md）

精确的 props、类型和默认值。传给 `toast()` 的选项会覆盖通过 Toaster 的 `toastOptions` 设置的同名选项。

## `<Toaster />`

| Prop | Type | Default | Description |
| --- | --- | --- | --- |
| `theme` | `string` | `'light'` | `'light'`, `'dark'`, or `'system'`. |
| `richColors` | `boolean` | `false` | Makes error and success states more colorful. |
| `expand` | `boolean` | `false` | Toasts expanded by default (otherwise they expand on hover). |
| `visibleToasts` | `number` | `3` | Amount of visible toasts. |
| `id` | `string` | – | Toaster id, targeted by `toast()`'s `toasterId` option. |
| `position` | `string` | `'bottom-right'` | `top-left`, `top-center`, `top-right`, `bottom-left`, `bottom-center`, `bottom-right`. |
| `closeButton` | `boolean` | `false` | Adds a close button to all toasts. |
| `offset` | `string \| number \| object` | `'32px'` | Offset from screen edges. Object form is per-side: `{ bottom: '24px', right: '16px' }`. |
| `mobileOffset` | `string \| number \| object` | `'16px'` | Offset when screen width < 600px. |
| `swipeDirections` | `array` | based on position | Allowed swipe-to-dismiss directions. |
| `dir` | `string` | `'ltr'` | Text directionality. |
| `hotkey` | `string` | `⌥/alt + T` | Keyboard shortcut that focuses the toaster area. |
| `invert` | `boolean` | `false` | Dark toasts in light mode and vice versa. |
| `toastOptions` | `object` | – | Default options applied to every toast (any `toast()` option below). |
| `gap` | `number` | `14` | Gap between toasts when expanded. |
| `icons` | `object` | – | Replace default icons: `{ success, info, warning, error, loading }`; `null` removes one. |

## `toast()` 选项

`toast(message, options)` —— message 是字符串、JSX 或返回 JSX 的函数。返回该 toast 的 id。

| Option | Type | Default | Description |
| --- | --- | --- | --- |
| `description` | `ReactNode` | – | Renders underneath the title; also accepts a function returning JSX. |
| `closeButton` | `boolean` | `false` | Adds a close button. |
| `invert` | `boolean` | `false` | Dark toast in light mode and vice versa. |
| `duration` | `number` | `4000` | Milliseconds before auto-close. `Infinity` persists the toast. |
| `position` | `string` | `'bottom-right'` | Position of this toast. |
| `dismissible` | `boolean` | `true` | If `false`, the user cannot dismiss the toast. |
| `icon` | `ReactNode` | – | Icon in front of the text; `null` removes the default. |
| `action` | `ReactNode \| { label, onClick }` | – | Primary button; clicking closes the toast unless `onClick` calls `event.preventDefault()`. |
| `cancel` | `ReactNode \| { label, onClick }` | – | Secondary button; clicking closes the toast. |
| `actionButtonStyle` | `object` | `{}` | Styles for the action button. |
| `cancelButtonStyle` | `object` | `{}` | Styles for the cancel button. |
| `id` | `string` | – | Custom id; calling `toast()` again with the same id updates the existing toast. |
| `testId` | `string` | – | Rendered as `data-testid` for e2e tests. |
| `toasterId` | `string` | – | Id of the toaster to render this toast in. |
| `style` | `object` | – | Inline styles for the toast. |
| `classNames` | `object` | – | Classes per part: `{ toast, title, description, actionButton, cancelButton, closeButton }`. Needs `!important` unless `unstyled`. |
| `unstyled` | `boolean` | `false` | Removes all default styles. |
| `onDismiss` | `(toast) => void` | – | Fires when the close button is clicked or the toast is swiped away. |
| `onAutoClose` | `(toast) => void` | – | Fires when the toast closes automatically after `duration`. |
| `containerAriaLabel` | `string` | `'Notifications'` | ARIA label for the toast container. |

## 函数

| Function | Purpose |
| --- | --- |
| `toast(message, opts?)` | Render a toast; returns its id. |
| `toast.success / .error / .info / .warning(message, opts?)` | Typed toast with matching icon. |
| `toast.loading(message, opts?)` | Toast with a spinner; update it by id. |
| `toast.promise(promise, { loading, success, error })` | Loading toast that resolves with the promise; `success`/`error` accept strings, JSX, functions of the result, or objects of toast options. |
| `toast.custom((t) => jsx, opts?)` | Headless toast — your JSX, Sonner's behavior. |
| `toast.dismiss(id?)` | Dismiss one toast, or all when called without an id. |
| `toast.getActiveToasts()` | All active toasts, usable outside React. |
| `useSonner()` | React hook returning `{ toasts }`. |
