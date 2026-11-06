/**
 * EI 平台数据交换结构统一入口。
 * 使用：`import { EI } from '@/utils/ei'`，再 `new EI.EIInfo()`、`EI.EiBlock.build(...)`；
 * JSON 契约类型经 `EITools.__EIInfoJson__` 等访问。
 */
export * from './type';
export * as EITools from './ei-tools';
export * as EI from './ei';
