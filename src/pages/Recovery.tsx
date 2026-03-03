import React, { useState } from "react";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "../components/ui/card";
import { Button } from "../components/ui/button";
import { Alert, AlertTitle, AlertDescription } from "../components/ui/alert";
import { Mail, CheckCircle2 } from "lucide-react";

export default function Recovery() {
  const [email, setEmail] = useState("");
  const [sent, setSent] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const validateEmail = (value: string) => /\S+@\S+\.\S+/.test(value);

  const handleSubmit = async (e?: React.FormEvent) => {
    e?.preventDefault();
    setError(null);
    if (!validateEmail(email)) {
      setError("Please enter a valid email address.");
      return;
    }
    setLoading(true);
    // Simulated request
    await new Promise((r) => setTimeout(r, 900));
    setLoading(false);
    setSent(true);
  };

  return (
    <div className="min-h-[70vh] flex items-center justify-center px-4">
      <Card
        style={{
          width: "min(920px, 96%)",
          borderRadius: 12,
          boxShadow: "0 8px 30px rgba(15, 23, 42, 0.08)",
          overflow: "hidden",
        }}
      >
        <div style={{ display: "flex", flexWrap: "wrap" }}>
          {/* Left brand / illustration */}
          <div
            style={{
              flex: "1 1 320px",
              minHeight: 240,
              padding: "28px",
              background:
                "linear-gradient(180deg, var(--color-equans-azure-blue) 0%, var(--color-equans-turquoise-20) 100%)",
              color: "var(--color-equans-white)",
              display: "flex",
              flexDirection: "column",
              justifyContent: "center",
              gap: 8,
            }}
          >
            <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
              <div
                style={{
                  width: 48,
                  height: 48,
                  borderRadius: 10,
                  background: "rgba(255,255,255,0.12)",
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                }}
              >
                <Mail size={22} />
              </div>
              <h2
                style={{
                  margin: 0,
                  fontSize: 20,
                  fontWeight: 700,
                  fontFamily: "Roboto, sans-serif",
                }}
              >
                Password recovery
              </h2>
            </div>
            <p
              style={{
                marginTop: 6,
                color: "rgba(255,255,255,0.92)",
                fontFamily: "Roboto, sans-serif",
                lineHeight: 1.45,
                maxWidth: 420,
              }}
            >
              Enter your account email and we’ll send a secure link to reset
              your password. The link will expire for safety.
            </p>
          </div>

          {/* Right form */}
          <div
            style={{
              flex: "1 1 360px",
              padding: 28,
              background: "var(--color-equans-white)",
            }}
          >
            <CardHeader style={{ padding: 0, marginBottom: 8 }}>
              <CardTitle
                className="text-lg"
                style={{
                  color: "var(--color-equans-dark-blue)",
                  fontFamily: "Roboto, sans-serif",
                  marginBottom: 6,
                }}
              >
                Recover your account
              </CardTitle>
              <CardDescription
                style={{
                  color: "var(--color-equans-dark-blue-60)",
                  fontFamily: "Roboto, sans-serif",
                  marginBottom: 10,
                }}
              >
                Provide the email associated with your account.
              </CardDescription>
            </CardHeader>

            {sent ? (
              <Alert
                style={{
                  borderRadius: 8,
                  background: "var(--color-equans-turquoise-20)",
                  borderColor: "var(--color-equans-dark-green)",
                }}
              >
                <div
                  style={{ display: "flex", gap: 10, alignItems: "flex-start" }}
                >
                  <CheckCircle2
                    size={18}
                    style={{ color: "var(--color-equans-dark-green)" }}
                  />
                  <div>
                    <AlertTitle
                      style={{
                        color: "var(--color-equans-dark-blue)",
                        fontFamily: "Roboto, sans-serif",
                      }}
                    >
                      Email sent
                    </AlertTitle>
                    <AlertDescription
                      style={{
                        color: "var(--color-equans-dark-blue-60)",
                        fontFamily: "Roboto, sans-serif",
                      }}
                    >
                      If an account exists for <strong>{email}</strong>, you’ll
                      receive a recovery link shortly.
                    </AlertDescription>
                  </div>
                </div>
              </Alert>
            ) : (
              <form onSubmit={handleSubmit} style={{ marginTop: 6 }}>
                <label style={{ display: "block", marginBottom: 8 }}>
                  <span
                    style={{
                      display: "block",
                      marginBottom: 6,
                      color: "var(--color-equans-dark-blue)",
                      fontFamily: "Roboto, sans-serif",
                      fontWeight: 500,
                    }}
                  >
                    Email address
                  </span>
                  <input
                    type="email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    placeholder="name@company.com"
                    className="w-full rounded-md px-3 py-2 border"
                    style={{
                      borderColor: "var(--color-equans-dark-blue-20)",
                      fontFamily: "Roboto, sans-serif",
                      outline: "none",
                    }}
                    aria-label="Email address"
                  />
                </label>

                {error && (
                  <p
                    style={{
                      color: "var(--color-equans-orange)",
                      marginTop: 6,
                    }}
                  >
                    {error}
                  </p>
                )}

                <div
                  style={{
                    display: "flex",
                    gap: 10,
                    justifyContent: "flex-end",
                    marginTop: 16,
                  }}
                >
                  <Button
                    variant="outline"
                    onClick={() => {
                      setEmail("");
                      setError(null);
                    }}
                    style={{
                      borderColor: "var(--color-equans-dark-blue-60)",
                      color: "var(--color-equans-dark-blue)",
                      fontFamily: "Roboto, sans-serif",
                    }}
                  >
                    Cancel
                  </Button>
                  <Button
                    onClick={() => handleSubmit()}
                    disabled={loading}
                    style={{
                      backgroundColor: "var(--color-equans-dark-green)",
                      color: "var(--color-equans-white)",
                      fontWeight: 600,
                      fontFamily: "Roboto, sans-serif",
                      paddingLeft: 18,
                      paddingRight: 18,
                    }}
                  >
                    {loading ? "Sending..." : "Send recovery link"}
                  </Button>
                </div>
              </form>
            )}
            <div style={{ marginTop: 18, textAlign: "center" }}>
              <small
                style={{
                  color: "var(--color-equans-dark-blue-60)",
                  fontFamily: "Roboto, sans-serif",
                }}
              >
                Need help? Contact support at{" "}
                <a
                  href="mailto:support@equans.com"
                  style={{ color: "var(--color-equans-azure-blue)" }}
                >
                  support@equans.com
                </a>
              </small>
            </div>
          </div>
        </div>
      </Card>
    </div>
  );
}
