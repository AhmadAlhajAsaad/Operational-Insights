import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { 
  AccountInfo, 
  InteractionStatus
} from '@azure/msal-browser';
import { useMsal, useIsAuthenticated } from '@azure/msal-react';
import { loginRequest, graphConfig } from '../config/msalConfig';

interface AuthContextType {
  isAuthenticated: boolean;
  user: AccountInfo | null;
  userInfo: any | null;
  login: () => Promise<void>;
  logout: () => Promise<void>;
  isLoading: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

export function AuthProvider({ children }: AuthProviderProps) {
  //  MOCK AUTHENTICATION - Voor prototype doeleinden
  // Dit simuleert een ingelogde gebruiker zonder echte Microsoft SSO
  const [mockAuthenticated, setMockAuthenticated] = useState(false);
  const [mockUser] = useState({
    name: 'Demo User',
    username: 'demo.user@equans.com',
    homeAccountId: 'mock-account-id',
    environment: 'mock',
    tenantId: 'mock-tenant',
    localAccountId: 'mock-local-id',
  });

  const login = async () => {
    // Simuleer een korte loading tijd voor realistisch effect
    await new Promise(resolve => setTimeout(resolve, 500));
    setMockAuthenticated(true);
  };

  const logout = async () => {
    setMockAuthenticated(false);
  };

  const value: AuthContextType = {
    isAuthenticated: mockAuthenticated,
    user: mockAuthenticated ? mockUser as any : null,
    userInfo: mockAuthenticated ? {
      displayName: 'Demo User',
      mail: 'demo.user@equans.com',
      jobTitle: 'System Administrator',
    } : null,
    login,
    logout,
    isLoading: false,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}
