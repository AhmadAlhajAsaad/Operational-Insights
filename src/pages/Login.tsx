import React from "react";
import { useAuth } from "../context/AuthContext";
import { Button } from "../components/ui/button";
import logo from "../assets/logo.png";

export function Login() {
  const { login, isLoading } = useAuth();

  const handleMicrosoftLogin = async () => {
    await login();
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary-50 via-white to-accent-50">
      <div className="w-full max-w-md">
        {/* Logo & Branding */}
        <div className="text-center mb-8">
          <div className="inline-flex items-center justify-center mb-4">
            <img src={logo} alt="Equans Logo" className="h-20 w-auto" />
          </div>
          <h1 className="text-3xl font-bold mb-2">Operational Insights</h1>
          <p className="text-neutral-600">Sign in with your Equans account</p>
        </div>

        {/* SSO Login Card */}
        <div className="bg-white rounded-2xl shadow-xl border border-neutral-200 p-8">
          <div className="space-y-6">
            {/* Microsoft SSO Button */}
            <Button
              onClick={handleMicrosoftLogin}
              disabled={isLoading}
              variant="primary"
              size="lg"
              className="w-full flex items-center justify-center gap-3"
            >
              {isLoading ? (
                <>
                  <div className="inline-block animate-spin rounded-full h-5 w-5 border-b-2 border-white"></div>
                  <span>Redirecting to Microsoft...</span>
                </>
              ) : (
                <>
                  <svg
                    className="w-5 h-5"
                    viewBox="0 0 23 23"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path d="M11 0H0V11H11V0Z" fill="#F25022" />
                    <path d="M23 0H12V11H23V0Z" fill="#7FBA00" />
                    <path d="M11 12H0V23H11V12Z" fill="#00A4EF" />
                    <path d="M23 12H12V23H23V12Z" fill="#FFB900" />
                  </svg>
                  Sign in with Microsoft
                </>
              )}
            </Button>

            {/* Info Text */}
            <div className="text-center space-y-2">
              <p className="text-sm text-neutral-600">
                Use your Equans Microsoft account to sign in
              </p>
              <p className="text-sm text-neutral-500">
                Secured by Microsoft Entra ID
              </p>
            </div>
          </div>

          <div className="mt-6 pt-6 border-t border-neutral-200">
            <p className="text-sm text-center text-neutral-600">
              Need access? Contact your administrator
            </p>
          </div>
        </div>

        {/* Footer */}
        <p className="text-sm text-center text-neutral-500 mt-8">
          © 2025 Equans. All rights reserved.
        </p>
      </div>
    </div>
  );
}
