import React, { useContext, useState } from "react";
import {Redirect} from "react-router-dom";

import { auth_context } from "../../contexts/auth_context";
import styled from "styled-components";

const Label = styled.label`
  color: black
`;

const Input = styled.input`

`;

const SignIn = () => {
  const { set_auth_data } = useContext(auth_context);

  const [key, set_key] = useState();
  const [redirect, set_redirect] = useState(false);

  const on_submit = (e: React.FormEvent) => {
    e.preventDefault();
    set_auth_data!(key);
    set_redirect(true);
  };

  return (redirect? <Redirect to="/admin/"/> : <form onSubmit={on_submit}>
    <Label>
      Key:
      <input type="password" name="key" placeholder="key" onChange={e => {set_key(e.target.value)}}/>
    </Label>
    <Input type="submit" value="Submit"/>
  </form>);
};

export default SignIn;
