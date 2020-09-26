import React, {useState, useEffect} from "react";
import { Switch, Route, useRouteMatch } from "react-router-dom";

import NoMatch from "../404";
import BlogHome from "./blog_home";
import BlogPage from "./blog_page";

import { entries, BlogEntry } from "../../data/blog/entries";

const Blog = () => {
  const { path } = useRouteMatch();
  const [blog_entries, set_blog_entries] = useState<Array<BlogEntry>>([]);

  // Start loading blogs
  useEffect(() => {
    entries.map(async (entry_loc: string, idx: number) => {
      let entry = await import(/* webpackPrefetch: true */ `../../data/blog/posts/${entry_loc}`);
      set_blog_entries([...blog_entries, entry.entry]);
    });
  // We can disable this lint because we don't care about
  // `bloge_entries` changing
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const routes = blog_entries.map((blog: BlogEntry, idx: number) => {
    return <Route key={idx}>
      <BlogPage blog={blog}></BlogPage>
    </Route>;
  });

  return (
    <Switch>
      <Route exact path={`${path}/`}>
        <BlogHome blog_entries={blog_entries}/>
      </Route>
      {routes}
      <Route path="*">
        <NoMatch />
      </Route>
    </Switch>
  );
};

export default Blog;
