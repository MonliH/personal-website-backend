
import React from "react";

enum Tag {
    // Langauges
    Typescript,
    Rust,
    Python,
    Jupyter,
    
    // Libraries/frameworks
    React,
    Pytorch,
    Keras,

    // Platforms
    Desktop,
    Web
}

interface Project {
    display_name: string,
    rank: number,
    link: string,
    tags: Array<Tag>
    description: string
}

const project_list: Array<Project> = [
    {
        display_name: "fluo-lang",
        rank: 0,
        link: "https://github.com/fluo-lang/fluoc",
        tags: [Tag.Rust, Tag.Desktop],
        description: ""
    },

]

const projects = () => {
    return (
        <div>
        </div>  
    );
}

export default projects;

