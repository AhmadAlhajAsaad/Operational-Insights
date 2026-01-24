import React from 'react';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Legend } from 'recharts';

interface LicenseChartProps {
  data: Array<{ name: string; total: number; active: number; unused: number }>;
}

export function LicenseChart({ data }: LicenseChartProps) {
  return (
    <ResponsiveContainer width="100%" height={300}>
      <BarChart data={data}>
        <CartesianGrid strokeDasharray="3 3" stroke="#e1e7ef" />
        <XAxis 
          dataKey="name" 
          tick={{ fill: '#718096', fontSize: 12 }}
        />
        <YAxis 
          tick={{ fill: '#718096', fontSize: 12 }}
        />
        <Tooltip 
          contentStyle={{
            backgroundColor: '#ffffff',
            border: '1px solid #e1e7ef',
            borderRadius: '8px',
            padding: '8px 12px'
          }}
        />
        <Legend 
          wrapperStyle={{ fontSize: '12px', paddingTop: '16px' }}
        />
        <Bar dataKey="active" fill="#00c4a7" name="Active Licenses" radius={[4, 4, 0, 0]} />
        <Bar dataKey="unused" fill="#cbd5e0" name="Unused Licenses" radius={[4, 4, 0, 0]} />
      </BarChart>
    </ResponsiveContainer>
  );
}