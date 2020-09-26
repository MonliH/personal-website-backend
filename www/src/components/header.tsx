import React from "react";

import styled from "styled-components";

import { HashLink } from "react-router-hash-link";
import GithubImage from "../img/github-white.png";

const HeaderMain = styled.div`
  overflow: hidden;
  background-color: #1d1d1d;
  background-color: rgba(29, 29, 29, 0.5);
  border-bottom: 1px solid #171717;
  z-index: 100;
  white-space: nowrap;
  position: fixed;
  width: 100vw;
  height: 50px;
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
`;

const HeaderLinks = styled.div`
  font: 15px Montserrat, sans-serif;
  float: right;
  display: flex;
  flex: 0 0;
  margin-right: 30px;
  position: relative;
  top: 50%;
  transform: translateY(-50%);
`;

const HeaderLink = styled(HashLink)`
  font: 15px Montserrat, sans-serif;
  color: white;
  text-decoration: none;
  padding-left: 30px;
`;

const HeaderLinkGithub = styled.a`
  font: 15px Montserrat, sans-serif;
  color: white;
  text-decoration: none;
  padding-left: 30px;
`;

const HeaderName = styled(HashLink)`
  margin-left: 30px;
  font: bold 20px Montserrat, sans-serif;
  margin-bottom: 20px;
  float: left;
  color: white;
  text-decoration: none;
  position: relative;
  top: 50%;
  transform: translateY(-50%);
`;

const HeaderImage = styled.img`
  margin-top: -2px;
  float: right;
  width: 27px;
  height: 27px;
`;

const links = [
  ["About Me", "#about"],
  ["My Projects", "#projects"],
  //["Blog", "blog"],
  ["Contact Me", "#contact"],
];

const Header = () => {
  let links_left = new Array(links.length);
  for (const [display, hash] of links) {
    links_left.push(
      <HeaderLink to={`/${hash}`} key={hash}>
        {display}
      </HeaderLink>
    );
  }

  return (
    <HeaderMain>
      <HeaderName to="/#master">Jonathan Li</HeaderName>
      <HeaderLinks>
        {links_left}

        {/* Github logo */}
        <HeaderLinkGithub
          href="https://github.com/MonliH"
          target="_blank"
          rel="noopener noreferrer"
          className="header-link"
        >
          <HeaderImage
            src={GithubImage}
            id="header-image"
            alt="Github"
          ></HeaderImage>
        </HeaderLinkGithub>
      </HeaderLinks>
    </HeaderMain>
  );
};

export default Header;
