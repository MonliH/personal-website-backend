import React from "react";

import {BlogEntry} from "../../data/blog/entries";

const BlogPage = ({blog}: {blog: BlogEntry}) => {
  return <div>
    {blog.title}
    {blog.contents}
  </div>
}

export default BlogPage;

