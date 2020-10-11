import React from "react";
import { Switch, Route, useLocation } from "react-router-dom";

import BlogHome from "./blog_home";
import BlogPage from "./blog_page";

import useBlogEntries from "../../hooks/useBlogEntries";
import useBg from "../../hooks/useBg";

export const posts_per_page = 10;

const Blog = () => {
 const location = useLocation().pathname;

  const [pages, page_no, set_page_no, blog_entries, loading] = useBlogEntries(
    posts_per_page
  );

  useBg("#FFFFFF");

  return (
    <Switch>
      <Route exact path={"/blog/"}>
        <BlogHome
          current_page={page_no}
          set_page={set_page_no}
          total_pages={pages}
          blog_entries={blog_entries}
          loading={loading}
        />
      </Route>
      <Route path="*">
        <BlogPage blog_url={location.split("/").pop() as string} />
      </Route>
    </Switch>
  );
};

export default Blog;
