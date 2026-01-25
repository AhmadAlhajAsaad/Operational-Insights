import React from 'react';
import { Search, Bell, User } from 'lucide-react';

interface TopbarProps {
  title: string;
  breadcrumb: string;
}

export function Topbar({ title, breadcrumb }: TopbarProps) {
  return (
    <header className="h-16 bg-white border-b border-neutral-200 flex items-center justify-between px-6">
      <div>
        <h1 className="text-xl font-bold text-neutral-900">{title}</h1>
        <p className="text-xs text-neutral-500">{breadcrumb}</p>
      </div>

      <div className="flex items-center gap-4">
        {/* Search */}
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-neutral-400" strokeLinejoin="round" strokeLinecap="round" strokeWidth={1.8} />
          <input
            type="text"
            placeholder="Search..."
            className="pl-10 pr-4 py-2 w-64 rounded-xl border border-[#EAF1F9] bg-[#F1F8FE] focus:outline-none focus:ring-2 focus:ring-[#276FD1] focus:bg-white transition-all text-sm"
          />
        </div>

        {/* Notifications */}
        <button 
          className="p-2 rounded-xl transition-all"
          style={{ backgroundColor: 'transparent' }}
          onMouseEnter={(e) => e.currentTarget.style.backgroundColor = '#EAF1F9'}
          onMouseLeave={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
        >
          <Bell className="w-5 h-5 text-neutral-600" strokeLinejoin="round" strokeLinecap="round" strokeWidth={1.8} />
        </button>

        {/* User Menu */}
        <div className="flex items-center gap-3 pl-4 border-l border-neutral-200">
          <div className="text-right">
            <p className="text-sm font-medium text-neutral-900">Admin User</p>
            <p className="text-xs text-neutral-500">Administrator</p>
          </div>
          <div 
            className="w-9 h-9 rounded-xl flex items-center justify-center text-white"
            style={{ backgroundColor: '#276FD1' }}
          >
            <User className="w-5 h-5" strokeLinejoin="round" strokeLinecap="round" strokeWidth={1.8} />
          </div>
        </div>
      </div>
    </header>
  );
}