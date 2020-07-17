
import React, { useRef, useState, useLayoutEffect, useEffect } from "react";
import { useTransition, animated, useSpring } from "react-spring";

import "../../css/projects.css";

enum Tag {
    // Languages
    Typescript,
    Rust,
    Python,
    
    // Libraries/frameworks
    React,
    ML,
}

interface Project {
    display_name: string,
    rank: number,
    link: string,
    tags: Array<Tag>
    description: string,
    visible: boolean
}

const project_list: Array<Project> = [
    {
        display_name: "fluo-lang",
        rank: 0,
        link: "https://github.com/fluo-lang/fluoc",
        tags: [Tag.Rust],
        description: "a compiled programming language",
        visible: true
    },

    {
        display_name: "adai",
        rank: 1,
        link: "https://github.com/MonliH/adai",
        tags: [Tag.Python, Tag.ML],
        description: "alzheimer's disease ai",
        visible: true
    },

    {
        display_name: "emu-rs",
        rank: 2,
        link: "https://github.com/MonliH/emurs",
        tags: [Tag.Rust],
        description: "an emulator for the 8080 microprocessor",
        visible: true
    },

    {
        display_name: "audioify",
        rank: 3,
        link: "https://github.com/MonliH/audioify",
        tags: [Tag.Typescript, Tag.React],
        description: "generate music from code",
        visible: true
    },

    {
        display_name: "iNNteractive",
        rank: 4,
        link: "https://github.com/MonliH/iNNteractive",
        tags: [Tag.Python, Tag.ML],
        description: "interactive neural networks behind a GUI",
        visible: true
    },

    {
        display_name: "four-ai",
        rank: 5,
        link: "https://github.com/MonliH/four-ai",
        tags: [Tag.Rust],
        description: "neural networks trained with genetic algorithm that play connect four",
        visible: true
    },

    {
        display_name: "personal-website",
        rank: 6,
        link: "https://github.com/MonliH/personal-website",
        tags: [Tag.Typescript, Tag.React],
        description: "my personal website (you don't say...)",
        visible: true
    },

];

function useWindowSize() {
    const [size, setSize] = useState([0, 0]);
    useLayoutEffect(() => {
        function updateSize() {
          setSize([window.innerWidth, window.innerHeight]);
        }
        window.addEventListener('resize', updateSize);
        updateSize();
        return () => window.removeEventListener('resize', updateSize);
    }, []);
    return size;
}

interface ProjectGridProps {
    items: Array<Project>
}

interface ProjectPosition {
    xy: [number, number],
    project: Project,
}

const ProjectGrid = (p: ProjectGridProps) => {
    const [vwidth] = useWindowSize();
    const width = vwidth*0.7;
    
    // Card width
    const cardw = 300;
    // Card height
    const cardh = 350;


    // Card width margin
    const cardwm = 20;
    // Card height margin
    const cardhm = 20;

    // Number of comlumns
    const approx_cols = Math.floor(width/cardw);
    const columns = Math.floor((width - cardwm * approx_cols)/cardw); // Account for margin after approximate cols

    let counter_col = 0;
    let counter_row = 0;
    let grid_items: Array<ProjectPosition> = p.items.map((child, i) => {
        const col = counter_col;
        const row = counter_row;

        if (child.visible) {
            counter_col++;
        }

        if ((i % columns === columns-1) && child.visible) {
            counter_row++;
            counter_col = 0;
        }

        return {
            xy: [(cardw + cardwm) * col, row * (cardh + cardhm)],
            project: child
        };
    });

    const transitions = useTransition(grid_items, {
        from: () => ({ xy: [0, 0], width: cardw, height: cardh, opacity: 0 }),
        enter: ({ xy }) => ({ xy, width: cardw, height: cardh, opacity: 1 }),
        update: ({ xy }) => ({ xy, width: cardw, height: cardh, }),
        leave: { height: 0, opacity: 0 },
        config: { mass: 5, tension: 500, friction: 100 },
        keys: (item: ProjectPosition) => item.project.rank
    });

    const fragment = transitions((style: any, item: ProjectPosition) => {
        let {xy, ...others} = style;
        return (
            <animated.div key={item.project.rank} style={{ position: "absolute", transform: xy.interpolate((x: number, y: number) => `translate3d(${x}px, ${y}px, 0px)`), ...others}}>
                <div className="project-card" style={{ width: cardw, height: cardh }}>
                    <div className="project-texts">
                        <div className="project-title">{item.project.display_name}</div>
                        <div className="project-text">{item.project.description}</div>
                    </div>
                </div>
            </animated.div>
        );
    });

    const anims = useSpring({height: (counter_row + 1) * (cardh + cardhm), width: columns * (cardw + cardwm)});

    return (
        <animated.div id="project-grid" style={anims}>
            {fragment}
        </animated.div>
    );
}

const Projects = () => {
    const [items, itemsSet] = useState(project_list);

    return (
        <div id="projects">
            <ProjectGrid items={items}></ProjectGrid>
        </div>
    )
}

export default Projects;
