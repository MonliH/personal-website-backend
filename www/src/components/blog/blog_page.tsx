import React, { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";

import { BlogEntry, BLOG_COLOR_BG, into_blog_entry } from "../../data/blog";
import BlogHeader from "./blog_header";
import NoMatch from "../404";

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
  font: 19px Lato, sans-serif;
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
      <BlogHeader />
      <BlogTitle>{blog.title}</BlogTitle>
      <BlogText dangerouslySetInnerHTML={{ __html: blog.contents }}></BlogText>
    </BlogContentWrapper>
  );
};

const BlogPage = () => {
  const { pathname } = useLocation();
  const [blog_404, set_blog_404] = useState<null | string>(null);
  const [blog, set_blog] = useState<BlogEntry | null>(null);

  const get_blog = async () => {
    const end_blog_path = pathname.split("/").pop();
    fetch(`/api/blog/entry/${end_blog_path}`)
      .then((res) => {
        if (!res.ok) {
          return Promise.reject(new Error("cannot find specified blog page"));
        }
        return res.text();
      })
      .then((text) => set_blog(into_blog_entry(JSON.parse(text))))
      .catch((why: Error) => {set_blog_404(why.message);});
  };
  useEffect(() => {
    get_blog();
  }, []);

  useEffect(() => {
    document.title = `Jonathan's blog${
      blog ? ` - ${blog.url.split("-").join(" ")}` : ""
    }`;
  }, [blog]);

  return blog_404 ? (
    <NoMatch msg={blog_404}/>
  ) : (blog ? (
    <BlogPageWrapper>
      <SubBlogPage blog={blog} />
    </BlogPageWrapper>
  ) : (
    <div />
  ));
};

export default BlogPage;
