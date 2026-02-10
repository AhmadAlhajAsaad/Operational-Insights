import React from "react";
import { Card } from "../components/ui/card";
import { Button } from "../components/ui/Button";
import { User, Mail, Building2, Calendar, ArrowLeft } from "lucide-react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Legend,
} from "recharts";
import { users, userActivityData } from "../data/mockData";

interface UserDetailProps {
  userId: number;
  onBack: () => void;
}

export function UserDetail({ userId, onBack }: UserDetailProps) {
  const user = users.find((u) => u.id === userId) || users[0];

  return (
    <div className="space-y-6">
      {/* Colored Header with Breadcrumb */}
      <div
        className="px-6 py-4 rounded-full border-t  flex flex-col md:flex-row md:items-center gap-2 mb-2 text-white shadow-sm"
        style={{ backgroundColor: "var( --color-equans-turquoise)" }}
      >
        <div className="font-semibold text-2xl">User Detail</div>
        <div className="flex items-center text-sm ml-2">
          <span style={{ color: "var(--color-ui-blue-lighter)" }}>Home</span>
          <span
            className="mx-1"
            style={{ color: "var(--color-ui-blue-lighter)" }}
          >
            /
          </span>
          <span style={{ color: "var(--color-ui-blue-lighter)" }}>Users</span>
          <span
            className="mx-1"
            style={{ color: "var(--color-ui-blue-lighter)" }}
          >
            /
          </span>
          <span className="text-white font-medium">Detail</span>
        </div>
      </div>

      <Button
        variant="ghost"
        onClick={onBack}
        className="hover:text-white"
        style={
          {
            "--hover-bg": "var(--color-ui-blue-lighter)",
            "--hover-color": "var(--color-ui-blue-primary)",
          } as React.CSSProperties
        }
        onMouseEnter={(e) => {
          e.currentTarget.style.backgroundColor =
            "var(--color-ui-blue-lighter)";
          e.currentTarget.style.color = "var(--color-ui-blue-primary)";
        }}
        onMouseLeave={(e) => {
          e.currentTarget.style.backgroundColor = "transparent";
          e.currentTarget.style.color = "inherit";
        }}
      >
        <ArrowLeft className="w-4 h-4 mr-2" />
        Back to Dashboard
      </Button>

      <Card
        className="shadow-sm"
        style={{
          backgroundColor: "var(--color-ui-blue-lighter)",
          borderColor: "var(--color-ui-blue-light)",
          borderRadius: "16px",
        }}
      >
        <div className="p-6">
          <div className="flex items-start gap-6">
            <div
              className="w-20 h-20 text-white rounded-full flex items-center justify-center flex-shrink-0"
              style={{ backgroundColor: "var(--color-ui-blue-primary)" }}
            >
              <User className="w-10 h-10" />
            </div>
            <div className="flex-1">
              <h2 className="text-2xl font-bold text-neutral-900 mb-2">
                {user.name}
              </h2>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div className="flex items-center gap-2 text-neutral-700">
                  <Mail
                    className="w-4 h-4"
                    style={{ color: "var(--color-ui-blue-primary)" }}
                  />
                  <span className="text-sm">{user.email}</span>
                </div>
                <div className="flex items-center gap-2 text-neutral-700">
                  <Building2
                    className="w-4 h-4"
                    style={{ color: "var(--color-ui-blue-primary)" }}
                  />
                  <span className="text-sm">{user.department}</span>
                </div>
                <div className="flex items-center gap-2 text-neutral-700">
                  <Calendar
                    className="w-4 h-4"
                    style={{ color: "var(--color-ui-blue-primary)" }}
                  />
                  <span className="text-sm">
                    Last active: {user.lastActive}
                  </span>
                </div>
              </div>
              <div className="mt-3">
                <span
                  className={`inline-flex px-3 py-1.5 rounded-full text-sm font-semibold`}
                  style={{
                    backgroundColor:
                      user.status === "active"
                        ? "var(--color-ui-green-light)"
                        : "var(--color-neutral-100)",
                    color:
                      user.status === "active"
                        ? "var(--color-ui-green)"
                        : "var(--color-neutral-600)",
                  }}
                >
                  {user.status === "active" ? "Active User" : "Inactive User"}
                </span>
              </div>
            </div>
            <div className="flex gap-2">
              <Button
                variant="secondary"
                size="sm"
                style={{
                  borderColor: "var(--color-ui-blue-primary)",
                  color: "var(--color-ui-blue-primary)",
                }}
              >
                Edit User
              </Button>
              <Button
                variant="primary"
                size="sm"
                style={{
                  backgroundColor: "var(--color-ui-blue-primary)",
                  color: "white",
                }}
              >
                Manage Licenses
              </Button>
            </div>
          </div>
        </div>
      </Card>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card
          className="shadow-sm border"
          style={{
            backgroundColor: "var(--color-ui-blue-light)",
            borderColor: "var(--color-ui-blue-primary)",
            borderRadius: "16px",
          }}
        >
          <div className="p-6">
            <h3
              className="text-lg font-bold mb-4"
              style={{ color: "var(--color-ui-blue-primary)" }}
            >
              Assigned Licenses
            </h3>
            <div className="space-y-3">
              {user.licenses.map((license, index) => (
                <div
                  key={index}
                  className="flex items-center justify-between p-3 bg-white rounded-xl transition-colors"
                  onMouseEnter={(e) =>
                    (e.currentTarget.style.backgroundColor =
                      "var(--color-ui-blue-lighter)")
                  }
                  onMouseLeave={(e) =>
                    (e.currentTarget.style.backgroundColor = "white")
                  }
                >
                  <div className="flex items-center gap-3">
                    <div
                      className="w-10 h-10 text-white rounded-lg flex items-center justify-center"
                      style={{
                        backgroundColor: "var(--color-ui-blue-primary)",
                      }}
                    >
                      <span className="text-sm font-bold">{license[0]}</span>
                    </div>
                    <div>
                      <p className="font-medium text-neutral-900">{license}</p>
                      <p className="text-xs text-neutral-500">
                        Active since Jan 2024
                      </p>
                    </div>
                  </div>
                  <Button
                    variant="ghost"
                    size="sm"
                    style={{ color: "var(--color-ui-red)" }}
                  >
                    Remove
                  </Button>
                </div>
              ))}
            </div>
            <Button
              variant="secondary"
              className="w-full mt-4"
              style={{
                borderColor: "var(--color-ui-blue-primary)",
                color: "var(--color-ui-blue-primary)",
              }}
            >
              Add New License
            </Button>
          </div>
        </Card>

        <Card
          className="shadow-sm border"
          style={{
            backgroundColor: "var(--color-ui-green-lighter)",
            borderColor: "var(--color-ui-green)",
            borderRadius: "16px",
          }}
        >
          <div className="p-6">
            <h3
              className="text-lg font-bold mb-4"
              style={{ color: "var(--color-ui-green)" }}
            >
              License Summary
            </h3>
            <div className="space-y-4">
              <div
                className="flex justify-between items-center p-4 rounded-xl"
                style={{ backgroundColor: "var(--color-ui-blue-light)" }}
              >
                <div>
                  <p
                    className="text-sm font-medium"
                    style={{ color: "var(--color-ui-blue-primary)" }}
                  >
                    Total Licenses
                  </p>
                  <h2 className="text-3xl font-bold text-neutral-900">
                    {user.licenses.length}
                  </h2>
                </div>
                <div
                  className="w-12 h-12 text-white rounded-full flex items-center justify-center"
                  style={{ backgroundColor: "var(--color-ui-blue-primary)" }}
                >
                  <span className="text-xl font-bold">
                    {user.licenses.length}
                  </span>
                </div>
              </div>
              <div
                className="p-4 border rounded-xl bg-white"
                style={{ borderColor: "var(--color-ui-blue-light)" }}
              >
                <p className="text-sm text-neutral-600 mb-2">Monthly Cost</p>
                <h3 className="text-xl font-bold text-neutral-900">{`€${(user.licenses.length * 85).toFixed(2)}`}</h3>
                <p className="text-xs text-neutral-500 mt-1">
                  Average per license: €85.00
                </p>
              </div>
              <div
                className="p-4 border rounded-xl"
                style={{
                  borderColor: "var(--color-ui-green-light)",
                  backgroundColor: "var(--color-ui-green-light)",
                }}
              >
                <p className="text-sm text-neutral-600 mb-2">Account Status</p>
                <h4 className="text-lg font-bold text-neutral-900">
                  Full Access
                </h4>
                <p className="text-xs text-neutral-500 mt-1">
                  All licenses active and accessible
                </p>
              </div>
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}
