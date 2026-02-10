import React from 'react';
import { StatCard } from '../components/ui/StatCard';
import { Card } from '../components/ui/card';
import { TrendChart } from '../components/charts/TrendChart';
import { Button } from '../components/ui/button';
import { DollarSign, Users, Package, TrendingUp, ArrowLeft, User } from 'lucide-react';
import { organizations } from '../data/organizationData';
import { users as allUsers } from '../data/mockData';

interface OrganizationDetailProps {
  organizationId: string;
  onBack: () => void;
  onNavigateToProduct: (orgId: string, productName: string) => void;
  onNavigateToUser: (userId: number) => void;
}

export function OrganizationDetail({ 
  organizationId, 
  onBack, 
  onNavigateToProduct,
  onNavigateToUser 
}: OrganizationDetailProps) {
  const organization = organizations.find(org => org.id === organizationId);
  
  if (!organization) {
    return <div>Organization not found</div>;
  }

  const utilization = Math.round((organization.activeUsers / organization.totalLicenses) * 100);
  
  // Get users for this organization (for now using all users, will be filtered when mockData is updated)
  const orgUsers = allUsers.slice(0, 5); // Temporary - will use organizationId filter

  const productColumns = [
    { key: 'name', label: 'Product' },
    { key: 'licenses', label: 'Licenses', align: 'center' as const },
    { key: 'activeUsers', label: 'Active Users', align: 'center' as const },
    { key: 'cost', label: 'Monthly Cost', align: 'right' as const },
    { key: 'utilization', label: 'Utilization', align: 'center' as const },
  ];

  const userColumns = [
    { key: 'name', label: 'User Name' },
    { key: 'email', label: 'Email' },
    { key: 'department', label: 'Department' },
    { key: 'licenseCount', label: 'Licenses', align: 'center' as const },
    { key: 'status', label: 'Status', align: 'center' as const },
  ];

  // Array of vibrant background colors for product rows
  const productColors = [
    '#EFF6FF', // Light Blue
    '#ECFDF5', // Light Green
    '#FEF3C7', // Light Orange
    '#F3E8FF', // Light Purple
    '#FCE7F3', // Light Pink
    '#DBEAFE', // Blue
    '#D1FAE5', // Green
    '#FDE68A', // Yellow
  ];

  // Array of vibrant background colors for user rows
   const userColors = [
    '#EFF6FF', // Light Blue
    '#ECFDF5', // Light Green
    '#FEF3C7', // Light Orange
    '#F3E8FF', // Light Purple
    '#FCE7F3', // Light Pink
    '#DBEAFE', // Blue
    '#D1FAE5', // Green
    '#FDE68A', // Yellow
  ];

  return (
    <div className="space-y-6">
      {/* Back Button and Header */}
      <div className="flex items-center gap-4">
        <Button 
          variant="outline" 
          onClick={onBack}
          className="border-[#EAF1F9] hover:bg-[#F1F8FE] hover:border-[#276FD1] text-neutral-700"
        >
          <ArrowLeft className="w-4 h-4 mr-2" />
          Back to Overview
        </Button>
      </div>

      <div>
        <h2 className="text-2xl font-bold text-neutral-900 mb-1">
          {organization.name}
        </h2>
        <p className="text-neutral-600">
          <span className="inline-flex px-2.5 py-1 bg-[#EAF1F9] text-[#276FD1] rounded-full text-sm font-medium mr-2">
            {organization.businessUnit}
          </span>
          Detailed cost and product analysis
        </p>
      </div>

      {/* Organization KPIs with Vibrant Colors */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard
          icon={<DollarSign className="w-5 h-5" />}
          label="Monthly Cost"
          value={`€${organization.monthlyCost.toLocaleString()}`}
          change={{ value: '+4.1% vs last month', trend: 'up' }}
          variant="blue"
        />
        <StatCard
          icon={<Package className="w-5 h-5" />}
          label="Total Licenses"
          value={organization.totalLicenses.toString()}
          change={{ value: `${organization.products.length} products`, trend: 'neutral' }}
          variant="purple"
        />
        <StatCard
          icon={<Users className="w-5 h-5" />}
          label="Active Users"
          value={organization.activeUsers.toString()}
          change={{ value: `${orgUsers.length} total users`, trend: 'neutral' }}
          variant="cyan"
        />
        <StatCard
          icon={<TrendingUp className="w-5 h-5" />}
          label="Utilization Rate"
          value={`${utilization}%`}
          change={{ value: '+1.8% vs last month', trend: 'up' }}
          variant="green"
        />
      </div>

      {/* Cost Trend */}
      <Card className="bg-white shadow-sm border-[#EAF1F9]" style={{ borderRadius: '16px' }}>
        <div className="p-6">
          <h3 className="text-lg font-bold text-neutral-900 mb-4">Cost Trend - {organization.name}</h3>
          <TrendChart data={organization.costTrend} />
        </div>
      </Card>

      {/* Products Table - Interactive Drill-down with Colorful Rows */}
      <Card className="bg-white  shadow-sm border-[#EAF1F9]" style={{ borderRadius: '16px' }}>
        <div className="p-6">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-bold text-neutral-900 mb-1">Products & Licenses</h3>
              <p className="text-sm text-neutral-500">
                Click any product to view user-level details
              </p>
            </div>
          </div>

          {/* Custom Colorful Table */}
          <div className="relative w-full overflow-x-auto rounded-full border border-[#EAF1F9]">
            <table className="w-full caption-bottom text-sm">
              <thead className="border-b border-[#EAF1F9]" style={{ backgroundColor: 'var(--color-equans-turquoise)' }}>
                <tr>
                  {productColumns.map((column) => (
                    <th
                      key={column.key}
                      className={`h-12 px-4 align-middle font-semibold text-white whitespace-nowrap ${
                        column.align === 'center' ? 'text-center' : 
                        column.align === 'right' ? 'text-right' : 'text-left'
                      }`}
                    >
                      {column.label}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {organization.products.map((product, index) => {
                  const productUtilization = Math.round((product.activeUsers / product.licenses) * 100);
                  const bgColor = productColors[index % productColors.length];
                  
                  return (
                    <tr
                      key={product.name}
                      onClick={() => onNavigateToProduct(organizationId, product.name)}
                      className="border-b border-[#EAF1F9] last:border-0 transition-all duration-200 cursor-pointer hover:shadow-md hover:scale-[1.01]"
                      style={{ backgroundColor: bgColor }}
                    >
                      <td className="p-4 align-middle whitespace-nowrap">
                        <div className="flex items-center gap-2">
                          <Package className="w-4 h-4 text-[#276FD1]" strokeWidth={2} />
                          <span className="font-semibold text-neutral-900">{product.name}</span>
                        </div>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-center">
                        <span className="font-semibold text-neutral-900">{product.licenses}</span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-center">
                        <span className="font-semibold text-neutral-900">{product.activeUsers}</span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-right">
                        <span className="font-bold text-neutral-900">€{product.cost.toLocaleString()}</span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-center">
                        <span className={`inline-flex px-3 py-1.5 rounded-full text-xs font-bold ${
                          productUtilization >= 75
                            ? 'bg-[#E9FDF2] text-[#059669]'
                            : 'bg-[#FEFBEA] text-[#b45309]'
                        }`}>
                          {productUtilization}%
                        </span>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      </Card>

      {/* Users in Organization with Colorful Rows */}
      <Card className="bg-white shadow-sm border-[#EAF1F9]" style={{ borderRadius: '16px' }}>
        <div className="p-6">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-bold text-neutral-900 mb-1">Users in Organization</h3>
              <p className="text-sm text-neutral-500">
                <span className="font-semibold text-[#276FD1]">{orgUsers.length} users</span> · Click to view individual details
              </p>
            </div>
          </div>

          {/* Custom Colorful Users Table */}
          <div className="relative w-full overflow-x-auto rounded-full border border-[#EAF1F9]">
            <table className="w-full caption-bottom text-sm">
             <thead className="border-b border-[#EAF1F9]" style={{ backgroundColor: 'var(--color-equans-turquoise)' }}>
                <tr>
                  {userColumns.map((column) => (
                    <th
                      key={column.key}
                      className={`h-12 px-4 align-middle font-semibold text-white whitespace-nowrap ${
                        column.align === 'center' ? 'text-center' : 
                        column.align === 'right' ? 'text-right' : 'text-left'
                      }`}
                    >
                      {column.label}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {orgUsers.map((user, index) => {
                  const bgColor = userColors[index % userColors.length];
                  
                  return (
                    <tr
                      key={user.id}
                      onClick={() => onNavigateToUser(user.id)}
                      className="border-b border-[#EAF1F9] last:border-0 transition-all duration-200 cursor-pointer hover:shadow-md hover:scale-[1.01]"
                      style={{ backgroundColor: bgColor }}
                    >
                      <td className="p-4 align-middle whitespace-nowrap">
                        <div className="flex items-center gap-2">
                          <User className="w-4 h-4 text-[#10B981]" strokeWidth={2} />
                          <span className="font-semibold text-neutral-900">{user.name}</span>
                        </div>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap">
                        <span className="text-neutral-600">{user.email}</span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap">
                        <span className="font-medium text-neutral-700">{user.department}</span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-center">
                        <span className="inline-flex items-center justify-center w-8 h-8 rounded-full bg-[#276FD1] text-white font-bold text-xs">
                          {user.licenses.length}
                        </span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-center">
                        <span className={`inline-flex px-3 py-1.5 rounded-full text-xs font-bold ${
                          user.status === 'active' 
                            ? 'bg-[#E9FDF2] text-[#059669]' 
                            : 'bg-neutral-100 text-neutral-600'
                        }`}>
                          {user.status}
                        </span>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      </Card>
    </div>
  );
}