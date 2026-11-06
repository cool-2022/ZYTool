/**
 * EI 平台（EiInfo）数据交换结构的前端实现。
 * 经 index.ts 以 `export * as EI` 导出，使用方式为 `new EI.EIInfo()`、`EI.EiBlock.build(...)` 等；
 * 结构与 Rust 后端 models/ei.rs 一一对应，JSON 线格式一致（block 序列化为 meta.columns + rows）。
 */
import type { DataType, IColumnJson, IServiceTransformer, ValueType } from './type';
import type * as EITools from './ei-tools';

function deepClone<T>(value: T): T {
  if (value === null || typeof value !== 'object') return value;
  if (value instanceof Date) return new Date(value.getTime()) as T;
  if (value instanceof Map) {
    const out = new Map<any, any>();
    value.forEach((v, k) => out.set(k, deepClone(v)));
    return out as T;
  }
  if (Array.isArray(value)) return value.map((item) => deepClone(item)) as T;
  const out: { [key: string]: any } = {};
  for (const [k, v] of Object.entries(value)) out[k] = deepClone(v);
  return out as T;
}

function inferDataType(value: ValueType): DataType {
  if (value instanceof Date) return 'date';
  switch (typeof value) {
    case 'number':
      return 'number';
    case 'boolean':
      return 'boolean';
    case 'object':
      return 'object';
    default:
      return 'string';
  }
}

/** 解析三段式 key：blockName-rowNum-colName（列名允许含 '-'），与 Rust 端 parse_triple 行为一致 */
function parseTriple(key: string): [string, number, string] | null {
  const i1 = key.indexOf('-');
  if (i1 <= 0) return null;
  const i2 = key.indexOf('-', i1 + 1);
  if (i2 < 0) return null;
  const blockName = key.slice(0, i1);
  const rowNum = Number(key.slice(i1 + 1, i2));
  const colName = key.slice(i2 + 1);
  if (!colName || !Number.isInteger(rowNum) || rowNum < 0) return null;
  return [blockName, rowNum, colName];
}

/** 列元数据 */
export class EiColumn implements IColumnJson {
  [key: string]: any;
  pos = 0;
  name = '';
  type: DataType = 'string';
  descName = '';
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

/** ArrayObject 结构的 Block：data 为行对象数组，columns 为列元数据 */
export class EiBlock {
  name: string;
  data: { [key: string]: ValueType }[] = [];
  columns: EiColumn[] = [];
  extAttr: Map<string, any> = new Map();

  constructor(blockName = '') {
    this.name = blockName;
  }

  /** 新增一条或多条数据；autoAddColumn 为 true 时自动为未知键生成列元数据 */
  pushData(data: { [key: string]: ValueType } | { [key: string]: ValueType }[], autoAddColumn = false): void {
    const rows = Array.isArray(data) ? data : [data];
    for (const row of rows) {
      if (autoAddColumn) {
        for (const key of Object.keys(row)) {
          if (!this.containsColumn(key)) this.addColumn(key, '', inferDataType(row[key]));
        }
      }
      this.data.push(row);
    }
  }

  /** 获取元数据信息 */
  getBlockMeta(): IColumnJson[] {
    return this.columns;
  }

  /** 设置元数据信息 */
  setBlockMeta(meta: IColumnJson[]): void {
    this.columns = meta.map((m) => (m instanceof EiColumn ? m : Object.assign(new EiColumn(), m)));
  }

  /** 根据列名获取列对象 */
  getColumn(columnName: string): EiColumn | undefined {
    return this.columns.find((c) => c.name === columnName);
  }

  /** 删除某个字段（连同各行数据中的该键） */
  removeColumn(columnName: string): EiBlock {
    this.columns = this.columns.filter((c) => c.name !== columnName);
    for (const row of this.data) delete row[columnName];
    return this;
  }

  /** 检测元数据中是否包含指定字段名 */
  containsColumn(columnName: string): boolean {
    return this.columns.some((c) => c.name === columnName);
  }

