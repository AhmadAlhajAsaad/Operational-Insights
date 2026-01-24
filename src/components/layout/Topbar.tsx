import React from 'react';
import { Bell, Search, User } from 'lucide-react';

interface TopbarProps {
  title: string;
  breadcrumbs?: string[];
  userName?: string;
}

export function Topbar({ title, breadcrumbs, userName = 'Admin User' }: TopbarProps) {
  return (
    <div className="h-16 bg-white border-b border-neutral-100 px-6 flex items-center justify-between shadow-sm">
      {/* Left side - Title & Breadcrumbs */}
      <div>
        <h2 className="text-lg font-bold text-neutral-900">{title}</h2>
        {breadcrumbs && breadcrumbs.length > 0 && (
          <div className="flex items-center gap-2 mt-0.5">
            {breadcrumbs.map((crumb, index) => (
              <React.Fragment key={index}>
                <span className="text-xs text-[#276FD1]">{crumb}</span>
                {index < breadcrumbs.length - 1 && (
                  <span className="text-xs text-neutral-300">/</span>
                )}
              </React.Fragment>
            ))}
          </div>
        )}
      </div>

      {/* Right side - Search & User */}
      <div className="flex items-center gap-4">
        {/* Search */}
        <div className="relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-neutral-400" />
          <input
            type="text"
            placeholder="Search..."
            className="w-64 pl-10 pr-4 py-2 bg-[#F1F8FE] border border-[#EAF1F9] rounded-xl
              focus:outline-none focus:ring-2 focus:ring-[#276FD1] focus:bg-white
              placeholder:text-neutral-400 transition-all duration-200"
          />
        </div>

        {/* Notifications */}
        <button className="p-2.5 bg-[#EAF1F9] hover:bg-[#276FD1] hover:text-white rounded-xl transition-all duration-200 text-[#276FD1]">
          <Bell className="w-5 h-5" />
        </button>

        {/* User Menu */}
        <div className="flex items-center gap-3 pl-4 border-l border-neutral-200">
          <div className="text-right">
            <p className="text-sm font-semibold text-neutral-900">{userName}</p>
            <p className="text-xs text-[#276FD1]">Administrator</p>
          </div>
          <div className="w-10 h-10 bg-[#276FD1] text-white rounded-full flex items-center justify-center shadow-md">
            <User className="w-5 h-5" />
          </div>
        </div>
      </div>
    </div>
  );
}
