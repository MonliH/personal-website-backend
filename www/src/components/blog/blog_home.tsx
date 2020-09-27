import React, {useState, useEffect} from "react";
import { Link } from "react-router-dom";
import styled from "styled-components";

import { shared_title } from "../title";
import { BlogEntry } from "../../data/blog";

import yaml from "js-yaml";

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

const BlogHome = () => {
  const [blog_entries, set_blog_entries] = useState<Array<BlogEntry>>([]);
  const [page_no, set_page_no] = useState(0);

  const posts_per_page = 10;

  const fetch_entries = async () => {
    const blog_pages_num = parseInt(await(await fetch("/api/blog/pages")).text());
    const page_start = posts_per_page*page_no;
    const possible_end = page_start + posts_per_page;
    const entries_res = 
      await fetch(`/api/blog/entries/${page_start}/${possible_end > blog_pages_num? blog_pages_num : possible_end}`);
    const entries: Array<BlogEntry> = yaml.loadAll(await entries_res.text());
    set_blog_entries(entries);
  };

  // eslint-disable-next-line react-hooks/exhaustive-deps
  useEffect(() => { fetch_entries(); }, [page_no]);

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
