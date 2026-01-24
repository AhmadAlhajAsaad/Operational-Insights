import React from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Legend } from 'recharts';

interface TrendChartProps {
  data: Array<{ month: string; cost: number; users: number }>;
}

export function TrendChart({ data }: TrendChartProps) {
  return (
    <ResponsiveContainer width="100%" height={300}>
      <LineChart data={data}>
        <CartesianGrid strokeDasharray="3 3" stroke="#e1e7ef" />
        <XAxis 
          dataKey="month" 
          tick={{ fill: '#718096', fontSize: 12 }}
        />
        <YAxis 
          yAxisId="left"
          tick={{ fill: '#718096', fontSize: 12 }}
        />
        <YAxis 
          yAxisId="right"
          orientation="right"
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
        <Line 
          yAxisId="left"
          type="monotone" 
          dataKey="cost" 
          stroke="#00c4a7" 
          strokeWidth={2}
          dot={{ fill: '#00c4a7', r: 4 }}
          name="Monthly Cost (€)"
        />
        <Line 
          yAxisId="right"
          type="monotone" 
          dataKey="users" 
          stroke="#008c6a" 
          strokeWidth={2}
          dot={{ fill: '#008c6a', r: 4 }}
          name="Active Users"
        />
      </LineChart>
    </ResponsiveContainer>
  );
}