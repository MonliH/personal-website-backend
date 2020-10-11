import React, { useState, useContext } from "react";
import { Redirect, useRouteMatch } from "react-router-dom";

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

const PanelButton = styled.button``;

interface PanelProps {
  loading: boolean;
  blog_entries: Array<BlogEntry>;
}

const Panel = (p: PanelProps) => {
  const { path } = useRouteMatch();

  const { set_auth_data } = useContext(auth_context);
  const [new_post, set_new_post] = useState(false);

  const on_log_out = () => {
    set_auth_data!(undefined);
  };

  const new_blog = () => {
    set_new_post(true);
  };

  return new_post ? (
    <Redirect to="/admin/new" />
  ) : (
    <div>
      <Title>Admin Panel</Title>
      <PanelButton onClick={on_log_out}>Log Out</PanelButton>
      <SubTitle>Blogs</SubTitle>
      <PanelButton onClick={new_blog}>New Post</PanelButton>
      {p.loading ? (
        <Loading />
      ) : (
        <BlogSummaryList blog_entries={p.blog_entries} prefix={path} />
      )}
    </div>
  );
};

export default Panel;
