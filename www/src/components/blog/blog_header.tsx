import React from "react";

import styled from "styled-components";

import AnimatedLink from "../styled_link";

const Header = styled.div`
  color: black;
  font: 400 45px "IBM Plex Mono", monospace;
  width: 700px;

  @media (max-width: 825px) {
    width: 95vw;
  }
`;

const StyledAnimatedLink = styled(AnimatedLink)`
  font: inherit;
`;

const BlogHeader = ({ blog }: { blog?: boolean }) => {
  return (
    <Header>
      <StyledAnimatedLink link="/" text="Jonathan Li" />
      's {blog ? <StyledAnimatedLink link="/blog" text="blog" /> : <>blog</>}
    </Header>
  );
};

export default BlogHeader;
