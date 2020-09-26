export interface BlogEntry {
  readonly title: string;
  readonly url: string;
  readonly date: Date;
  readonly contents: JSX.Element;
}

export const entries: Array<string> = [
  "wasm_react_ts",
];
