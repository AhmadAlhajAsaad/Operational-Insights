import React from "react";
import {
  PieChart,
  Pie,
  Cell,
  ResponsiveContainer,
  Legend,
  Tooltip,
} from "recharts";

interface UsageDonutProps {
  data: Array<{ name: string; value: number }>;
}

const COLORS = [
  "var(--color-equans-turquoise)",
  "var(--color-equans-dark-green)",
  "var(--color-equans-dark-blue)",
  "var(--color-equans-azure-blue)",
];

export function UsageDonut({ data }: UsageDonutProps) {
  return (
    <ResponsiveContainer width="100%" height={300}>
      <PieChart>
        <Pie
          data={data}
          cx="50%"
          cy="50%"
          innerRadius={60}
          outerRadius={100}
          paddingAngle={5}
          dataKey="value"
        >
          {data.map((entry, index) => (
            <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
          ))}
        </Pie>
        <Tooltip
          contentStyle={{
            backgroundColor: "#ffffff",
            border: "1px solid #e1e7ef",
            borderRadius: "8px",
            padding: "8px 12px",
          }}
        />
        <Legend wrapperStyle={{ fontSize: "12px" }} iconType="circle" />
      </PieChart>
    </ResponsiveContainer>
  );
}
