export interface BlogEntry {
  readonly title: string;
  readonly url: string;
  readonly date: Date;
  readonly contents: string;
}

export const BLOG_COLOR_BG = "#FAFAFA";

export const into_blog_entry = (json: any): BlogEntry => {
  json.date = new Date(json.date);
  return json as BlogEntry;
};
