import React, { useState } from 'react';
import { Sidebar } from './components/layout/Sidebar';
import { Topbar } from './components/layout/Topbar';
import { Dashboard } from './pages/Dashboard';
import { OrganizationOverview } from './pages/OrganizationOverview';
import { OrganizationDetail } from './pages/OrganizationDetail';
import { Users } from './pages/Users';
import { UserDetail } from './pages/UserDetail';
import { ProductBreakdown } from './pages/ProductBreakdown';

function App() {
  const [currentPage, setCurrentPage] = useState('dashboard');
  const [selectedOrganization, setSelectedOrganization] = useState<string | null>(null);
  const [selectedUser, setSelectedUser] = useState<string | null>(null);

  const handleNavigate = (page: string) => {
    setCurrentPage(page);
    setSelectedOrganization(null);
    setSelectedUser(null);
  };

  const handleNavigateToOrganization = (orgId: string) => {
    setSelectedOrganization(orgId);
    setCurrentPage('organization-detail');
  };

  const handleNavigateToUser = (userId: string) => {
    setSelectedUser(userId);
    setCurrentPage('user-detail');
  };

  const handleBackToOrganizations = () => {
    setSelectedOrganization(null);
    setCurrentPage('organizations');
  };

  const handleBackToUsers = () => {
    setSelectedUser(null);
    setCurrentPage('users');
  };

  const renderPage = () => {
    switch (currentPage) {
      case 'dashboard':
        return <Dashboard />;
      case 'organizations':
        return <OrganizationOverview onNavigateToOrganization={handleNavigateToOrganization} />;
      case 'organization-detail':
        return selectedOrganization ? (
          <OrganizationDetail 
            organizationId={selectedOrganization} 
            onBack={handleBackToOrganizations}
            onNavigateToUser={handleNavigateToUser}
          />
        ) : (
          <OrganizationOverview onNavigateToOrganization={handleNavigateToOrganization} />
        );
      case 'users':
        return <Users onNavigateToUser={handleNavigateToUser} />;
      case 'user-detail':
        return selectedUser !== null ? (
          <UserDetail 
            userId={selectedUser} 
            onBack={handleBackToUsers}
          />
        ) : (
          <Users onNavigateToUser={handleNavigateToUser} />
        );
      case 'products':
        return <ProductBreakdown />;
      default:
        return <Dashboard />;
    }
  };

  const getPageTitle = () => {
    switch (currentPage) {
      case 'dashboard':
        return 'Dashboard';
      case 'organizations':
        return 'Organizations';
      case 'organization-detail':
        return 'Organization Detail';
      case 'users':
        return 'Users';
      case 'user-detail':
        return 'User Detail';
      case 'products':
        return 'Product Breakdown';
      default:
        return 'Dashboard';
    }
  };

  const getBreadcrumb = () => {
    switch (currentPage) {
      case 'dashboard':
        return 'Home';
      case 'organizations':
        return 'Home / Organizations';
      case 'organization-detail':
        return 'Home / Organizations / Detail';
      case 'users':
        return 'Home / Users';
      case 'user-detail':
        return 'Home / Users / Detail';
      case 'products':
        return 'Home / Products';
      default:
        return 'Home';
    }
  };

  return (
    <div className="flex h-screen bg-background">
      <Sidebar currentPage={currentPage} onNavigate={handleNavigate} />
      <div className="flex-1 flex flex-col overflow-hidden">
        <Topbar title={getPageTitle()} breadcrumb={getBreadcrumb()} />
        <main className="flex-1 overflow-y-auto p-6">
          <div className="max-w-7xl mx-auto">
            {renderPage()}
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;