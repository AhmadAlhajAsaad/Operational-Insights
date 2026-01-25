import React from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Legend, TooltipProps } from 'recharts';
import { ArrowUp, ArrowDown, Minus } from 'lucide-react';

interface TrendChartProps {
  data: Array<{ month: string; cost: number; users: number }>;
}

interface CustomTooltipProps extends TooltipProps<number, string> {
  data: Array<{ month: string; cost: number; users: number }>;
}

const CustomTooltip = ({ active, payload, label, data }: CustomTooltipProps) => {
  if (active && payload && payload.length && data) {
    const currentIndex = data.findIndex(item => item.month === label);
    const currentData = data[currentIndex];
    const previousData = currentIndex > 0 ? data[currentIndex - 1] : null;

    // Calculate changes
    const costChange = previousData ? currentData.cost - previousData.cost : 0;
    const costChangePercent = previousData ? ((costChange / previousData.cost) * 100).toFixed(1) : '0';
    const usersChange = previousData ? currentData.users - previousData.users : 0;
    const usersChangePercent = previousData ? ((usersChange / previousData.users) * 100).toFixed(1) : '0';

    const getCostArrow = () => {
      if (costChange > 0) return <ArrowUp className="w-4 h-4 text-red-500" />;
      if (costChange < 0) return <ArrowDown className="w-4 h-4 text-green-500" />;
      return <Minus className="w-4 h-4 text-neutral-400" />;
    };

    const getUsersArrow = () => {
      if (usersChange > 0) return <ArrowUp className="w-4 h-4 text-green-500" />;
      if (usersChange < 0) return <ArrowDown className="w-4 h-4 text-red-500" />;
      return <Minus className="w-4 h-4 text-neutral-400" />;
    };

    return (
      <div className="bg-white border border-[#EAF1F9] rounded-xl shadow-lg p-4 min-w-[280px]">
        <div className="font-bold text-neutral-900 text-base mb-3 pb-2 border-b border-[#EAF1F9]">
          {label}
        </div>
        
        {/* Monthly Cost */}
        <div className="mb-3">
          <div className="flex items-center justify-between mb-1">
            <span className="text-sm text-neutral-600">Monthly Cost</span>
            {previousData && (
              <div className="flex items-center gap-1">
                {getCostArrow()}
                <span className={`text-xs font-semibold ${
                  costChange > 0 ? 'text-red-500' : costChange < 0 ? 'text-green-500' : 'text-neutral-400'
                }`}>
                  {costChange > 0 ? '+' : ''}{costChangePercent}%
                </span>
              </div>
            )}
          </div>
          <div className="flex items-baseline gap-2">
            <span className="text-lg font-bold text-[#00c4a7]">
              €{currentData.cost.toLocaleString()}
            </span>
            {previousData && costChange !== 0 && (
              <span className={`text-xs font-medium ${
                costChange > 0 ? 'text-red-500' : 'text-green-500'
              }`}>
                {costChange > 0 ? '+' : ''}€{Math.abs(costChange).toLocaleString()}
              </span>
            )}
          </div>
        </div>

        {/* Active Users */}
        <div>
          <div className="flex items-center justify-between mb-1">
            <span className="text-sm text-neutral-600">Active Users</span>
            {previousData && (
              <div className="flex items-center gap-1">
                {getUsersArrow()}
                <span className={`text-xs font-semibold ${
                  usersChange > 0 ? 'text-green-500' : usersChange < 0 ? 'text-red-500' : 'text-neutral-400'
                }`}>
                  {usersChange > 0 ? '+' : ''}{usersChangePercent}%
                </span>
              </div>
            )}
          </div>
          <div className="flex items-baseline gap-2">
            <span className="text-lg font-bold text-[#008c6a]">
              {currentData.users.toLocaleString()}
            </span>
            {previousData && usersChange !== 0 && (
              <span className={`text-xs font-medium ${
                usersChange > 0 ? 'text-green-500' : 'text-red-500'
              }`}>
                {usersChange > 0 ? '+' : ''}{Math.abs(usersChange)}
              </span>
            )}
          </div>
        </div>
      </div>
    );
  }

  return null;
};

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
          content={<CustomTooltip data={data} />}
          cursor={{ stroke: '#276FD1', strokeWidth: 1, strokeDasharray: '5 5' }}
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
          activeDot={{ r: 6 }}
          name="Monthly Cost (€)"
        />
        <Line 
          yAxisId="right"
          type="monotone" 
          dataKey="users" 
          stroke="#008c6a" 
          strokeWidth={2}
          dot={{ fill: '#008c6a', r: 4 }}
          activeDot={{ r: 6 }}
          name="Active Users"
        />
      </LineChart>
    </ResponsiveContainer>
  );
}