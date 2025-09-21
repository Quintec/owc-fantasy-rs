import React, { createContext, useContext, useEffect, useState } from "react";

export type User = {
  id: number;
  username: string;
  avatar_url?: string | null;
};

type AuthContextType = {
  user: User | null;
  loading: boolean;
  refresh: () => Promise<void>;
  logout: () => Promise<void>;
};

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState<boolean>(true);

  const refresh = async () => {
    setLoading(true);
    try {
      const res = await fetch("http://localhost:8080/api/auth/me", {
        credentials: "include",
      });
      console.log('refresh() http://localhost:8080/api/auth/me status', res.status)
      if (res.ok) {
        const data = await res.json();
        console.log('refresh() user data', data)
        setUser(data);
      } else {
        const text = await res.text().catch(() => '')
        console.log('refresh() not ok, body:', text)
        setUser(null);
      }
    } catch (e) {
      console.error('refresh() fetch error', e)
      setUser(null);
    } finally {
      setLoading(false);
    }
  };

  const logout = async () => {
    try {
      const res = await fetch("http://localhost:8080/api/auth/logout", {
        method: "POST",
        credentials: "include",
      });
      console.log('logout() status', res.status)
    } catch (e) {
      // ignore
    } finally {
      setUser(null);
    }
  };

  useEffect(() => {
    refresh();
  }, []);

  return (
    <AuthContext.Provider value={{ user, loading, refresh, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const ctx = useContext(AuthContext);
  if (!ctx) {
    throw new Error("useAuth must be used inside AuthProvider");
  }
  return ctx;
};