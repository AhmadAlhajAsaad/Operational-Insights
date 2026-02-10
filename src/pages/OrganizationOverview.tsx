import React, { useState } from "react";
import { StatCard } from "../components/ui/StatCard";
import { Card } from "../components/ui/card";
import { CylindricalMonthlyChart } from "../components/charts/CylindricalMonthlyChart";
import { Building2, DollarSign, Users, TrendingUp, Search } from "lucide-react";
import { organizations } from "../data/organizationData";

interface OrganizationOverviewProps {
  onNavigateToOrganization: (orgId: string) => void;
}

export function OrganizationOverview({
  onNavigateToOrganization,
}: OrganizationOverviewProps) {
  const [searchTerm, setSearchTerm] = useState("");

  // Calculate aggregate metrics
  const totalCost = organizations.reduce(
    (sum, org) => sum + org.monthlyCost,
    0,
  );
  const totalLicenses = organizations.reduce(
    (sum, org) => sum + org.totalLicenses,
    0,
  );
  const totalActiveUsers = organizations.reduce(
    (sum, org) => sum + org.activeUsers,
    0,
  );
  const avgUtilization = Math.round((totalActiveUsers / totalLicenses) * 100);

  // Prepare data for cylindrical chart - Last 6 months by business unit
  const cylindricalChartData = organizations[0].costTrend.map(
    (_, monthIndex) => {
      const month = organizations[0].costTrend[monthIndex].month;

      // Group by business unit
      const businessUnitData: {
        [key: string]: { cost: number; activeUsers: number };
      } = {};
      let totalMonthCost = 0;

      organizations.forEach((org) => {
        const cost = org.costTrend[monthIndex].cost;
        if (!businessUnitData[org.businessUnit]) {
          businessUnitData[org.businessUnit] = { cost: 0, activeUsers: 0 };
        }
        businessUnitData[org.businessUnit].cost += cost;
        businessUnitData[org.businessUnit].activeUsers += org.activeUsers;
        totalMonthCost += cost;
      });

      // Convert to array with percentages
      const businessUnits = Object.entries(businessUnitData).map(
        ([unit, data]) => ({
          businessUnit: unit,
          cost: data.cost,
          activeUsers: data.activeUsers,
          percentage: (data.cost / totalMonthCost) * 100,
          color: getBusinessUnitColor(unit),
        }),
      );

      return {
        month,
        totalCost: totalMonthCost,
        businessUnits,
      };
    },
  );

  const tableColumns = [
    { key: "id", label: "Organization ID", align: "left" as const },
    { key: "name", label: "Organization Name" },
    { key: "businessUnit", label: "Business Unit" },
    { key: "totalLicenses", label: "Total Licenses", align: "center" as const },
    { key: "activeUsers", label: "Active Users", align: "center" as const },
    { key: "monthlyCost", label: "Monthly Cost", align: "right" as const },
    { key: "utilization", label: "Utilization", align: "center" as const },
  ];

  // Filter organizations based on search term
  const filteredOrganizations = organizations.filter(
    (org) =>
      org.id.toLowerCase().includes(searchTerm.toLowerCase()) ||
      org.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      org.businessUnit.toLowerCase().includes(searchTerm.toLowerCase()),
  );

  const tableData = filteredOrganizations.map((org) => ({
    ...org,
    monthlyCost: `€${org.monthlyCost.toLocaleString()}`,
    utilization: (
      <span
        className={`inline-flex px-2.5 py-1 rounded-full text-xs font-semibold ${
          (org.activeUsers / org.totalLicenses) * 100 >= 75
            ? "bg-[#E9FDF2] text-[#059669]"
            : "bg-[#FEFBEA] text-[#b45309]"
        }`}
      >
        {Math.round((org.activeUsers / org.totalLicenses) * 100)}%
      </span>
    ),
  }));

  return (
    <div className="space-y-6">
      {/* Page Header */}
      <div>
        <h2 className="text-2xl font-bold text-neutral-900 mb-2">
          Organization Overview
        </h2>
        <p className="text-neutral-600">
          Cost and license insights by organization and business unit. Click any
          row to drill down.
        </p>
      </div>

      {/* KPI Cards - Organization Level with Vibrant Colors */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard
          icon={
            <DollarSign
              className="w-5 h-5"
              strokeLinejoin="round"
              strokeLinecap="round"
              strokeWidth={1.8}
            />
          }
          label="Total Monthly Cost"
          value={`€${totalCost.toLocaleString()}`}
          change={{ value: "+5.2% vs last month", trend: "up" }}
          variant="blue"
        />
        <StatCard
          icon={
            <Building2
              className="w-5 h-5"
              strokeLinejoin="round"
              strokeLinecap="round"
              strokeWidth={1.8}
            />
          }
          label="Organizations"
          value={organizations.length.toString()}
          variant="purple"
        />
        <StatCard
          icon={
            <Users
              className="w-5 h-5"
              strokeLinejoin="round"
              strokeLinecap="round"
              strokeWidth={1.8}
            />
          }
          label="Total Active Users"
          value={totalActiveUsers.toString()}
          variant="cyan"
        />
        <StatCard
          icon={
            <TrendingUp
              className="w-5 h-5"
              strokeLinejoin="round"
              strokeLinecap="round"
              strokeWidth={1.8}
            />
          }
          label="Avg Utilization"
          value={`${avgUtilization}%`}
          change={{ value: "+2.3% vs last month", trend: "up" }}
          variant="green"
        />
      </div>

      {/* Cylindrical Monthly Cost Chart by Business Unit */}
      <Card
        className="bg-white shadow-sm border chart-container"
        style={{
          borderRadius: "12px",
          borderColor: "var(--color-primary-20)",
          fontFamily: "Roboto, sans-serif",
        }}
      >
        <div className="p-6">
          <h3
            className="mb-2 text-lg font-bold"
            style={{
              color: "var(--color-text-primary)",
              fontFamily: "Roboto, sans-serif",
              fontWeight: 700,
            }}
          >
            Monthly Cost Trend by Business Unit
          </h3>
          <p
            className="text-sm mb-6 data-secondary"
            style={{
              color: "var(--color-text-secondary)",
              fontFamily: "Roboto, sans-serif",
              fontWeight: 400,
            }}
          >
            Hover over each business unit segment to view detailed costs and
            percentages
          </p>
          <CylindricalMonthlyChart data={cylindricalChartData} />
        </div>
      </Card>

      {/* Organizations Table with Colorful Rows */}
      <Card
        className="'var(--color-text-secondary)' shadow-sm border chart-container"
        style={{
          borderRadius: "16px",
          borderColor: "var(--color-primary-20)",
          fontFamily: "Roboto, sans-serif",
        }}
      >
        <div className="p-6">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-bold text-neutral-900 mb-1">
                Organizations by Cost
              </h3>
              <p className="text-sm text-neutral-500">
                Search by Organization ID, name, or business unit
              </p>
            </div>
          </div>

          {/* Search Input */}
          <div className="mb-4">
            <div className="relative">
              <Search
                className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-neutral-400"
                strokeLinejoin="round"
                strokeLinecap="round"
                strokeWidth={1.8}
              />
              <input
                type="text"
                placeholder="Search by Organization ID (e.g., 21959ca7-236b-11j7-k470)..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="w-full pl-10 pr-4 py-3 bg-[#F1F8FE] border border-[#EAF1F9] rounded-full focus:outline-none focus:ring-2 focus:ring-[#276FD1] focus:bg-white transition-all"
              />
            </div>
            {searchTerm && (
              <p className="text-sm text-[#276FD1] mt-2 font-medium">
                Showing {filteredOrganizations.length} of {organizations.length}{" "}
                organizations
              </p>
            )}
          </div>

          {/* Custom Table with Colored Rows */}
          <div className="relative w-full overflow-x-auto rounded-full  border border-[#EAF1F9]">
            <table className="w-full caption-bottom text-sm">
              <thead
                className="border-b border-[#EAF1F9]"
                style={{ backgroundColor: "var(--color-equans-turquoise)" }}
              >
                {" "}
                <tr>
                  {tableColumns.map((column) => (
                    <th
                      key={column.key}
                      className={`h-12 px-4 align-middle font-semibold text-white whitespace-nowrap ${
                        column.align === "center"
                          ? "text-center"
                          : column.align === "right"
                            ? "text-right"
                            : "text-left"
                      }`}
                    >
                      {column.label}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {filteredOrganizations.map((org, index) => {
                  const bgColor = getBusinessUnitBackgroundColor(
                    org.businessUnit,
                  );
                  const utilization = Math.round(
                    (org.activeUsers / org.totalLicenses) * 100,
                  );

                  return (
                    <tr
                      key={org.id}
                      onClick={() => onNavigateToOrganization(org.id)}
                      className="border-b border-[#EAF1F9] last:border-0 transition-all duration-200 cursor-pointer hover:shadow-md hover:scale-[1.01]"
                      style={{ backgroundColor: bgColor }}
                    >
                      <td className="p-4 align-middle whitespace-nowrap text-left">
                        <span className="font-mono text-xs text-neutral-600">
                          {org.id}
                        </span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap">
                        <span className="font-semibold text-neutral-900">
                          {org.name}
                        </span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap">
                        <span className="inline-flex items-center gap-2">
                          <div
                            className="w-3 h-3 rounded-full"
                            style={{
                              backgroundColor: getBusinessUnitColor(
                                org.businessUnit,
                              ),
                            }}
                          />
                          <span className="font-medium text-neutral-700">
                            {org.businessUnit}
                          </span>
                        </span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-center">
                        <span className="font-semibold text-neutral-900">
                          {org.totalLicenses}
                        </span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-center">
                        <span className="font-semibold text-neutral-900">
                          {org.activeUsers}
                        </span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-right">
                        <span className="font-bold text-neutral-900">
                          €{org.monthlyCost.toLocaleString()}
                        </span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-center">
                        <span
                          className={`inline-flex px-3 py-1.5 rounded-full text-xs font-bold ${
                            utilization >= 75
                              ? "bg-[#E9FDF2] text-[#059669]"
                              : "bg-[#FEFBEA] text-[#b45309]"
                          }`}
                        >
                          {utilization}%
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

      {/* Cost Distribution Insights */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card
          className="bg-[#F1F8FE] shadow-sm border-[#EAF1F9]"
          style={{ borderRadius: "16px" }}
        >
          <div className="p-6">
            <h3 className="text-lg font-bold text-neutral-900 mb-4">
              Top Cost Drivers
            </h3>
            <div className="space-y-3">
              {[...organizations]
                .sort((a, b) => b.monthlyCost - a.monthlyCost)
                .slice(0, 5)
                .map((org, index) => (
                  <div
                    key={org.id}
                    className="flex items-center justify-between bg-white p-3 rounded-full"
                  >
                    <div className="flex items-center gap-3">
                      <div className="w-8 h-8 bg-[#276FD1] text-white rounded-full flex items-center justify-center font-bold text-sm">
                        {index + 1}
                      </div>
                      <div>
                        <div className="font-semibold text-neutral-900">
                          {org.name}
                        </div>
                        <div className="text-xs text-neutral-500">
                          {org.businessUnit}
                        </div>
                      </div>
                    </div>
                    <div className="text-right">
                      <div className="font-bold text-[#276FD1]">
                        €{org.monthlyCost.toLocaleString()}
                      </div>
                      <div className="text-xs text-neutral-500">
                        {Math.round((org.monthlyCost / totalCost) * 100)}% of
                        total
                      </div>
                    </div>
                  </div>
                ))}
            </div>
          </div>
        </Card>

        <Card
          className="bg-[#EDFCF5] shadow-sm border-[#E9FDF2]"
          style={{ borderRadius: "16px" }}
        >
          <div className="p-6">
            <h3 className="text-lg font-bold text-neutral-900 mb-4">
              Utilization Leaders
            </h3>
            <div className="space-y-3">
              {[...organizations]
                .sort(
                  (a, b) =>
                    b.activeUsers / b.totalLicenses -
                    a.activeUsers / a.totalLicenses,
                )
                .slice(0, 5)
                .map((org, index) => {
                  const utilization = Math.round(
                    (org.activeUsers / org.totalLicenses) * 100,
                  );
                  return (
                    <div
                      key={org.id}
                      className="flex items-center justify-between bg-white p-3 rounded-xl"
                    >
                      <div className="flex items-center gap-3">
                        <div className="w-8 h-8 bg-[#10b981] text-white rounded-lg flex items-center justify-center font-bold text-sm">
                          {index + 1}
                        </div>
                        <div>
                          <div className="font-semibold text-neutral-900">
                            {org.name}
                          </div>
                          <div className="text-xs text-neutral-500">
                            {org.activeUsers} / {org.totalLicenses} users
                          </div>
                        </div>
                      </div>
                      <div
                        className={`px-3 py-1.5 rounded-full text-sm font-bold ${
                          utilization >= 75
                            ? "bg-[#E9FDF2] text-[#059669]"
                            : "bg-[#FEFBEA] text-[#b45309]"
                        }`}
                      >
                        {utilization}%
                      </div>
                    </div>
                  );
                })}
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}
// Helper function to get Equans corporate colors for business units
function getBusinessUnitColor(businessUnit: string): string {
  const equansColors: { [key: string]: string } = {
    "Digital Services": "var(--color-equans-turquoise)", // Main Corporate: Turquoise Green
    "IT Operations": "var(--color-equans-dark-green)", // Accompanying: Dark Green
    "Smart Energy": "var(--color-equans-turquoise)", // Main Corporate: Turquoise Green
    "Building Solutions": "var(--color-equans-azure-blue)", // Main Corporate: Azure Blue
    "Field Operations": "var(--color-equans-dark-blue)", // Accompanying: Dark Blue
  };
  return equansColors[businessUnit] || "#002439"; // Default to Dark Blue
}

// Helper function to get Equans-compliant background colors (20% opacity rule)
function getBusinessUnitBackgroundColor(businessUnit: string): string {
  const equansBackgroundColors: { [key: string]: string } = {
    "Digital Services": "rgba(0, 222, 232, 0.08)", // Light Blue with 20% opacity
    "IT Operations": "rgba(0, 129, 99, 0.08)", // Dark Green with 20% opacity
    "Smart Energy": "rgba(112, 189, 149, 0.08)", // Turquoise Green with 20% opacity
    "Building Solutions": "rgba(0, 89, 206, 0.08)", // Azure Blue with 20% opacity
    "Field Operations": "rgba(0, 36, 57, 0.05)", // Dark Blue with 5% opacity (to differentiate from default)
  };
  return equansBackgroundColors[businessUnit] || "rgba(0, 36, 57, 0.05)";
}
