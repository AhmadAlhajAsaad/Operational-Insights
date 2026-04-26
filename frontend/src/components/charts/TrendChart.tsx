import React from 'react';
import { 
  AreaChart, 
  Area, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
  Tooltip, 
  ResponsiveContainer, 
  Legend, 
  TooltipProps 
} from 'recharts';
import { ArrowUp, ArrowDown, Minus, TrendingUp } from 'lucide-react';

// Updated interface to support cost breakdown
interface CostTrendData {
  month: string;
  licenseCost: number;
  consultancyCost: number;
  cost: number;
  users: number;
}

interface TrendChartProps {
  data: CostTrendData[];
  showForecast?: boolean;
  forecastData?: { nextMonth: number; threeMonths: number; percentageChange: number };
}

interface CustomTooltipProps extends TooltipProps<number, string> {
  data: CostTrendData[];
}

const CustomTooltip = ({ active, payload, label, data }: CustomTooltipProps) => {
  if (active && payload && payload.length && data) {
    const currentIndex = data.findIndex(item => item.month === label);
    const currentData = data[currentIndex];
    const previousData = currentIndex > 0 ? data[currentIndex - 1] : null;

    const totalChange = previousData ? currentData.cost - previousData.cost : 0;
    const totalChangePercent = previousData ? ((totalChange / previousData.cost) * 100).toFixed(1) : '0';
    
    const licenseChange = previousData ? currentData.licenseCost - previousData.licenseCost : 0;
    const consultancyChange = previousData ? currentData.consultancyCost - previousData.consultancyCost : 0;

    const getArrow = (change: number, invertColors = false) => {
      const upColor = invertColors ? 'text-red-500' : 'text-green-500';
      const downColor = invertColors ? 'text-green-500' : 'text-red-500';
      
      if (change > 0) return <ArrowUp className={`w-4 h-4 ${upColor}`} />;
      if (change < 0) return <ArrowDown className={`w-4 h-4 ${downColor}`} />;
      return <Minus className="w-4 h-4 text-neutral-400" />;
    };

    return (
      <div className="bg-white border border-[#EAF1F9] rounded-xl shadow-lg p-4 min-w-[300px]">
        <div className="font-bold text-neutral-900 text-base mb-3 pb-2 border-b border-[#EAF1F9]">
          {label}
        </div>
        
        <div className="mb-3 pb-3 border-b border-[#EAF1F9]">
          <div className="flex items-center justify-between mb-1">
            <span className="text-sm font-semibold text-neutral-700">Total Cost</span>
            {previousData && (
              <div className="flex items-center gap-1">
                {getArrow(totalChange, true)}
                <span className={`text-xs font-semibold ${
                  totalChange > 0 ? 'text-red-500' : totalChange < 0 ? 'text-green-500' : 'text-neutral-400'
                }`}>
                  {totalChange > 0 ? '+' : ''}{totalChangePercent}%
                </span>
              </div>
            )}
          </div>
          <span className="text-xl font-bold" style={{ color: '#002439' }}>
            EUR {currentData.cost.toLocaleString()}
          </span>
        </div>

        <div className="mb-2">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-sm" style={{ backgroundColor: '#008163' }} />
              <span className="text-sm text-neutral-600">License Cost</span>
            </div>
            <div className="flex items-center gap-2">
              <span className="font-bold" style={{ color: '#008163' }}>
                EUR {currentData.licenseCost.toLocaleString()}
              </span>
              {previousData && licenseChange !== 0 && (
                <span className={`text-xs ${licenseChange > 0 ? 'text-red-500' : 'text-green-500'}`}>
                  {licenseChange > 0 ? '+' : ''}EUR {Math.abs(licenseChange).toLocaleString()}
                </span>
              )}
            </div>
          </div>
        </div>

        <div className="mb-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-sm" style={{ backgroundColor: '#C865FF' }} />
              <span className="text-sm text-neutral-600">Consultancy</span>
            </div>
            <div className="flex items-center gap-2">
              <span className="font-bold" style={{ color: '#C865FF' }}>
                EUR {currentData.consultancyCost.toLocaleString()}
              </span>
              {previousData && consultancyChange !== 0 && (
                <span className={`text-xs ${consultancyChange > 0 ? 'text-red-500' : 'text-green-500'}`}>
                  {consultancyChange > 0 ? '+' : ''}EUR {Math.abs(consultancyChange).toLocaleString()}
                </span>
              )}
            </div>
          </div>
        </div>

        <div className="pt-2 border-t border-[#EAF1F9]">
          <div className="flex items-center justify-between">
            <span className="text-sm text-neutral-600">Active Users</span>
            <span className="font-bold text-[#0059CE]">
              {currentData.users.toLocaleString()}
            </span>
          </div>
        </div>
      </div>
    );
  }

  return null;
};

