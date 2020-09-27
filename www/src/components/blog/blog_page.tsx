import React, {useState, useEffect} from "react";
import { Switch, Route, useLocation } from "react-router-dom";

import {BlogEntry} from "../../data/blog";
import NoMatch from "../404";

import yaml from "js-yaml";

const SubBlogPage = ({blog}: {blog: BlogEntry}) => {
  return (
    <div>
      <div>{blog.title}</div>
      <div dangerouslySetInnerHTML={{__html: blog.contents}}></div>
    </div>
  );
};

const BlogPage = () => {
  const {pathname} = useLocation();
  const [ blog_404, set_blog_404 ] = useState(false);
  const [ blog, set_blog ] = useState<BlogEntry | null>(null);

  const get_blog = async () => {
    const end_blog_path = pathname.split("/").pop();
      fetch(`/api/blog/entry/${end_blog_path}`)
      .then((res) => {
        if (!res.ok) {
          throw new Error();
        }
        return res.text();
      })
      .then((text) => set_blog(yaml.load(text)))
      .catch(() => set_blog_404(true));
  };
  useEffect(() => {get_blog();}, []);

  return (
    blog_404? <NoMatch/> : (blog? <SubBlogPage blog={blog}/>: <div/>)
  );
}

export default BlogPage;

