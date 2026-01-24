import React, { useState } from 'react';
import { Card } from '../components/ui/Card';
import { Select } from '../components/ui/Select';
import { Table } from '../components/ui/Table';
import { LicenseChart } from '../components/charts/LicenseChart';
import { UsageDonut } from '../components/charts/UsageDonut';
import { Package, Users, DollarSign, TrendingUp } from 'lucide-react';
import { products, users } from '../data/mockData';

interface ProductBreakdownProps {
  onNavigateToUser: (userId: number) => void;
}

export function ProductBreakdown({ onNavigateToUser }: ProductBreakdownProps) {
  const [selectedProduct, setSelectedProduct] = useState('jira');

  const productOptions = products.map(p => ({
    value: p.id,
    label: p.name
  }));

  const currentProduct = products.find(p => p.id === selectedProduct) || products[0];

  // Filter users by product
  const productUsers = users.filter(user => 
    user.licenses.some(license => 
      license.toLowerCase() === currentProduct.name.toLowerCase()
    )
  );

  const userTableColumns = [
    { key: 'name', label: 'User Name' },
    { key: 'email', label: 'Email' },
    { key: 'department', label: 'Department' },
    { key: 'lastActive', label: 'Last Active', align: 'center' as const },
    { key: 'status', label: 'Status', align: 'center' as const },
  ];

  const userTableData = productUsers.map(user => ({
    ...user,
    status: (
      <span className={`inline-flex px-2 py-1 rounded-full text-xs ${
        user.status === 'active' 
          ? 'bg-success-50 text-success-700' 
          : 'bg-neutral-100 text-neutral-600'
      }`}>
        {user.status}
      </span>
    )
  }));

  // Product comparison data
  const comparisonData = products.map(p => ({
    name: p.name,
    total: p.totalLicenses,
    active: p.activeLicenses,
    unused: p.unusedLicenses
  }));

  // Usage distribution for current product
  const usageData = [
    { name: 'Active Usage', value: currentProduct.activeLicenses },
    { name: 'Unused', value: currentProduct.unusedLicenses }
  ];

  return (
    <div className="space-y-6">
      {/* Filter Section */}
      <Card>
        <div className="flex items-center justify-between">
          <div>
            <h3 className="mb-1">Product Analysis</h3>
            <p className="caption text-neutral-600">
              Analyze license usage and costs per product
            </p>
          </div>
          <div className="w-64">
            <Select
              label="Select Product"
              options={productOptions}
              value={selectedProduct}
              onChange={(e) => setSelectedProduct(e.target.value)}
            />
          </div>
        </div>
      </Card>

      {/* Product KPIs */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Card>
          <div className="flex items-start gap-3">
            <div className="p-2 bg-primary-50 rounded-lg text-primary-600">
              <Package className="w-5 h-5" />
            </div>
            <div className="flex-1">
              <p className="caption text-neutral-600">Total Licenses</p>
              <p className="text-2xl font-semibold mt-1">{currentProduct.totalLicenses}</p>
            </div>
          </div>
        </Card>
        <Card>
          <div className="flex items-start gap-3">
            <div className="p-2 bg-primary-50 rounded-lg text-primary-600">
              <Users className="w-5 h-5" />
            </div>
            <div className="flex-1">
              <p className="caption text-neutral-600">Active Licenses</p>
              <p className="text-2xl font-semibold mt-1">{currentProduct.activeLicenses}</p>
            </div>
          </div>
        </Card>
        <Card>
          <div className="flex items-start gap-3">
            <div className="p-2 bg-primary-50 rounded-lg text-primary-600">
              <DollarSign className="w-5 h-5" />
            </div>
            <div className="flex-1">
              <p className="caption text-neutral-600">Monthly Cost</p>
              <p className="text-2xl font-semibold mt-1">€{currentProduct.monthlyCost.toLocaleString()}</p>
            </div>
          </div>
        </Card>
        <Card>
          <div className="flex items-start gap-3">
            <div className="p-2 bg-primary-50 rounded-lg text-primary-600">
              <TrendingUp className="w-5 h-5" />
            </div>
            <div className="flex-1">
              <p className="caption text-neutral-600">Utilization Rate</p>
              <p className="text-2xl font-semibold mt-1">{currentProduct.utilizationRate}%</p>
              <p className={`caption mt-1 ${currentProduct.utilizationRate >= 80 ? 'text-success-600' : 'text-error-600'}`}>
                {currentProduct.utilizationRate >= 80 ? 'Excellent' : 'Needs improvement'}
              </p>
            </div>
          </div>
        </Card>
      </div>

      {/* Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Product Comparison */}
        <Card>
          <h3 className="mb-4">License Comparison Across Products</h3>
          <LicenseChart data={comparisonData} />
        </Card>

        {/* Current Product Usage */}
        <Card>
          <h3 className="mb-4">{currentProduct.name} Usage Distribution</h3>
          <UsageDonut data={usageData} />
          <div className="mt-4 p-4 bg-neutral-50 rounded-lg">
            <div className="flex justify-between items-center">
              <span className="caption text-neutral-700">Cost per Active License:</span>
              <span className="font-semibold text-neutral-900">
                €{(currentProduct.monthlyCost / currentProduct.activeLicenses).toFixed(2)}
              </span>
            </div>
            <div className="flex justify-between items-center mt-2">
              <span className="caption text-neutral-700">Potential Savings:</span>
              <span className="font-semibold text-error-600">
                €{((currentProduct.unusedLicenses / currentProduct.totalLicenses) * currentProduct.monthlyCost).toFixed(2)}
              </span>
            </div>
          </div>
        </Card>
      </div>

      {/* Users with this product */}
      <Card>
        <div className="flex items-center justify-between mb-4">
          <div>
            <h3>Users with {currentProduct.name} License</h3>
            <p className="caption text-neutral-600 mt-1">
              {productUsers.length} users currently assigned
            </p>
          </div>
        </div>
        <Table 
          columns={userTableColumns} 
          data={userTableData}
          onRowClick={(row) => onNavigateToUser(row.id)}
        />
      </Card>

      {/* Recommendations */}
      <Card>
        <h3 className="mb-3">Optimization Recommendations</h3>
        <div className="space-y-3">
          {currentProduct.utilizationRate < 80 && (
            <div className="flex gap-3 p-3 bg-warning-50 border border-warning-200 rounded-lg">
              <div className="w-1 bg-warning-500 rounded-full"></div>
              <div className="flex-1">
                <p className="font-medium text-warning-900">Low Utilization Detected</p>
                <p className="caption text-warning-700 mt-1">
                  Consider reducing licenses by {currentProduct.unusedLicenses} to optimize costs
                </p>
              </div>
            </div>
          )}
          {currentProduct.utilizationRate >= 80 && (
            <div className="flex gap-3 p-3 bg-success-50 border border-success-200 rounded-lg">
              <div className="w-1 bg-success-500 rounded-full"></div>
              <div className="flex-1">
                <p className="font-medium text-success-900">Optimal Usage</p>
                <p className="caption text-success-700 mt-1">
                  License allocation is well-optimized for this product
                </p>
              </div>
            </div>
          )}
        </div>
      </Card>
    </div>
  );
}
