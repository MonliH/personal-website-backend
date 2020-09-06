import React, { useRef } from "react";

import * as Video from "../../helpers/video";

import AboutMeWEBM from "../../videos/about_me.webm";
import AboutMeMP4 from "../../videos/about_me.mp4";

const About = () => {
  let video_element = useRef(null);
  return (
    <div className="wrapper-center">
      <div id="about" className="wrapper-inner">
        <pre className="title" style={{ marginLeft: "50px" }}>
          About Me
        </pre>
        <div id="about-me-section">
          <video
            onMouseEnter={() => Video.enter(video_element)}
            onMouseLeave={() => Video.leave(video_element)}
            id="about-vid"
            ref={video_element}
            playsInline
            muted
            loop
          >
            <source src={AboutMeWEBM} type="video/webm" />
            <source src={AboutMeMP4} type="video/mp4" />
            Your browser does not support the video element.
          </video>
          <div id="about-text">
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
          </div>
        </div>
      </div>
    </div>
  );
};

export default About;
