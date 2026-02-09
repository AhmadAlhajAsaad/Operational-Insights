import React from 'react';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Legend } from 'recharts';

interface LicenseChartProps {
  data: Array<{ name: string; total: number; active: number; unused: number }>;
}

export function LicenseChart({ data }: LicenseChartProps) {
  // Equans Corporate Colors
  const equansColors = {
    primary: '#002439',      // Dark Blue - for primary data (active licenses)
    secondary: '#008163',    // Dark Green - for secondary data 
    accent: '#70BD95',       // Turquoise - for accents
    accompanying: '#0059CE', // Azure Blue - as accompanying color
    muted: 'rgba(0, 36, 57, 0.2)' // 20% opacity for unused/background data
  };

  return (
    <div className="chart-container">
      <ResponsiveContainer width="100%" height={300}>
        <BarChart 
          data={data}
          style={{ fontFamily: 'Roboto, sans-serif' }}
        >
          <CartesianGrid 
            strokeDasharray="3 3" 
            stroke={equansColors.muted}
          />
          <XAxis 
            dataKey="name" 
            tick={{ 
              fill: equansColors.primary, 
              fontSize: 12,
              fontFamily: 'Roboto, sans-serif',
              fontWeight: 400
            }}
            axisLine={{ stroke: equansColors.muted }}
          />
          <YAxis 
            tick={{ 
              fill: equansColors.primary, 
              fontSize: 12,
              fontFamily: 'Roboto, sans-serif',
              fontWeight: 400
            }}
            axisLine={{ stroke: equansColors.muted }}
          />
          <Tooltip 
            contentStyle={{
              backgroundColor: '#ffffff',
              border: `1px solid ${equansColors.muted}`,
              borderRadius: '8px',
              padding: '12px',
              fontFamily: 'Roboto, sans-serif',
              fontSize: '12px',
              boxShadow: '0 4px 6px rgba(57, 37, 0, 0.1)'
            }}
            labelStyle={{
              color: equansColors.primary,
              fontWeight: 500,
              marginBottom: '4px'
            }}
          />
          <Legend 
            wrapperStyle={{ 
              fontSize: '12px', 
              paddingTop: '16px',
              fontFamily: 'Roboto, sans-serif'
            }}
          />
          {/* Primary data: 100% opacity - Active Licenses */}
          <Bar 
            dataKey="active" 
            fill={equansColors.secondary}
            name="Active Licenses" 
            radius={[4, 4, 0, 0]}
          />
          {/* Background/contextual data: 20% opacity - Unused Licenses */}
          <Bar 
            dataKey="unused" 
            fill={equansColors.muted}
            name="Unused Licenses" 
            radius={[4, 4, 0, 0]}
          />
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
}