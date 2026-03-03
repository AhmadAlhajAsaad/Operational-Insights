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
    // Simulate API request
    await new Promise((r) => setTimeout(r, 900));
    setLoading(false);
    setSent(true);
  };

  return (
    <div className="min-h-[80vh] flex items-center justify-center px-4 py-12">
      <Card
        style={{
          width: "min(980px, 96%)",
          borderRadius: 14,
          boxShadow: "0 12px 30px rgba(2,6,23,0.08)",
          overflow: "hidden",
        }}
      >
        <div style={{ display: "flex", flexWrap: "wrap" }}>
          {/* Left brand / illustration */}
          <div
            style={{
              flex: "1 1 360px",
              minHeight: 300,
              padding: 30,
              background:
                "linear-gradient(180deg, var(--color-equans-azure-blue) 0%, var(--color-equans-turquoise) 100%)",
              color: "var(--color-equans-white)",
              display: "flex",
              flexDirection: "column",
              justifyContent: "center",
              gap: 12,
            }}
          >
            <div style={{ display: "flex", alignItems: "center", gap: 14 }}>
              <div
                style={{
                  width: 56,
                  height: 56,
                  borderRadius: 12,
                  background: "rgba(255,255,255,0.12)",
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                }}
              >
                <Mail size={26} />
              </div>
              <div>
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
                <p
                  style={{
                    margin: 0,
                    fontSize: 13,
                    color: "rgba(255,255,255,0.92)",
                    fontFamily: "Roboto, sans-serif",
                    opacity: 0.95,
                  }}
                >
                  Securely reset your account password.
                </p>
              </div>
            </div>

            <p
              style={{
                marginTop: 8,
                color: "rgba(255,255,255,0.9)",
                fontFamily: "Roboto, sans-serif",
                lineHeight: 1.45,
                maxWidth: 440,
              }}
            >
              Enter the email address associated with your Equans account and
              we’ll send a secure reset link. The link expires for security
              reasons.
            </p>

            <div
              style={{
                marginTop: 10,
                opacity: 0.95,
                display: "flex",
                gap: 10,
                flexWrap: "wrap",
              }}
            >
              <div
                style={{
                  background: "rgba(255,255,255,0.06)",
                  padding: "8px 12px",
                  borderRadius: 8,
                  fontSize: 13,
                }}
              >
                Secure • Expiring link
              </div>
              <div
                style={{
                  background: "rgba(255,255,255,0.06)",
                  padding: "8px 12px",
                  borderRadius: 8,
                  fontSize: 13,
                }}
              >
                Enterprise support
              </div>
            </div>
          </div>

          {/* Right form */}
          <div
            style={{
              flex: "1 1 420px",
              padding: 30,
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
                  color: "var(--equans-blue-60)",
                  fontFamily: "Roboto, sans-serif",
                  marginBottom: 10,
                }}
              >
                Provide the email associated with your account and we’ll send a
                reset link.
              </CardDescription>
            </CardHeader>

            <CardContent style={{ padding: 0 }}>
              {sent ? (
                <Alert
                  style={{
                    borderRadius: 10,
                    background: "var(--equans-green-20, rgba(226,245,220,0.2))",
                    borderColor: "var(--color-equans-dark-green)",
                    padding: 14,
                  }}
                >
                  <div
                    style={{
                      display: "flex",
                      gap: 12,
                      alignItems: "flex-start",
                    }}
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
                          color: "var(--equans-blue-60)",
                          fontFamily: "Roboto, sans-serif",
                        }}
                      >
                        If an account exists for <strong>{email}</strong>,
                        you’ll receive a recovery link shortly.
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
                        marginBottom: 8,
                        color: "var(--color-equans-dark-blue)",
                        fontFamily: "Roboto, sans-serif",
                        fontWeight: 600,
                        fontSize: 13,
                      }}
                    >
                      Email address
                    </span>
                    <input
                      type="email"
                      value={email}
                      onChange={(e) => setEmail(e.target.value)}
                      placeholder="name@company.com"
                      className="recovery-input"
                      style={{
                        width: "100%",
                      }}
                      aria-label="Email address"
                    />
                  </label>

                  {error && (
                    <p
                      style={{
                        color: "var(--color-equans-orange)",
                        marginTop: 6,
                        fontSize: 13,
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
                      marginTop: 18,
                    }}
                  >
                    <Button
                      variant="outline"
                      onClick={() => {
                        setEmail("");
                        setError(null);
                      }}
                      style={{
                        borderColor: "var(--equans-blue-60)",
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
                        fontWeight: 700,
                        fontFamily: "Roboto, sans-serif",
                        paddingLeft: 20,
                        paddingRight: 20,
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
                    color: "var(--equans-blue-60)",
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
            </CardContent>
          </div>
        </div>
      </Card>
    </div>
  );
}
