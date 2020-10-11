import React from "react";

import useBlogPost from "../../hooks/useBlogPost";

import AdminBlogPage from "./admin_blog_page";
import Err from "../error";

const EditPost = ({ blog_path }: { blog_path: string }) => {
  const [blog, blog_404] = useBlogPost(blog_path);
  return blog_404 ? <Err /> : <AdminBlogPage blog={blog!} />;
};

export default EditPost;