export function TrendChart({ data, showForecast, forecastData }: TrendChartProps) {
  const chartData = showForecast && forecastData 
    ? [...data, { 
        month: 'Forecast', 
        licenseCost: Math.round(forecastData.nextMonth * 0.7),
        consultancyCost: Math.round(forecastData.nextMonth * 0.3),
        cost: forecastData.nextMonth, 
        users: data[data.length - 1]?.users || 0 
      }]
    : data;

  return (
    <div>
      <ResponsiveContainer width="100%" height={320}>
        <AreaChart data={chartData}>
          <defs>
            <linearGradient id="licenseCostGradient" x1="0" y1="0" x2="0" y2="1">
              <stop offset="5%" stopColor="#008163" stopOpacity={0.8}/>
              <stop offset="95%" stopColor="#008163" stopOpacity={0.2}/>
            </linearGradient>
            <linearGradient id="consultancyCostGradient" x1="0" y1="0" x2="0" y2="1">
              <stop offset="5%" stopColor="#C865FF" stopOpacity={0.8}/>
              <stop offset="95%" stopColor="#C865FF" stopOpacity={0.2}/>
            </linearGradient>
          </defs>
          
          <CartesianGrid strokeDasharray="3 3" stroke="#e1e7ef" />
          <XAxis 
            dataKey="month" 
            tick={{ fill: '#718096', fontSize: 12, fontFamily: 'Roboto' }}
            axisLine={{ stroke: '#e1e7ef' }}
          />
          <YAxis 
            tick={{ fill: '#718096', fontSize: 12, fontFamily: 'Roboto' }}
            tickFormatter={(value) => `EUR ${(value / 1000).toFixed(0)}k`}
            axisLine={{ stroke: '#e1e7ef' }}
          />
          <Tooltip 
            content={<CustomTooltip data={chartData} />}
            cursor={{ stroke: '#002439', strokeWidth: 1, strokeDasharray: '5 5' }}
          />
          <Legend 
            wrapperStyle={{ fontSize: '12px', paddingTop: '16px', fontFamily: 'Roboto' }}
            iconType="rect"
          />
          
          <Area 
            type="monotone" 
            dataKey="licenseCost" 
            stackId="1"
            stroke="#008163" 
            strokeWidth={2}
            fill="url(#licenseCostGradient)"
            name="License Cost"
          />
          <Area 
            type="monotone" 
            dataKey="consultancyCost" 
            stackId="1"
            stroke="#C865FF" 
            strokeWidth={2}
            fill="url(#consultancyCostGradient)"
            name="Consultancy Cost"
          />
        </AreaChart>
      </ResponsiveContainer>

      {showForecast && forecastData && (
        <div 
          className="mt-4 p-4 rounded-xl flex items-center justify-between"
          style={{ backgroundColor: 'rgba(255, 202, 0, 0.15)', border: '1px solid rgba(255, 202, 0, 0.3)' }}
        >
          <div className="flex items-center gap-3">
            <div 
              className="w-10 h-10 rounded-full flex items-center justify-center"
              style={{ backgroundColor: '#FFCA00' }}
            >
              <TrendingUp className="w-5 h-5 text-neutral-900" />
            </div>
            <div>
              <p className="text-sm font-semibold text-neutral-900">Forecast</p>
              <p className="text-xs text-neutral-600">Projected cost increase</p>
            </div>
          </div>
          <div className="text-right">
            <p className="text-lg font-bold text-neutral-900">
              EUR {forecastData.nextMonth.toLocaleString()}
            </p>
            <p className="text-xs text-amber-700 font-medium">
              +{forecastData.percentageChange}% next month
            </p>
          </div>
          <div className="text-right pl-6 border-l border-amber-200">
            <p className="text-lg font-bold text-neutral-900">
              EUR {forecastData.threeMonths.toLocaleString()}
            </p>
            <p className="text-xs text-neutral-600">3-month projection</p>
          </div>
        </div>
      )}
    </div>
  );
}