  addColumn(column: EiColumn, defaultValue?: ValueType): void;
  addColumn(newColumn: string, defaultValue?: ValueType, dataType?: DataType): void;
  /** 添加一列；已存在同名列时忽略，defaultValue 写入既有行（不传默认为 ''） */
  addColumn(column: EiColumn | string, defaultValue: ValueType = '', dataType: DataType = 'string'): void {
    const col =
      typeof column === 'string'
        ? Object.assign(new EiColumn(), { name: column, descName: column, type: dataType, pos: this.columns.length })
        : column;
    if (this.containsColumn(col.name)) return;
    this.columns.push(col);
    for (const row of this.data) {
      if (!(col.name in row)) row[col.name] = defaultValue;
    }
  }

  addColumns(...columnNames: string[]): void;
  addColumns(columnArray: EiColumn[]): void;
  /** 添加多列：列名字符串或 EiColumn 数组 */
  addColumns(...args: (string | EiColumn[])[]): void {
    if (args.length === 1 && Array.isArray(args[0])) {
      for (const col of args[0] as EiColumn[]) this.addColumn(col);
      return;
    }
    for (const name of args as string[]) this.addColumn(name);
  }

  /** 重命名字段名（连同各行数据中的键） */
  renameColumn(oldColumnName: string, newColumnName: string): EiBlock {
    const col = this.getColumn(oldColumnName);
    if (col) col.name = newColumnName;
    for (const row of this.data) {
      if (oldColumnName in row) {
        row[newColumnName] = row[oldColumnName];
        delete row[oldColumnName];
      }
    }
    return this;
  }

  /** 复制表（含数据行）；不设置块名时为「原表名 copy」 */
  copy(blockName?: string): EiBlock {
    return this.fullClone(blockName ?? `${this.name} copy`, true);
  }

  /** 复制表；excludeRows 为 true 时不带数据行 */
  clone(blockName: string, excludeRows: boolean): EiBlock {
    return this.fullClone(blockName, !excludeRows);
  }

  private fullClone(blockName: string, includeRows: boolean): EiBlock {
    const block = new EiBlock(blockName);
    block.columns = this.columns.map((c) => Object.assign(new EiColumn(), deepClone({ ...c })));
    block.extAttr = deepClone(this.extAttr);
    block.data = includeRows ? deepClone(this.data) : [];
    return block;
  }

  /** 添加一行数据 */
  addRow(rowData: { [key: string]: ValueType }): EiBlock {
    this.data.push(rowData);
    return this;
  }

  /** 删除指定行，返回被删除的数据；index 越界时返回 undefined（对应 Rust 端 Option） */
  deleteRow(index: number): { [key: string]: ValueType } | undefined {
    if (index < 0 || index >= this.data.length) return undefined;
    return this.data.splice(index, 1)[0];
  }

  /** 在指定位置插入一行 */
  insertRow(index: number, rowData: { [key: string]: ValueType }): EiBlock {
    this.data.splice(index, 0, rowData);
    return this;
  }

  toJSON(dataTransformer?: IServiceTransformer): EITools.__BlockJson__ {
    let columns: IColumnJson[] = this.columns.map((c) => ({ ...c }));
    let rows = this.data.map((r) => ({ ...r }));
    const { transformColumn, transformRow } = dataTransformer ?? {};
    if (transformColumn) columns = columns.map((c) => transformColumn(c));
    if (transformRow) rows = rows.map((r) => transformRow(r));
    const json: EITools.__BlockJson__ = { meta: { columns }, rows };
    if (this.extAttr.size > 0) json.extAttr = Object.fromEntries(this.extAttr);
    return json;
  }

  /** 通过列名数组或数据行数组创建 EiBlock（数据行形式自动生成列元数据） */
  static build(blockId: string, data: string[] | { [key: string]: ValueType }[]): EiBlock {
    const block = new EiBlock(blockId);
    if (data.length > 0 && typeof data[0] === 'string') {
      block.addColumns(...(data as string[]));
    } else {
      block.pushData(data as { [key: string]: ValueType }[], true);
    }
    return block;
  }

