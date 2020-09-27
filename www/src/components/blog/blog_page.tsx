import React, {useState, useEffect} from "react";
import { Switch, Route, useLocation } from "react-router-dom";

import {BlogEntry} from "../../data/blog";
import NoMatch from "../404";

import styled from "styled-components";

const BlogMenuPageWrapper = styled.div`
display: flew;
flex-direction: row;
`;

const BlogPageWrapper = styled.div`
background-color: #ebeaed;
min-height: 100vh;
display: flex;
flex-direction: column;
align-items: center;
`;

const BlogTitle = styled.div`
font: 500 37px Montserrat, sans-serif;
color: #000000;
margin-bottom: 35px;
width: 800px;
`;

const BlogText = styled.div`
font: 19px Lato, sans-serif;
color: #000000;
width: 750px;
`;

const BlogContentWrapper = styled.div`
margin-left: -30vw;
margin-top: 100px;
`;

const SubBlogPage = ({blog, blog_entries}: {blog: BlogEntry, blog_entries: Array<BlogEntry>}) => {
  return (
    <BlogMenuPageWrapper>
      <BlogContentWrapper>
        <BlogTitle>{blog.title}</BlogTitle>
        <BlogText dangerouslySetInnerHTML={{__html: blog.contents}}></BlogText>
      </BlogContentWrapper>
    </BlogMenuPageWrapper>
  );
};

const BlogPage = ({blog_entries}: {blog_entries: Array<BlogEntry>}) => {
  const {pathname} = useLocation();
  const [ blog_404, set_blog_404 ] = useState(false);
  const [ blog, set_blog ] = useState<BlogEntry | null>({
    title: "Setting up a WASM + React + TypeScript project",
    contents: `
    <div>
    I love writing
    <a href="https://www.rust-lang.org/" rel="noopener noreferrer" target="_blank">
      Rust
    </a>
    . Recently, I've gotten interested in using Rust for web development, via
      web assembly (WASM). It turns out setting up WASM with React and
    TypeScript is harder than it looks&mdash;that's why I'm writing this post.
      </div>
    `,
    date: new Date("2020-09-25"),
    url: "wasm-react-ts"
  });

  /*
     const get_blog = async () => {
     const end_blog_path = pathname.split("/").pop();
     fetch(`/api/blog/entry/${end_blog_path}`)
     .then((res) => {
     if (!res.ok) {
     throw new Error();
     }
     return res.text();
     })
     .then((text) => set_blog(yaml.load(text)))
     .catch(() => set_blog_404(true));
     };
     useEffect(() => {get_blog();}, []);
   */

  return (
    blog_404? <NoMatch/> : (blog? <BlogPageWrapper><SubBlogPage blog={blog} blog_entries={blog_entries}/></BlogPageWrapper>: <div/>)
  );
}

export default BlogPage;

