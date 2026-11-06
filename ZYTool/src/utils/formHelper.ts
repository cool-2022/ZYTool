/**
 * FormHelper：EI 风格的后台服务调用助手，供增删改查页面统一使用。
 * - 子类 SysInfoHelper：采集当前系统信息（当前时间、电脑名、IP、当前画面ID 等）
 * - callService：统一调用后台，支持 EIInfo（自动填充 sys、按 EI 契约收发）与普通 JSON，
 *   底层走 ApiService 的 requestWithFallback（Rust 优先、Python 兜底）
 */
import type { AxiosRequestConfig } from 'axios';
import { EI } from './ei';
import { getBaseInfo, buildDefaultBaseInfo } from './auth';
import { requestWithFallback } from '../services/api';

/** 当前系统信息 */
export interface CurrentSysInfo {
  /** 当前时间，格式 YYYY-MM-DD HH:mm:ss */
  currentTime: string;
  /** 电脑名（浏览器无法直接获取，取登录时保存的 BaseInfo） */
  computerName: string;
  /** IP（同上，取 BaseInfo） */
  ip: string;
  /** 当前画面ID：路由 path，如 /tools/json */
  screenId: string;
  /** 当前画面名：document.title */
  screenName: string;
  /** 当前登录用户名 */
  username: string;
  userAgent: string;
  language: string;
}

function formatTime(date: Date): string {
  const pad = (n: number) => String(n).padStart(2, '0');
  return (
    `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())} ` +
    `${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`
  );
}

/** FormHelper 子类：当前系统信息采集 */
export class SysInfoHelper {
  /** 采集当前系统信息 */
  static collect(): CurrentSysInfo {
    const baseInfo = getBaseInfo() || buildDefaultBaseInfo();
    return {
      currentTime: formatTime(new Date()),
      computerName: baseInfo.computer_name || '',
      ip: baseInfo.ip || '',
      screenId: typeof window !== 'undefined' ? window.location.pathname : '',
      screenName: baseInfo.screen_name || (typeof document !== 'undefined' ? document.title : ''),
      username: baseInfo.username || '',
      userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : '',
      language: typeof navigator !== 'undefined' ? navigator.language : '',
    };
  }

  /**
   * 把系统信息写入 eiInfo.sys；service/method 一并写入 sys.serviceName/sys.methodName。
   * 同一画面重复调用会覆盖同名字段。
   */
  static applyToEI(eiInfo: EI.EIInfo, extra?: { service?: string; method?: string }): EI.EIInfo {
    Object.assign(eiInfo.sys, SysInfoHelper.collect());
    if (extra?.service) eiInfo.sys.serviceName = extra.service;
    if (extra?.method) eiInfo.sys.methodName = extra.method;
    return eiInfo;
  }
}

export interface CallServiceOptions {
  /** 请求路径（相对 apiPrefix），如 /ei/user/query */
  url: string;
  /** EI 风格服务名，写入 sys.serviceName */
  service?: string;
  /** EI 风格方法名，写入 sys.methodName */
  method?: string;
  /** 请求数据：EIInfo 实例或普通 JSON 对象 */
  data?: EI.EIInfo | Record<string, any>;
  /** HTTP 方法，默认 post；get/delete 时普通 JSON 会并入 query params（EIInfo 始终放 body） */
  httpMethod?: 'post' | 'get' | 'put' | 'delete';
  /** EIInfo 数据时是否自动填充系统信息到 sys，默认 true */
  fillSysInfo?: boolean;
  /**
   * 响应解析方式：
   * - 'auto'（默认）：响应形如 EI JSON（含 blocks 或 sys+status）时解析为 EIInfo，否则原样返回
   * - 'ei'：强制按 EIInfo 解析
   * - 'json'：原样返回
   */
  responseType?: 'auto' | 'ei' | 'json';
  /** 透传 axios 配置 */
  config?: AxiosRequestConfig;
}

/** 判断响应是否形如 EIInfo JSON */
function looksLikeEiJson(value: any): boolean {
  return (
    !!value &&
    typeof value === 'object' &&
    !Array.isArray(value) &&
    ('blocks' in value || ('sys' in value && 'status' in value))
  );
}

/** FormHelper 方法类 */
export class FormHelper {
  /** 子类：当前系统信息采集，等价直接使用 SysInfoHelper */
  static readonly SysInfo = SysInfoHelper;

  /** 创建已填充系统信息的 EIInfo */
  static createEIInfo(name = '', extra?: { service?: string; method?: string }): EI.EIInfo {
    const eiInfo = new EI.EIInfo();
    eiInfo.setName(name);
    return SysInfoHelper.applyToEI(eiInfo, extra);
  }

  /**
   * 统一调用后台。
   * data 为 EIInfo 时按 EI 契约收发（自动填充 sys、toJSON 序列化）；
   * 为普通 JSON 时原样传递，响应默认原样返回。
   */
  static async callService(options: CallServiceOptions): Promise<any> {
    const {
      url,
      service,
      method,
      data,
      httpMethod = 'post',
      fillSysInfo = true,
      responseType = 'auto',
      config,
    } = options;

    const isEi = data instanceof EI.EIInfo;
    let payload: any = data;
    if (isEi) {
      if (fillSysInfo) SysInfoHelper.applyToEI(data, { service, method });
      payload = data.toJSON();
    }

    const requestConfig: AxiosRequestConfig = { ...config };
    let body: any = payload;
    if (!isEi && (httpMethod === 'get' || httpMethod === 'delete') && payload && typeof payload === 'object') {
      requestConfig.params = { ...payload, ...requestConfig.params };
      body = undefined;
    }

    const result: any = await requestWithFallback(httpMethod, url, body, requestConfig);

    if (responseType === 'json') return result;
    if (responseType === 'ei' || (responseType === 'auto' && looksLikeEiJson(result))) {
      return EI.EIInfo.fromJSON(result);
    }
    return result;
  }

  /** 便捷方法：EIInfo 进、EIInfo 出 */
  static async callEiService(
    url: string,
    eiInfo: EI.EIInfo,
    options: Omit<CallServiceOptions, 'url' | 'data'> = {}
  ): Promise<EI.EIInfo> {
    const result = await FormHelper.callService({ ...options, url, data: eiInfo, responseType: 'ei' });
    return result as EI.EIInfo;
  }
}
