import React, { useState, useContext } from "react";
import { Redirect } from "react-router-dom";

import { auth_context } from "../../contexts/auth_context";
import delete_post from "../../helpers/delete_post";

import styled from "styled-components";

const DivDeleteMsg = styled.div`
  color: black;
`;

const DeletePost = ({ blog_name }: { blog_name: string }) => {
  const [redirect_path, set_redirect] = useState<null | string>(null);

  const { auth } = useContext(auth_context);

  if (!redirect_path) {
    return (
      <div>
        <DivDeleteMsg>
          Are you sure you want to delete <code>{blog_name}</code>?
        </DivDeleteMsg>
        <button
          onClick={() => {
            set_redirect(`/admin/blog/${blog_name}`);
          }}
        >
          NO!
        </button>
        <button
          onClick={async () => {
            if (auth.key) {
              await delete_post(auth.key, blog_name);
              set_redirect("/admin/");
            }
          }}
        >
          YES!
        </button>
      </div>
    );
  } else {
    return <Redirect to={redirect_path} />;
  }
};

export default DeletePost;
