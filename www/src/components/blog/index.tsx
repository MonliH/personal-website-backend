import React, { useState, useEffect } from "react";
import { Switch, Route, useRouteMatch } from "react-router-dom";

import BlogHome from "./blog_home";
import BlogPage from "./blog_page";
import BlogPageChanger from "./blog_page_changer";

import useBlogEntries from "../../hooks/useBlogEntries";
import useBg from "../../hooks/useBg";

const Blog = () => {
  const { path } = useRouteMatch();
  const posts_per_page = 10;

  const [page_no, set_page_no, blog_entries, loading] = useBlogEntries(
    posts_per_page
  );

  useBg("#FFFFFF");

  return (
    <Switch>
      <Route exact path={`${path}/`}>
        <BlogPageChanger
          current_page={page_no}
          set_page={set_page_no}
          total_pages={Math.ceil(blog_entries.length / posts_per_page)}
        />
        <BlogHome blog_entries={blog_entries} loading={loading} />
      </Route>
      <Route path="*">
        <BlogPage />
      </Route>
    </Switch>
  );
};

export default Blog;
