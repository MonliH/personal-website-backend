import React from "react";

import styled from "styled-components";

import AnimatedLink from "../styled_link";

const Header = styled.div`
  color: black;
  font: 400 19px "IBM Plex Mono", monospace;

  @media (max-width: 825px) {
    width: 95vw;
  }
`;

const StyledAnimatedLink = styled(AnimatedLink)`
  font: 400 19px "IBM Plex Mono", monospace;
`;

const BlogHeader = ({blog}: {blog?: boolean}) => {
  return (
    <Header>
      <StyledAnimatedLink link="/" text="Jonathan Li" />
      's {blog? <StyledAnimatedLink link="/blog" text="blog" /> : <>blog</>}
    </Header>
  );
};

export default BlogHeader;
