import React, { useEffect } from "react";
import { useLocation } from "react-router-dom";

import { BlogEntry, BLOG_COLOR_BG } from "../../data/blog";
import BlogHeader from "./blog_header";
import NoMatch from "../404";
import useBlogPost from "../../hooks/useBlogPost";

import styled from "styled-components";

const BlogPageWrapper = styled.div`
  background-color: ${BLOG_COLOR_BG};
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
`;

export const BlogTitle = styled.div`
  font: 700 37px "IBM Plex Mono", monospace;
  color: #000000;
  margin-bottom: 35px;
  margin-top: 35px;
  width: 800px;

  @media (max-width: 850px) {
    width: 90vw;
  }
`;

const BlogText = styled.div`
  font: 16px Lato, sans-serif;
  color: #191919;
  width: 750px;
  @media (max-width: 850px) {
    width: 85vw;
  }
`;

const BlogContentWrapper = styled.div`
  margin-left: -20vw;
  margin-top: 100px;
  @media (max-width: 1225px) {
    margin-left: 0;
    padding-left: 20px;
  }
`;

const SubBlogPage = ({ blog }: { blog: BlogEntry }) => {
  return (
    <BlogContentWrapper>
      <BlogHeader blog />
      <BlogTitle>{blog.title}</BlogTitle>
      <BlogText dangerouslySetInnerHTML={{ __html: blog.contents }}></BlogText>
    </BlogContentWrapper>
  );
};

const BlogPage = () => {
  const { pathname } = useLocation();
  const [blog, blog_404] = useBlogPost(pathname.split("/").pop() as string);

  useEffect(() => {
    document.title = `Jonathan's blog${
      blog ? ` - ${blog.url.split("-").join(" ")}` : ""
    }`;
  }, [blog]);

  return blog_404 ? (
    <NoMatch msg={blog_404} />
  ) : blog ? (
    <BlogPageWrapper>
      <SubBlogPage blog={blog} />
    </BlogPageWrapper>
  ) : (
    <div>Loading...</div>
  );
};

export default BlogPage;
