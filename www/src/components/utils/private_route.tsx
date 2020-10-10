import React, { useContext } from "react";
import { Route, RouteProps, Redirect } from "react-router-dom";
import { auth_context } from "../../contexts/auth_context";
import validate_key from "../../helpers/validate_key";

import Loading from "../loading";

interface PrivateRouteProps extends RouteProps {
  c: JSX.Element;
}

const PrivateRoute = ({ c, ...rest }: PrivateRouteProps) => {
  const { auth } = useContext(auth_context);

  if (!auth || auth.loading) {
    return <Route {...rest} render={() => <Loading />} />;
  }

  return (
    <Route
      {...rest}
      render={() =>
        auth.key && validate_key(auth.key) ? c : <Redirect to={`/admin/sign-in`} />
      }
    />
  );
};

export default PrivateRoute;
