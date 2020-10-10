import React from "react";
import { Route, Switch, useRouteMatch } from "react-router-dom";

import SignIn from "./sign_in";
import Panel from "./panel";
import PrivateRoute from "../utils/private_route";
import AdminBlogPage from "./admin_blog_page";
import BlogPageChanger from "../blog/blog_page_changer";

import useBlogEntries from "../../hooks/useBlogEntries";
import { posts_per_page } from "../blog/index";

import useBg from "../../hooks/useBg";

const AdminPanel = () => {
  const { path } = useRouteMatch();
  useBg("#FFFFFF");

  const [pages, page_no, set_page_no, blog_entries, loading] = useBlogEntries(
    posts_per_page
  );

  return (
    <Switch>
      <PrivateRoute
        exact
        path={`${path}/`}
        c={
          <>
            <BlogPageChanger
              current_page={page_no}
              set_page={set_page_no}
              total_pages={pages}
            />
            <Panel loading={loading} blog_entries={blog_entries} />
          </>
        }
      />
      <Route exact path={`${path}/sign-in`}>
        <SignIn />
      </Route>
      <PrivateRoute path={`${path}/*`} c={<AdminBlogPage />} />
    </Switch>
  );
};

export default AdminPanel;
