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
  const [wrong, set_wrong] = useState(false);

  const on_submit = async (e: React.FormEvent) => {
    e.preventDefault();

    const request_options = {
      method: "POST",
      body: key
    };

    if (await fetch("/api/admin/key", request_options).then((r) => r.ok)) {
      set_auth_data!(key);
      set_redirect(true);
    } else {
      set_wrong(true);
    }
  };

  return (redirect? <Redirect to="/admin/"/> : <form onSubmit={on_submit}>
    <Label>
      Key:
      <input type="password" name="key" placeholder="key" onChange={e => {set_key(e.target.value)}}/>
    </Label>
    <Input type="submit" value="Submit"/>
    {wrong? <Label>Wrong Key</Label> : <></>}
  </form>);
};

export default SignIn;
