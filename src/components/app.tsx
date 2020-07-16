import React from "react";

import Home from "./home";
import Blog from "./blog";

import "../css/app.css";

import {
  BrowserRouter as Router,
  Switch,
  Route,
} from "react-router-dom";

const App = () => {
    return (
        <Router>
            <Switch>
                <Route path="/blog">
                    <Blog />
                </Route>
                <Route path="/">
                    <Home />
                </Route>
            </Switch>
        </Router>
    );
}


export default App;
