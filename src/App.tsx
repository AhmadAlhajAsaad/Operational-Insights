import React, { useState } from 'react';
import { Sidebar } from './components/layout/Sidebar';
import { Topbar } from './components/layout/Topbar';
import { Login } from './pages/Login';
import { OrganizationOverview } from './pages/OrganizationOverview';
import { OrganizationDetail } from './pages/OrganizationDetail';
import { ProductBreakdown } from './pages/ProductBreakdown';
import { UserDetail } from './pages/UserDetail';

type View = 'login' | 'organizations' | 'organization-detail' | 'product-detail' | 'user-detail' | 'settings';

export default function App() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [currentView, setCurrentView] = useState<View>('organizations');
  const [selectedOrganizationId, setSelectedOrganizationId] = useState<string>('');
  const [selectedProduct, setSelectedProduct] = useState<string>('');
  const [selectedUserId, setSelectedUserId] = useState<number>(1);

  const handleLogin = () => {
    setIsAuthenticated(true);
    setCurrentView('organizations');
  };

  const handleLogout = () => {
    setIsAuthenticated(false);
    setCurrentView('login');
  };

  const handleNavigate = (view: string) => {
    if (view === 'organizations' || view === 'dashboard') {
      setCurrentView('organizations');
    } else {
      setCurrentView(view as View);
    }
  };

  const handleNavigateToOrganization = (orgId: string) => {
    setSelectedOrganizationId(orgId);
    setCurrentView('organization-detail');
  };

  const handleNavigateToProduct = (orgId: string, productName: string) => {
    setSelectedOrganizationId(orgId);
    setSelectedProduct(productName);
    setCurrentView('product-detail');
  };

  const handleNavigateToUser = (userId: number) => {
    setSelectedUserId(userId);
    setCurrentView('user-detail');
  };

  const handleBackToOrganizations = () => {
    setCurrentView('organizations');
  };

  const handleBackToOrganizationDetail = () => {
    setCurrentView('organization-detail');
  };

  // Login view
  if (!isAuthenticated) {
    return <Login onLogin={handleLogin} />;
  }

  // Get page title and breadcrumbs
  const getPageInfo = () => {
    switch (currentView) {
      case 'organizations':
        return { title: 'Organization Overview', breadcrumbs: ['Home', 'Organizations'] };
      case 'organization-detail':
        return { title: 'Organization Details', breadcrumbs: ['Home', 'Organizations', selectedOrganizationId] };
      case 'product-detail':
        return { title: 'Product Details', breadcrumbs: ['Home', 'Organizations', selectedOrganizationId, selectedProduct] };
      case 'user-detail':
        return { title: 'User Details', breadcrumbs: ['Home', 'Organizations', 'User'] };
      case 'settings':
        return { title: 'Settings', breadcrumbs: ['Home', 'Settings'] };
      default:
        return { title: 'Organizations', breadcrumbs: ['Home'] };
    }
  };

  const pageInfo = getPageInfo();

  return (
    <div className="flex h-screen bg-background">
      {/* Sidebar */}
      <Sidebar 
        activeView={currentView === 'organizations' ? 'dashboard' : currentView} 
        onNavigate={handleNavigate}
        onLogout={handleLogout}
      />

      {/* Main Content */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Topbar */}
        <Topbar 
          title={pageInfo.title}
          breadcrumbs={pageInfo.breadcrumbs}
        />

        {/* Page Content */}
        <main className="flex-1 overflow-y-auto p-6">
          <div className="max-w-7xl mx-auto">
            {currentView === 'organizations' && (
              <OrganizationOverview onNavigateToOrganization={handleNavigateToOrganization} />
            )}
            {currentView === 'organization-detail' && (
              <OrganizationDetail 
                organizationId={selectedOrganizationId}
                onBack={handleBackToOrganizations}
                onNavigateToProduct={handleNavigateToProduct}
                onNavigateToUser={handleNavigateToUser}
              />
            )}
            {currentView === 'product-detail' && (
              <ProductBreakdown onNavigateToUser={handleNavigateToUser} />
            )}
            {currentView === 'user-detail' && (
              <UserDetail 
                userId={selectedUserId}
                onBack={handleBackToOrganizationDetail}
              />
            )}
            {currentView === 'settings' && (
              <div className="text-center py-12">
                <h3 className="mb-2">Settings</h3>
                <p className="text-neutral-600">This view is under development</p>
              </div>
            )}
          </div>
        </main>
      </div>
    </div>
  );
}
