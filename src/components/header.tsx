import React from "react";
import "../css/header.css";

import { NavHashLink } from "react-router-hash-link";

import GithubImage from "../img/github-white.png";

const links = [
    ["About Me", "#about"],
    ["My Projects", "#projects"],
    //["Blog", "blog"],
    ["Contact Me", "#contact"],
]

const Header = () => {
    let links_left = new Array(links.length);
    for (const [display, hash] of links) {
        links_left.push(<NavHashLink to={`/${hash}`} className="header-link" key={hash}>{display}</NavHashLink>)
    }

    return (
        <div className="header">
            <NavHashLink to="#master" className="header-name header-link">Jonathan Li</NavHashLink>
            <div className="header-links">
                {links_left}

                { /* Github logo */ }
                <a href="https://github.com/MonliH" target="_blank" rel="noopener noreferrer" className="header-link">
                    <img src={GithubImage} id="header-image" alt="Github"></img>
                </a>
            </div>
        </div>
    );
}

export default Header;
