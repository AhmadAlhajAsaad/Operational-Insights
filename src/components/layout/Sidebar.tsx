import React from "react";
import {
  Building2,
  Users,
  Package,
  PackageSearch,
  LogOut,
  Upload,
  Github,
} from "lucide-react";
import { useAuth } from "../../context/AuthContext";
import logo from "../../assets/logo.png";

interface SidebarProps {
  currentPage: string;
  onNavigate: (page: string) => void;
}

export function Sidebar({ currentPage, onNavigate }: SidebarProps) {
  const { logout } = useAuth();

  const menuItems = [
    { id: "organizations", label: "Organizations", icon: Building2 },
    { id: "products", label: "Product Details", icon: PackageSearch },
    { id: "users", label: "Users", icon: Users },
    { id: "import", label: "Data Import", icon: Upload },
  ];

  return (
    <div className="sidebar w-64 flex flex-col h-full">
      <div
        className="p-6 border-b"
        style={{
          borderColor: "var(--color-neutral-100)",
        }}
      >
        <div className="flex flex-col items-center gap-2">
          <img
            src={logo}
            alt="EQUANS Logo"
            className="w-16 h-16 rounded-xl object-contain"
          />
          <div>
            <p
              className="text-sm font-medium text-center data-primary"
              style={{
                color: "var(--color-primary)",
                fontFamily: "Roboto, sans-serif",
                fontWeight: 500,
              }}
            >
              Operational Insights
            </p>
          </div>
        </div>
      </div>
      <nav className="flex-1 p-4">
        <ul className="space-y-1">
          {menuItems.map((item) => {
            const Icon = item.icon;
            const isActive = currentPage === item.id;
            return (
              <li key={item.id}>
                <button
                  onClick={() => onNavigate(item.id)}
                  className={
                    "sidebar-nav-item w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-all duration-200 " +
                    (isActive ? "active" : "")
                  }
                  style={{
                    backgroundColor: isActive
                      ? "var(--color-equans-turquoise)"
                      : "transparent",
                    color: isActive
                      ? "var(--color-text-white)"
                      : "var(--color-equans-azure-blue)",
                    fontFamily: "Roboto, sans-serif",
                    border: "2px solid var(--color-equans-azure-blue)",
                    fontWeight: isActive ? 500 : 400,
                  }}
                >
                  <Icon size={20} />
                  <span className="text-sm">{item.label}</span>
                </button>
              </li>
            );
          })}
        </ul>
      </nav>
      <div
        className="p-4 border-t"
        style={{ borderColor: "var(--color-neutral-100)" }}
      >
        <button
          onClick={() => window.open('https://github.com/AhmadAlhajAsaad/', '_blank')}
          className="sidebar-nav-item w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors mb-2"
          style={{
            color: "var(--color-text-secondary)",
            fontFamily: "Roboto, sans-serif",
            border: "3px solid var(--color-equans-azure-blue)",
            fontWeight: 400,
          }}
        >
          <Github size={20} />
          <span className="text-sm">GitHub</span>
        </button>
        <button
          onClick={logout}
          className="sidebar-nav-item w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-colors"
          style={{
            color: "var(--color-text-secondary)",
            fontFamily: "Roboto, sans-serif",
            border: "3px solid var(--color-equans-azure-blue)",
            fontWeight: 400,
          }}
        >
          <LogOut size={20} />
          <span className="text-sm">Sign Out</span>
        </button>
      </div>
    </div>
  );
}