  /** 通过列英文名、列中文名数组创建 EiBlock */
  static buildByNames(blockId: string, enames: string[], cnames: string[]): EiBlock {
    const block = new EiBlock(blockId);
    enames.forEach((ename, i) => {
      block.addColumn(Object.assign(new EiColumn(), { name: ename, descName: cnames[i] ?? ename, pos: i }));
    });
    return block;
  }

  /** 通过 [{field, title, ...}] 数组创建 EiBlock */
  static buildByColumns(blockId: string, columns: { field: string; title: string; [key: string]: ValueType }[]): EiBlock {
    const block = new EiBlock(blockId);
    columns.forEach((c, i) => {
      const { field, title, ...rest } = c;
      block.addColumn(Object.assign(new EiColumn(), rest, { name: field, descName: title, pos: i }));
    });
    return block;
  }

  /** 从 JSON 构建 EiBlock（toJSON 的逆操作） */
  static fromJSON(blockName: string, json: EITools.__BlockJson__): EiBlock {
    const block = new EiBlock(blockName);
    block.columns = (json?.meta?.columns ?? []).map((c) => Object.assign(new EiColumn(), c));
    block.data = json?.rows ?? [];
    if (json?.extAttr) block.extAttr = new Map(Object.entries(json.extAttr));
    return block;
  }
}

/** EDInfo：fblk_name/func_name 定长 63，pk_name/pk_val 定长 3 */
export class EDInfo implements EITools.__IEDInfo__ {
  fblk_name = new Array<string>(63).fill('') as EITools.Tuple<string, 63>;
  func_name = new Array<string>(63).fill('') as EITools.Tuple<string, 63>;
  pk_name = new Array<string>(3).fill('') as EITools.Tuple<string, 3>;
  pk_val = new Array<string>(3).fill('') as EITools.Tuple<string, 3>;
}

/** ArrayObject 结构的 EIInfo */
export class EIInfo {
  context: EITools.IContent = {};
  sys: EITools.SysInfo = {};
  blocks: { [key: string]: EiBlock } = {};
  name = '';
  descName = '';
  msg = '';
  msgKey = '';
  detailMsg = '';
  status = 0;
  extAttr: { [key: string]: any } = {};
  edInfo?: EDInfo;

  /** 取得 EiInfo 的名称 */
  getName(): string {
    return this.name;
  }

  /** 设置 EiInfo 的名称 */
  setName(name: string): void {
    this.name = name;
  }

  /** 给 EI 添加 EDInfo 属性 */
  setED(ed: EDInfo): void {
    this.edInfo = ed;
  }

  /** 添加 EiBlock 块；blockName 不传则以 block.name 作为键名 */
  addBlock(eiBlock: EiBlock, blockName?: string): EiBlock {
    const key = blockName ?? eiBlock.name;
    if (!eiBlock.name) eiBlock.name = key;
    this.blocks[key] = eiBlock;
    return eiBlock;
  }

  /** 检查是否存在 block 块 */
  contains(blockName: string): boolean {
    return blockName in this.blocks;
  }

  /** 根据块名删除 block 块，返回被删除的块；块不存在时返回 undefined（对应 Rust 端 Option） */
  remove(blockName: string): EiBlock | undefined {
    const block = this.blocks[blockName];
    if (!block) return undefined;
    delete this.blocks[blockName];
    return block;
  }

  /** 重命名 block 名（键名与块自身的 name 一并更新）；原块不存在时返回 undefined */
  renameBlock(oldBlockName: string, newBlockName: string): EiBlock | undefined {
    const block = this.blocks[oldBlockName];
    if (!block) return undefined;
    delete this.blocks[oldBlockName];
    block.name = newBlockName;
    this.blocks[newBlockName] = block;
    return block;
  }

  /** 根据块名或索引号（按插入顺序）获取 block 块；不存在时返回 undefined（对应 Rust 端 Option） */
  getBlock(blockInfo: string | number): EiBlock | undefined {
    if (typeof blockInfo === 'number') return Object.values(this.blocks)[blockInfo];
    return this.blocks[blockInfo];
  }

