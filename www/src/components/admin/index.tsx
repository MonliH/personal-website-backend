import React  from "react";
import { Route, Switch, useRouteMatch } from "react-router-dom";

import SignIn from "./sign_in";
import Panel from "./panel";
import PrivateRoute from "../utils/private_route";

import useBg from "../../hooks/useBg";

import {auth_context} from "../../contexts/auth_context";

const AdminPanel = () => {
  const { path } = useRouteMatch();
  useBg("#FFFFFF");

  return (
    <Switch>
      <Route exact path={`${path}/`}>
        <PrivateRoute component={<Panel/>}/>
      </Route>
      <Route exact path={`${path}/sign-in`}>
        <SignIn />
      </Route>
    </Switch>
  );
};

export default AdminPanel;
