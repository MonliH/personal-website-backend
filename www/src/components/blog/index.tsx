import React from "react";
import { Switch, Route, useRouteMatch } from "react-router-dom";

import BlogHome from "./blog_home";
import BlogPage from "./blog_page";

const Blog = () => {
  const { path } = useRouteMatch();

  return (
    <Switch>
      <Route exact path={`${path}/`}>
        <BlogHome/>
      </Route>
      <Route path="*">
        <BlogPage />
      </Route>
    </Switch>
  );
};

export default Blog;
