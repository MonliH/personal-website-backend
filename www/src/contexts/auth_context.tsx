import React, { createContext, useState } from "react";

interface AuthContext {
  auth?: string;
  set_auth_data?: (v?: string) => void;
}

export const auth_context = createContext<AuthContext>({});

const AuthProvider = ({ children }: { children: React.ReactNode }) => {
  const [auth, set_auth] = useState<undefined | string>(undefined);

  const set_auth_data = (token?: string) => {
    set_auth(token);
  };

  return (
    <auth_context.Provider value={{ auth, set_auth_data }}>
      {children}
    </auth_context.Provider>
  );
};

export default AuthProvider;
