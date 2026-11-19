# 11 UI 动画设计规范

> 迁移自原前端知识库 `frontend-knowledge/resources/`，2026-11-18 核实。

本目录收录 ZYTool 前端的 UI 动画与设计规范，分摘要与完整版两层：

## 资源索引

- [05-ui-animation-design.md](05-ui-animation-design.md) — **摘要速查**：动画取值标准（easing/时长/弹簧）、常用配方、评审标准、Apple 设计原则、库选型、反模式清单。涉及动效编写/评审/组件选型时必读。
- [ui-animation-skills/](ui-animation-skills/00-索引.md) — 上述规范的**完整中文版**（00-索引 ~ 10，共 11 个文件），自包含、可直接移植到其他项目；需要完整规则细节时按其 `00-索引.md` 定位。

## 项目动效约定（速记）

项目内动效必须走全局 token（`ZYTool/src/styles/variables.css`），详见 [../06-前端架构.md](../06-前端架构.md)「关键约定」一节：

- easing：`--ease-out / --ease-in-out / --ease-drawer`
- 时长：`--duration-press(160ms) / fast(150ms) / base(200ms) / slow(250ms) / enter(300ms)`
- 禁止 `transition: all`、UI 禁 `ease-in`；按压反馈 `:active { transform: scale(0.97) }`
- hover 动效用 `@media (hover: hover) and (pointer: fine)` 门控；reduced-motion 降级非归零
