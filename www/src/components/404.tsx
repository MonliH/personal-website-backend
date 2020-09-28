import React from "react";
import styled from "styled-components";

const NoMatchStyled = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
`;

const NoMatch = () => {
  return <NoMatchStyled>404</NoMatchStyled>;
};

export default NoMatch;
