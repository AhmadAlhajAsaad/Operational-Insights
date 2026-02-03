import React from 'react';
import { 
  Building2, 
  Users, 
  Package,
  PackageSearch,
  LogOut 
} from 'lucide-react';
import { useAuth } from '../../context/AuthContext';

interface SidebarProps {
  currentPage: string;
  onNavigate: (page: string) => void;
}

export function Sidebar({ currentPage, onNavigate }: SidebarProps) {
  const { logout } = useAuth();

  const menuItems = [
    { id: 'organizations', label: 'Organizations', icon: Building2 },
    { id: 'users', label: 'Users', icon: Users },
    { id: 'products', label: 'Product Breakdown', icon: Package },
    { id: 'product-details', label: 'Product Details', icon: PackageSearch },
  ];

  return (
    <div className="w-64 bg-white border-r border-neutral-200 flex flex-col h-full">
      {/* Logo Header */}
      <div 
        className="p-6 border-b border-neutral-100"
        style={{ background: 'linear-gradient(135deg, #EAF1F9 0%, #F1F8FE 100%)' }}
      >
        <div className="flex items-center gap-3">
          <div 
            className="w-10 h-10 rounded-xl flex items-center justify-center font-bold text-white text-lg"
            style={{ backgroundColor: '#276FD1' }}
          >
            E
          </div>
          <div>
            <h1 className="font-bold text-lg text-neutral-900">EQUANS</h1>
            <p className="text-xs font-medium" style={{ color: '#276FD1' }}>Operational Insights</p>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 p-4">
        <ul className="space-y-1">
          {menuItems.map((item) => {
            const Icon = item.icon;
            const isActive = currentPage === item.id;
            return (
              <li key={item.id}>
                <button
                  onClick={() => onNavigate(item.id)}
                  className={`w-full flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium transition-all ${
                    isActive
                      ? 'text-white shadow-md'
                      : 'text-neutral-600 hover:text-[#276FD1]'
                  }`}
                  style={{
                    backgroundColor: isActive ? '#276FD1' : 'transparent',
                  }}
                  onMouseEnter={(e) => {
                    if (!isActive) {
                      e.currentTarget.style.backgroundColor = '#EAF1F9';
                    }
                  }}
                  onMouseLeave={(e) => {
                    if (!isActive) {
                      e.currentTarget.style.backgroundColor = 'transparent';
                    }
                  }}
                >
                  <Icon className="w-5 h-5" strokeLinejoin="round" strokeLinecap="round" strokeWidth={1.8} />
                  {item.label}
                </button>
              </li>
            );
          })}
        </ul>
      </nav>

      {/* Logout Button */}
      <div className="p-4 border-t border-neutral-100">
        <button 
          onClick={logout}
          className="w-full flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium text-neutral-600 transition-all"
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = '#FCECED';
            e.currentTarget.style.color = '#ef4444';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = 'transparent';
            e.currentTarget.style.color = '#4a5568';
          }}
        >
          <LogOut className="w-5 h-5" strokeLinejoin="round" strokeLinecap="round" strokeWidth={1.8} />
          Logout
        </button>
      </div>
    </div>
  );
}
