/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum Language {
  Eclair = 0,
  TS = 1,
  Bash = 2,
  LLVM = 3,
  C = 4,
  Rust = 5,
  Haskell = 6
}
export function highlight(code: string, language: Language): string
export interface HastProperties {
  className: string
}
export interface HastNode {
  type: string
  tagName: string
  properties: HastProperties
  children: Array<HastNode | HastTextNode>
}
export interface HastTextNode {
  type: string
  value: string
}
export function highlightHast(code: string, language: Language): HastNode
