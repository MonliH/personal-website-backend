import React, {useContext} from "react";
import styled from "styled-components";

import {auth_context} from "../../contexts/auth_context";

const Title = styled.div`
  color: black;
`;

const Panel = () => {
  const { set_auth_data } = useContext(auth_context);
  const on_log_out = () => {
    set_auth_data!(undefined);
  };

  return <Title>Token</Title>;
};

export default Panel;
