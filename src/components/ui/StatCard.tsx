import React from 'react';
import { ArrowUp, ArrowDown } from 'lucide-react';

interface StatCardProps {
  icon: React.ReactNode;
  label: string;
  value: string;
  change?: {
    value: string;
    trend: 'up' | 'down' | 'neutral';
  };
  onClick?: () => void;
  variant?: 'default' | 'blue' | 'green' | 'yellow' | 'pink' | 'purple' | 'cyan' | 'mint';
}

const variantStyles = {
  default: {
    bg: '#FFFFFF',
    iconBg: '#EAF1F9',
    iconColor: '#276FD1',
    border: '#EAF1F9',
  },
  blue: {
    bg: '#EAF1F9',
    iconBg: '#276FD1',
    iconColor: '#FFFFFF',
    border: 'rgba(39, 111, 209, 0.2)',
  },
  green: {
    bg: '#E9FDF2',
    iconBg: '#10b981',
    iconColor: '#FFFFFF',
    border: 'rgba(16, 185, 129, 0.2)',
  },
  yellow: {
    bg: '#FEFBEA',
    iconBg: '#f59e0b',
    iconColor: '#FFFFFF',
    border: 'rgba(245, 158, 11, 0.2)',
  },
  pink: {
    bg: '#FCECED',
    iconBg: '#ef4444',
    iconColor: '#FFFFFF',
    border: 'rgba(239, 68, 68, 0.2)',
  },
  purple: {
    bg: '#F9F1FE',
    iconBg: '#a855f7',
    iconColor: '#FFFFFF',
    border: 'rgba(168, 85, 247, 0.2)',
  },
  cyan: {
    bg: '#EDFFFF',
    iconBg: '#06b6d4',
    iconColor: '#FFFFFF',
    border: 'rgba(6, 182, 212, 0.2)',
  },
  mint: {
    bg: '#EDFCF5',
    iconBg: '#34d399',
    iconColor: '#FFFFFF',
    border: 'rgba(52, 211, 153, 0.2)',
  },
};

export function StatCard({ icon, label, value, change, onClick, variant = 'default' }: StatCardProps) {
  const styles = variantStyles[variant];
  
  return (
    <div
      className={`p-6 transition-all shadow-sm ${onClick ? 'cursor-pointer hover:shadow-lg hover:scale-[1.02]' : ''}`}
      style={{ 
        borderRadius: '16px',
        backgroundColor: styles.bg,
        border: `1px solid ${styles.border}`
      }}
      onClick={onClick}
      role={onClick ? 'button' : undefined}
      tabIndex={onClick ? 0 : undefined}
      onKeyDown={(e) => {
        if (onClick && (e.key === 'Enter' || e.key === ' ')) {
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
              color: styles.iconColor
            }}
          >
            {icon}
          </div>
          <div>
            <p className="text-sm text-neutral-600 font-medium">{label}</p>
            <h3 className="text-2xl font-bold mt-1 text-neutral-900">{value}</h3>
          </div>
        </div>
      </div>
      {change && (
        <div className="mt-4 flex items-center gap-1 text-sm">
          {change.trend === 'up' ? (
            <ArrowUp className="w-4 h-4" style={{ color: '#10b981' }} />
          ) : change.trend === 'down' ? (
            <ArrowDown className="w-4 h-4" style={{ color: '#ef4444' }} />
          ) : null}
          <span style={{
            color: change.trend === 'up' ? '#10b981' : change.trend === 'down' ? '#ef4444' : '#6b7280',
            fontWeight: 500
          }}>
            {change.value}
          </span>
        </div>
      )}
    </div>
  );
}