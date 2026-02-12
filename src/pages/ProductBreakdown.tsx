import React, { useState } from "react";
import { Card } from "../components/ui/Card";
import { StatCard } from "../components/ui/StatCard";
import * as Select from "../components/ui/select";
import { Table } from "../components/ui/Table";
import { LicenseChart } from "../components/charts/LicenseChart";
import { UsageDonut } from "../components/charts/UsageDonut";
import { Package, Users, DollarSign, TrendingUp } from "lucide-react";
import { products, users } from "../data/mockData";

interface ProductBreakdownProps {
  onNavigateToUser: (userId: number) => void;
}

export function ProductBreakdown({ onNavigateToUser }: ProductBreakdownProps) {
  const [selectedProduct, setSelectedProduct] = useState("jira");

  const currentProduct =
    products.find((p) => p.id === selectedProduct) || products[0];

  // Filter users by product
  const productUsers = users.filter((user) =>
    user.licenses.some(
      (license) => license.toLowerCase() === currentProduct.name.toLowerCase(),
    ),
  );

  const userTableColumns = [
    { key: "name", label: "User Name" },
    { key: "email", label: "Email" },
    { key: "department", label: "Department" },
    { key: "lastActive", label: "Last Active", align: "center" as const },
    { key: "status", label: "Status", align: "center" as const },
  ];

  const userTableData = productUsers.map((user) => ({
    ...user,
    status: (
      <span
        className={`inline-flex px-2 py-1 rounded-full text-xs ${
          user.status === "active"
            ? "bg-success-50 text-success-700"
            : "bg-neutral-100 text-neutral-600"
        }`}
      >
        {user.status}
      </span>
    ),
  }));

  // Product comparison data
  const comparisonData = products.map((p) => ({
    name: p.name,
    total: p.totalLicenses,
    active: p.activeLicenses,
    unused: p.unusedLicenses,
  }));

  // Usage distribution for current product
  const usageData = [
    { name: "Active Usage", value: currentProduct.activeLicenses },
    { name: "Unused", value: currentProduct.unusedLicenses },
  ];

  return (
    <div className="space-y-6">
      {/* Filter Section */}
      <Card className="p-6  rounded-r-full">
        <div className="flex items-center justify-between">
          <div>
            <h3 className="mb-1 text-lg font-semibold">Product Analysis</h3>
            <p className="text-sm text-neutral-600">
              Analyze license usage and costs per product
            </p>
          </div>
          <div className="w-64">
            <Select.Select
              value={selectedProduct}
              onValueChange={setSelectedProduct}
            >
              <Select.SelectTrigger>
                <Select.SelectValue placeholder="Select Product" />
              </Select.SelectTrigger>
              <Select.SelectContent>
                {products.map((product) => (
                  <Select.SelectItem key={product.id} value={product.id}>
                    {product.name}
                  </Select.SelectItem>
                ))}
              </Select.SelectContent>
            </Select.Select>
          </div>
        </div>
      </Card>

      {/* Product KPIs */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard
          icon={<Package className="w-5 h-5" />}
          label="Total Licenses"
          variant="blue"
          value={currentProduct.totalLicenses.toString()}
        />
        <StatCard
          icon={<Users className="w-5 h-5" />}
          label="Active Licenses"
          variant="green"
          value={currentProduct.activeLicenses.toString()}
        />
        <StatCard
          icon={<DollarSign className="w-5 h-5" />}
          label="Monthly Cost"
          variant="cyan"
          value={`€${currentProduct.monthlyCost.toLocaleString()}`}
        />
        <StatCard
          icon={<TrendingUp className="w-5 h-5" />}
          label="Total Active Users"
          variant="purple"
          value={currentProduct.activeLicenses.toString()}
        />
      </div>

      {/* Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Product Comparison */}
        <Card className="p-6">
          <h3 className="mb-4 text-lg font-semibold">
            License Comparison Across Products
          </h3>
          <LicenseChart data={comparisonData} />
        </Card>

        {/* Current Product Usage */}
        <Card className="p-6">
          <h3 className="mb-4 text-lg font-semibold">
            {currentProduct.name} Usage Distribution
          </h3>
          <UsageDonut data={usageData} />
          <div className="mt-4 p-4 bg-neutral-50 rounded-lg">
            <div className="flex justify-between items-center">
              <span className="text-sm text-neutral-700">
                Cost per Active License:
              </span>
              <span className="font-semibold text-neutral-900">
                €
                {(
                  currentProduct.monthlyCost / currentProduct.activeLicenses
                ).toFixed(2)}
              </span>
            </div>
            <div className="flex justify-between items-center mt-2">
              <span className="text-sm text-neutral-700">
                Potential Savings:
              </span>
              <span className="font-semibold text-error-600">
                €
                {(
                  (currentProduct.unusedLicenses /
                    currentProduct.totalLicenses) *
                  currentProduct.monthlyCost
                ).toFixed(2)}
              </span>
            </div>
          </div>
        </Card>
      </div>

      {/* Users with this product */}
      <Card className="p-6  ">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h3 className="text-lg font-semibold">
              Users with {currentProduct.name} License
            </h3>
            <p className="text-sm text-neutral-600 mt-1">
              {productUsers.length} users currently assigned
            </p>
          </div>
        </div>
        <div className="relative w-full overflow-x-auto rounded-full border border-[#EAF1F9]">
          <Table
            columns={userTableColumns}
            data={userTableData}
            onRowClick={(row) => onNavigateToUser(row.id)}
            headerClassName="custom-table-header"
            rowClassName={(row: any, index: number) => {
              const colors = [
                "var(--equans-lightblue-20)",
                "var(--equans-green-20)",
                "var(--color-accent-20)",
                "var(--equans-blue-20)",
                "var(--color-primary-20)",
              ];
              return { backgroundColor: colors[index % colors.length] };
            }}
          />
        </div>
      </Card>
    </div>
  );
}
