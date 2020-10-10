import React, { useContext } from "react";
import { useRouteMatch } from "react-router-dom";

import styled from "styled-components";

import { auth_context } from "../../contexts/auth_context";
import { BlogSummaryList } from "../blog/blog_home";

import Loading from "../loading";
import { BlogEntry } from "../../data/blog";

const Title = styled.div`
  color: black;
`;

const SubTitle = styled.div`
  color: black;
`;

const LogOutButton = styled.button``;

interface PanelProps {
  loading: boolean;
  blog_entries: Array<BlogEntry>;
}

const Panel = (p: PanelProps) => {
  const { path } = useRouteMatch();

  const { set_auth_data } = useContext(auth_context);
  const on_log_out = () => {
    set_auth_data!(undefined);
  };

  return (
    <div>
      <Title>Admin Panel</Title>
      <LogOutButton onClick={on_log_out}>Log Out</LogOutButton>
      <SubTitle>Blogs</SubTitle>
      {p.loading ? (
        <Loading />
      ) : (
        <BlogSummaryList blog_entries={p.blog_entries} prefix={path} />
      )}
    </div>
  );
};

export default Panel;
