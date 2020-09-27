import React, { useEffect } from "react";
import { Link } from "react-router-dom";
import styled from "styled-components";

import { shared_title } from "../title";
import { BlogEntry, BLOG_COLOR_BG } from "../../data/blog";
import AnimatedLink from "../styled_link";

import BlogHeader from "./blog_header";

const BlogHomeWrapper = styled.div`
  background-color: ${BLOG_COLOR_BG};
  min-height: 100vh;
`;

const BlogTitleWrapper = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
`;

export const Title = styled.pre`
  ${shared_title}
  height: 57px;
  font: bold 45px Montserrat, sans-serif;
  background-position: left 19px top 34px;
  margin-left: -9px;
  margin-bottom: 50px;
`;

const ContentPreview = styled.div`
  max-height: 100px;
  width: 700px;
  overflow: hidden;
  color: #191919;
  mask-image: linear-gradient(to bottom, black 0%, transparent 100%);
`;

const BlogSummaryStyled = styled.div`
  padding-top: 15px;
  padding-bottom: 15px;
  font: 15px Lato, sans-serif;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
`;

const BlogSummaryInner = styled.div``;
const BlogMainInner = styled.div`
  padding-top: 60px;
`;

const BlogHeaderWrapper = styled.div`
  margin-bottom: 30px;
`;

const BlogTitle = styled(AnimatedLink)`
  font: 600 25px "IBM Plex Mono", monospace;
  margin-bottom: 10px;
  width: 600px;
`;

const StyledBlogTime = styled.div`
  color: black;
  margin-bottom: 20px;
`;

const BlogTime = ({ date }: { date: Date }) => {
  return <StyledBlogTime>{date.toLocaleDateString("en-US")}</StyledBlogTime>;
};

const BlogSummary = ({ blog_entry }: { blog_entry: BlogEntry }) => {
  return (
    <BlogSummaryStyled>
      <BlogSummaryInner>
        <BlogTitle
          link={`/blog/${blog_entry.url}`}
          text={blog_entry.title}
          style={{
            marginBottom: "10px",
            width: "600px",
            fontWeight: 600,
            fontSize: "27px",
            fontFamily: '"IBM Plex Mono", monospace',
          }}
        />
        <BlogTime date={blog_entry.date}></BlogTime>
        <ContentPreview
          dangerouslySetInnerHTML={{ __html: blog_entry.contents }}
        ></ContentPreview>
      </BlogSummaryInner>
    </BlogSummaryStyled>
  );
};

const BlogHome = ({ blog_entries }: { blog_entries: Array<BlogEntry> }) => {
  const blog_previews = (
    <div>
      {blog_entries.map((blog_entry: BlogEntry, idx: number) => {
        return <BlogSummary blog_entry={blog_entry} key={idx} />;
      })}
    </div>
  );

  useEffect(() => {
    document.title = "Jonathan Li's blog";
  });

  return (
    <BlogHomeWrapper>
      <BlogTitleWrapper>
        <BlogMainInner>
          <BlogHeaderWrapper>
            <BlogHeader/>
          </BlogHeaderWrapper>
          {blog_previews}
        </BlogMainInner>
      </BlogTitleWrapper>
    </BlogHomeWrapper>
  );
};

export default BlogHome;
