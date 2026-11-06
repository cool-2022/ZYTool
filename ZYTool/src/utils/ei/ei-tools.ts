/**
 * EI 平台数据交换结构的 JSON 契约类型（对应原 EITools 命名空间）。
 * 因 tsconfig 开启 erasableSyntaxOnly，不使用 namespace，经 index.ts 以 `export * as EITools` 导出。
 */
import type { IColumnJson, ValueType } from './type';

/** EiInfo.context：自由键值上下文 */
export interface IContent {
  [key: string]: any;
}

/** EiInfo.sys：系统信息，字段不确定，原样透传 */
export interface SysInfo {
  [key: string]: any;
}

/** 定长元组类型，如 Tuple<string, 63> */
export type Tuple<T, N extends number> = number extends N ? T[] : _TupleOf<T, N, []>;
type _TupleOf<T, N extends number, R extends unknown[]> = R['length'] extends N ? R : _TupleOf<T, N, [T, ...R]>;

/** Block 的 JSON 结构（与后端约定一致：meta 承载列元数据，rows 承载数据行） */
export interface __BlockJson__ {
  meta: {
    descName?: string;
    columns: IColumnJson[];
    [key: string]: any;
  };
  rows: { [key: string]: ValueType }[];
  extAttr?: { [key: string]: any };
}

/** EDInfo 的 JSON 结构：fblk_name/func_name 定长 63，pk_name/pk_val 定长 3 */
export interface __IEDInfo__ {
  fblk_name: Tuple<string, 63>;
  func_name: Tuple<string, 63>;
  pk_name: Tuple<string, 3>;
  pk_val: Tuple<string, 3>;
}

/** EiInfo 的 JSON 结构 */
export interface __EIInfoJson__ {
  name: string;
  descName: string;
  msg: string;
  msgKey: string;
  detailMsg: string;
  status: number;
  context: IContent;
  sys: SysInfo;
  extAttr: { [key: string]: any };
  edInfo?: __IEDInfo__;
  blocks: { [key: string]: __BlockJson__ };
}
