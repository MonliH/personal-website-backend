import React from "react";

interface ChangerProps {
  current_page: number;
  set_page: (n: number) => void;
  total_pages: number;
}

const BlogPageChanger = (props: ChangerProps) => {
  return (
    <div>
      {[...Array(props.total_pages).keys()].map((i: number) => {
        return (
          <button
            onClick={() => {
              props.set_page(i);
            }}
            key={i}
          >
            {i}
          </button>
        );
      })}
    </div>
  );
};

export default BlogPageChanger;
