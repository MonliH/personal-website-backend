import React from "react";
import { Link } from "react-router-dom";
import styled from "styled-components";

import { shared_title } from "../title";
import { BlogEntry } from "../../data/blog/entries";

const BlogHomeWrapper = styled.div`
  background-color: #1d1d1d;
`;

const BlogTitleWrapper = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
`;

const Title = styled.pre`
  ${shared_title}
  height: 57px;
  font: bold 45px Montserrat, sans-serif;
  background-position: left 19px top 34px;
  margin-left: -9px;
  margin-bottom: 50px;
`;

const ContentPreview = styled.div`
  max-height: 100px;
  width: 600px;
  overflow: hidden;
`;

const BlogSummaryStyled = styled.div`
  padding-top: 15px;
  padding-bottom: 15px;
  border-top: solid;
  border-width: 1px;
  border-color: #262626;
  width: 700px;
  font: 15px Lato, sans-serif;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
`;

const BlogTitle = styled(Link)`
  font: 25px Montserrat, sans-serif;
  margin-bottom: 15px;
`;

const BlogSummary = ({ blog_entry }: { blog_entry: BlogEntry }) => {
  return (
    <BlogSummaryStyled>
      <BlogTitle to={`blog/${blog_entry.url}`}>{blog_entry.title}</BlogTitle>
      <ContentPreview>{blog_entry.contents}</ContentPreview>
    </BlogSummaryStyled>
  );
};

const BlogHome = ({blog_entries}: {blog_entries: Array<BlogEntry>}) => {
  const blog_previews = (
    <div>
      {blog_entries.map((blog_entry: BlogEntry, idx: number) => {
        return <BlogSummary blog_entry={blog_entry} key={idx}/>;
      })}
    </div>
  );

  return (
    <BlogHomeWrapper>
      <BlogTitleWrapper>
        <Title>Jonathan Li's Blog</Title>
        {blog_previews}
      </BlogTitleWrapper>
    </BlogHomeWrapper>
  );
};

export default BlogHome;
