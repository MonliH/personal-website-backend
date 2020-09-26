import React, { useRef } from "react";
import styled from "styled-components";

import * as Video from "../../helpers/video";
import { WrapperCenter, WrapperInner } from "../wrapper";
import { Title } from "../title";

import AboutMeWEBM from "../../videos/about_me.webm";
import AboutMeMP4 from "../../videos/about_me.mp4";

const AboutVideo = styled.video`
  display: block;
  -moz-user-select: none;
  -webkit-user-select: none;
  user-select: none;
  height: 50vh;
  width: 50vh;
  filter: brightness(1.1) contrast(105%) saturate(135%);
`;

const AboutText = styled.div`
  text-align: left;
  width: 370px;
  color: white;
  font: 400 18px Lato, sans-serif;
`;

const AboutMeSection = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: -70px;
`;

const AboutStyled = styled.div`
  padding-top: 20vh;
  margin-top: -24vh;
  margin-bottom: 10vh;
  background-color: #1d1d1d;
`;

const About = () => {
  let video_element = useRef(null);
  return (
    <AboutStyled>
      <WrapperCenter>
        <WrapperInner>
          <Title style={{ marginLeft: "50px" }}>About Me&thinsp;</Title>
          <AboutMeSection>
            <AboutVideo
              onMouseEnter={() => Video.enter(video_element)}
              onMouseLeave={() => Video.leave(video_element)}
              ref={video_element}
              playsInline
              muted
              loop
            >
              <source src={AboutMeWEBM} type="video/webm" />
              <source src={AboutMeMP4} type="video/mp4" />
              Your browser does not support the video element.
            </AboutVideo>
            <AboutText>
              Hey! My name is Jonathan Li.
              <br />
              <br />
              I’m passionate about coding, design, and making simulations.
              <br />
              <br />
              Some things I enjoy creating include deep learning models,
              programming languages, and web apps.
              <br />
              <br />I revel in learning new things.
            </AboutText>
          </AboutMeSection>
        </WrapperInner>
      </WrapperCenter>
    </AboutStyled>
  );
};

export default About;
