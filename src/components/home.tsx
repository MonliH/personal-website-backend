import React from "react";
import Header from "./header";

import "../css/text.css";
import "../css/pages.css";

import TitleImage from "../img/title.png";
import Contact from "./home/contact";
import About from "./home/about";
import Projects from "./home/projects";

const Home = () => {
  const mappings: Array<[string, JSX.Element]> = [
    ["about", <About></About>],
    ["projects", <Projects></Projects>],
    ["contact", <Contact></Contact>],
  ];

  let other_pages = new Array(0);
  for (const [id_name, page] of mappings) {
    other_pages.push(
      <div className="page" id={id_name} key={id_name}>
        {page}
      </div>
    );
  }

  return (
    <div id="master">
      <Header />
      <div className="app">
        <div id="front-page">
          <div id="title-page">
            <div className="title-name-div">
              <div className="name-title">
                <div>JONATHAN </div>
              </div>
              <div className="name-title">
                <div>LI </div>
              </div>
              <div className="subheading" id="subheading_name">
                I Delight in Coding
              </div>
            </div>
          </div>
          <img src={TitleImage} id="title-image" alt=""></img>
        </div>
        <div id="bridge"></div>
        {other_pages}
      </div>
    </div>
  );
};

export default Home;
