/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export type JsPublisher = Publisher
export class Publisher { }
export namespace path {
  export function root(): string
  export function isAbsolute(path: string): boolean
  export function isParent(parent: string, other: string): boolean
  export function isImmediateParent(parent: string, other: string): boolean
  export function stripPrefix(prefix: string, path: string): string | null
  export function lcp(path0: string, path1: string): string
  export function escape(path: string): string
  export function unescape(path: string): string
  export function append(path: string, other: string): string
  export function parts(path: string): Array<string>
  export function dirnames(path: string): Array<string>
  export function levels(path: string): number
  export function dirname(path: string): string | null
  export function dirnameWithSep(path: string): string | null
  export function basename(path: string): string | null
  export function rfindSep(path: string): number | null
  export function findSep(path: string): number | null
}
