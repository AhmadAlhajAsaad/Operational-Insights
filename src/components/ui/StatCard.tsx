import React from "react";
import { ArrowUp, ArrowDown } from "lucide-react";

interface StatCardProps {
  icon: React.ReactNode;
  label: string;
  value: string | number;
  change?: {
    value: string;
    trend: "up" | "down" | "neutral";
  };
  onClick?: () => void;
  variant?:
    | "default"
    | "blue"
    | "green"
    | "yellow"
    | "pink"
    | "purple"
    | "cyan"
    | "mint";
}

const variantStyles = {
  default: {
    bg: "var(--color-equans-white)",
    iconBg: "var(--equans-blue-20)",
    iconColor: "var(--color-equans-azure-blue)",
    border: "var(--equans-blue-20)",
  },
  blue: {
    bg: "var(--equans-blue-20)",
    iconBg: "var(--color-equans-azure-blue)",
    iconColor: "var(--color-equans-white)",
    border: "var(--equans-blue-20)",
  },
  green: {
    bg: "var(--equans-green-20)",
    iconBg: "var(--color-equans-dark-green)",
    iconColor: "var(--color-equans-white)",
    border: "var(--equans-green-20)",
  },
  yellow: {
    bg: "var(--equans-yellow-20)",
    iconBg: "var(--color-equans-yellow)",
    iconColor: "var(--color-equans-white)",
    border: "var(--equans-yellow-20)",
  },
  pink: {
    bg: "var(--equans-pink-20)",
    iconBg: "var(--color-equans-pink)",
    iconColor: "var(--color-equans-white)",
    border: "var(--equans-pink-20)",
  },
  purple: {
    bg: "var(--equans-violet-20)",
    iconBg: "var(--color-equans-violet)",
    iconColor: "var(--color-equans-white)",
    border: "var(--equans-violet-20)",
  },
  cyan: {
    bg: "var(--equans-lightblue-20)",
    iconBg: "var(--color-equans-light-blue)",
    iconColor: "var(--color-equans-white)",
    border: "var(--equans-lightblue-20)",
  },
  mint: {
    bg: "var(--equans-lime-20)",
    iconBg: "var(--color-equans-lime-green)",
    iconColor: "var(--color-equans-white)",
    border: "var(--equans-lime-20)",
  },
};

export function StatCard({
  icon,
  label,
  value,
  change,
  onClick,
  variant = "default",
}: StatCardProps) {
  const styles = variantStyles[variant];

  return (
    <div
      className={`p-6 transition-all shadow-sm ${onClick ? "cursor-pointer hover:shadow-lg hover:scale-[1.02]" : ""}`}
      style={{
        borderRadius: "16px",
        backgroundColor: styles.bg,
        border: `1px solid ${styles.border}`,
      }}
      onClick={onClick}
      role={onClick ? "button" : undefined}
      tabIndex={onClick ? 0 : undefined}
      onKeyDown={(e) => {
        if (onClick && (e.key === "Enter" || e.key === " ")) {
          e.preventDefault();
          onClick();
        }
      }}
    >
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div
            className="p-3 rounded-xl flex items-center justify-center"
            style={{
              backgroundColor: styles.iconBg,
              color: styles.iconColor,
            }}
          >
            {icon}
          </div>
          <div>
            <p className="text-sm text-neutral-600 font-medium">{label}</p>
            <h3 className="text-2xl font-bold mt-1 text-neutral-900">
              {typeof value === "number" ? value.toLocaleString() : value}
            </h3>
          </div>
        </div>
      </div>
      {change && (
        <div className="mt-4 flex items-center gap-1 text-sm">
          {change.trend === "up" ? (
            <ArrowUp className="w-4 h-4" style={{ color: "#10b981" }} />
          ) : change.trend === "down" ? (
            <ArrowDown className="w-4 h-4" style={{ color: "#ef4444" }} />
          ) : null}
          <span
            style={{
              color:
                change.trend === "up"
                  ? "#10b981"
                  : change.trend === "down"
                    ? "#ef4444"
                    : "#6b7280",
              fontWeight: 500,
            }}
          >
            {change.value}
          </span>
        </div>
      )}
    </div>
  );
}
