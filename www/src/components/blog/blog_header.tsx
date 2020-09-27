import React from "react";

import styled from "styled-components";

import AnimatedLink from "../styled_link";

const Header = styled.div`
  color: black;
  font: 400 19px "IBM Plex Mono", monospace;
`;

const BlogHeader = () => {
  return (
    <Header>
      <AnimatedLink
        link="/"
        text="Jonathan Li"
        style={{
          fontSize: "19px",
          fontWeight: 400,
          fontFamily: '"IBM Plex Mono", monospace',
        }}
      />
      's{" "}
      <AnimatedLink
        link="/blog"
        text="blog"
        style={{
          fontSize: "19px",
          fontWeight: 400,
          fontFamily: '"IBM Plex Mono", monospace',
        }}
      />
    </Header>
  );
};

export default BlogHeader;