  /** 克隆 EiInfo（深拷贝） */
  clone(): EIInfo {
    const info = new EIInfo();
    info.context = deepClone(this.context);
    info.sys = deepClone(this.sys);
    info.name = this.name;
    info.descName = this.descName;
    info.msg = this.msg;
    info.msgKey = this.msgKey;
    info.detailMsg = this.detailMsg;
    info.status = this.status;
    info.extAttr = deepClone(this.extAttr);
    if (this.edInfo) info.edInfo = deepClone(this.edInfo);
    for (const [key, block] of Object.entries(this.blocks)) {
      info.blocks[key] = block.copy(key);
    }
    return info;
  }

  /** 三段式取数：blockName-rowNum-colName，如 'result-0-siteArticleButton'；取不到返回 null */
  get(str: string): any {
    const triple = parseTriple(str);
    if (!triple) return null;
    const [blockName, rowNum, colName] = triple;
    const row = this.blocks[blockName]?.data[rowNum];
    if (!row) return null;
    const value = row[colName];
    return value === undefined ? null : value;
  }

  /**
   * 三段式设值。支持两种形式：
   * set('result-0-name', value) 或 set('result', 0, 'name', value)；
   * block 不存在时忽略，行号超出时自动补空行。
   */
  set(...argumentArr: ValueType[]): void {
    if (argumentArr.length === 2 && typeof argumentArr[0] === 'string') {
      const triple = parseTriple(argumentArr[0]);
      if (!triple) return;
      this.setCell(triple[0], triple[1], triple[2], argumentArr[1]);
      return;
    }
    if (argumentArr.length >= 4) {
      const [blockName, rowNum, colName, value] = argumentArr;
      if (typeof blockName !== 'string' || typeof rowNum !== 'number' || typeof colName !== 'string') return;
      this.setCell(blockName, rowNum, colName, value);
    }
  }

  private setCell(blockName: string, rowNum: number, colName: string, value: ValueType): void {
    const block = this.blocks[blockName];
    if (!block) return;
    while (block.data.length <= rowNum) block.data.push({});
    const row = block.data[rowNum];
    if (row) row[colName] = value;
  }

  /** 转换为 JSON（与后端 models/ei.rs 序列化结构一致） */
  toJSON(dataTransformer?: IServiceTransformer): EITools.__EIInfoJson__ {
    const blocks: { [key: string]: EITools.__BlockJson__ } = {};
    for (const [key, block] of Object.entries(this.blocks)) {
      blocks[key] = block.toJSON(dataTransformer);
    }
    const json: EITools.__EIInfoJson__ = {
      name: this.name,
      descName: this.descName,
      msg: this.msg,
      msgKey: this.msgKey,
      detailMsg: this.detailMsg,
      status: this.status,
      context: { ...this.context },
      sys: { ...this.sys },
      extAttr: { ...this.extAttr },
      blocks,
    };
    if (this.edInfo) json.edInfo = { ...this.edInfo };
    return json;
  }

  /** 从 JSON 构建 EIInfo（toJSON 的逆操作，用于解析后台 EI 响应） */
  static fromJSON(json: EITools.__EIInfoJson__): EIInfo {
    const info = new EIInfo();
    if (!json || typeof json !== 'object') return info;
    info.name = json.name ?? '';
    info.descName = json.descName ?? '';
    info.msg = json.msg ?? '';
    info.msgKey = json.msgKey ?? '';
    info.detailMsg = json.detailMsg ?? '';
    info.status = json.status ?? 0;
    info.context = json.context ?? {};
    info.sys = json.sys ?? {};
    info.extAttr = json.extAttr ?? {};
    if (json.edInfo) info.edInfo = Object.assign(new EDInfo(), json.edInfo);
    for (const [key, blockJson] of Object.entries(json.blocks ?? {})) {
      info.blocks[key] = EiBlock.fromJSON(key, blockJson);
    }
    return info;
  }
}
