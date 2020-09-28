import React, { useState, useEffect } from "react";
import { Switch, Route, useRouteMatch } from "react-router-dom";

import BlogHome from "./blog_home";
import BlogPage from "./blog_page";

import { BlogEntry } from "../../data/blog";

import yaml from "js-yaml";

const Blog = () => {
  const { path } = useRouteMatch();

  const [blog_entries, set_blog_entries] = useState<Array<BlogEntry>>([]);
  const [page_no, set_page_no] = useState(0);

  const posts_per_page = 10;

  const fetch_entries = async () => {
    const blog_pages_num = parseInt(
      await (await fetch("/api/blog/pages")).text()
    );
    const page_start = posts_per_page * page_no;
    const possible_end = page_start + posts_per_page;
    const entries_res = await fetch(
      `/api/blog/entries/${page_start}/${
        possible_end > blog_pages_num ? blog_pages_num : possible_end
      }`
    );
    const entries: Array<BlogEntry> = yaml.loadAll(await entries_res.text());
    set_blog_entries(entries);
  };

  // eslint-disable-next-line react-hooks/exhaustive-deps
  useEffect(() => {
    fetch_entries();
  }, [page_no]);

  useEffect(() => {
    document.body.style.backgroundColor = "#FFFFFF";
  }, []);

  return (
    <Switch>
      <Route exact path={`${path}/`}>
        <BlogHome blog_entries={blog_entries} />
      </Route>
      <Route path="*">
        <BlogPage />
      </Route>
    </Switch>
  );
};

export default Blog;
