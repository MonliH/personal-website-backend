import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import Home from "./home/index";
import Blog from "./blog/index";
import AdminPanel from "./admin/index";
import Err from "./error";

import { Globals } from "react-spring";

// Disable animations if firefox resist fingerprinting is on
// Really laggy/buggy if it's on
if (performance.mark && performance.getEntries) {
  performance.mark("dummy_check");
  const entries = performance.getEntries();
  Globals.assign({
    skipAnimation: entries && entries.length === 0,
  });
}

const Main = () => {
  return (
    <Router>
      <Switch>
        <Route exact path="/">
          <Home />
        </Route>
        <Route path="/blog">
          <Blog />
        </Route>
        <Route path="/admin">
          <AdminPanel />
        </Route>
        <Route path="*">
          <Err />
        </Route>
      </Switch>
    </Router>
  );
};

export default Main;
