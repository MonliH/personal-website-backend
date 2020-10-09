import React, { useContext } from 'react';
import { Route, RouteProps, Redirect, useRouteMatch } from 'react-router-dom';
import { auth_context } from '../../contexts/auth_context';

interface PrivateRouteProps {
    component: JSX.Element,
    rest?: RouteProps,
}

const PrivateRoute = ({ component, ...rest }: PrivateRouteProps) => {
  const { auth } = useContext(auth_context);
  const { path } = useRouteMatch();

  return (
    <Route
      {...rest}
      render={() => (
        auth ? component : <Redirect to={`${path}sign-in`} />
      )}
    />

  );
};

export default PrivateRoute;
