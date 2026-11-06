/**
 * EI 平台（EiInfo）数据交换结构的基础类型。
 * 与 Rust 后端 models/ei.rs 保持一致，前后端增删改查统一按此结构传递。
 */

/** 单元格值类型 */
export type ValueType = string | number | boolean | Date | null | undefined | Record<string, any> | any[];

/** 字段数据类型：EI 契约中为字符串，这里列出常见取值并兼容其他字符串 */
export type DataType = 'string' | 'number' | 'boolean' | 'date' | 'datetime' | 'object' | (string & {});

/** 列元数据 JSON 结构 */
export interface IColumnJson {
  pos: number;
  name: string;
  type: DataType;
  descName: string;
  regex?: string;
  formatter?: string;
  editor?: string;
  minLength?: number;
  maxLength?: number;
  primaryKey?: boolean;
  nullable?: boolean;
  visible?: string;
  readonly?: string;
  displayType?: string;
  errorPrompt?: string;
  width?: number;
  height?: number;
  align?: string;
  blockName?: string;
  sourceName?: string;
  labelProperty?: string;
  valueProperty?: string;
}

/** toJSON 时的数据转换器，可按列元数据 / 行数据分别转换 */
export interface IServiceTransformer {
  transformColumn?(column: IColumnJson): IColumnJson;
  transformRow?(row: { [key: string]: ValueType }): { [key: string]: ValueType };
}
