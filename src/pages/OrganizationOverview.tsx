import React, { useState } from "react";
import { StatCard } from "../components/ui/StatCard";
import { Card } from "../components/ui/card";
import { CylindricalMonthlyChart } from "../components/charts/CylindricalMonthlyChart";
import {
  Building2,
  DollarSign,
  Users,
  Search,
  EuroIcon,
  Briefcase,
  TrendingUp,
} from "lucide-react";
import { organizations } from "../data/organizationData";

interface OrganizationOverviewProps {
  onNavigateToOrganization: (orgId: string) => void;
}

export function OrganizationOverview({
  onNavigateToOrganization,
}: OrganizationOverviewProps) {
  const [searchTerm, setSearchTerm] = useState("");

  // Calculate aggregate metrics with consultancy
  const totalLicenseCost = organizations.reduce(
    (sum, org) => sum + org.licenseCost,
    0,
  );
  const totalConsultancyCost = organizations.reduce(
    (sum, org) => sum + org.consultancyCost,
    0,
  );
  const totalCost = organizations.reduce(
    (sum, org) => sum + org.monthlyCost,
    0,
  );
  const totalChargeback = organizations.reduce(
    (sum, org) => sum + org.chargebackAmount,
    0,
  );
  const totalForecast = organizations.reduce(
    (sum, org) => sum + org.forecast.nextMonth,
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
    { key: "licenseCost", label: "License Cost", align: "right" as const },
    { key: "consultancyCost", label: "Consultancy", align: "right" as const },
    { key: "monthlyCost", label: "Total Cost", align: "right" as const },
    { key: "chargebackAmount", label: "Chargeback", align: "right" as const },
  ];

  // Filter organizations based on search term
  const filteredOrganizations = organizations.filter(
    (org) =>
      org.id.toLowerCase().includes(searchTerm.toLowerCase()) ||
      org.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      org.businessUnit.toLowerCase().includes(searchTerm.toLowerCase()),
  );

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

      {/* KPI Cards - Complete Cost Overview with Consultancy */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-6 gap-4">
        <StatCard
          icon={<DollarSign className="w-5 h-5" strokeWidth={1.8} />}
          label="License Costs"
          value={`EUR ${totalLicenseCost.toLocaleString()}`}
          change={{ value: "+3.8% vs last month", trend: "up" }}
          variant="blue"
        />

        <StatCard
          icon={<Briefcase className="w-5 h-5" strokeWidth={1.8} />}
          label="Consultancy Costs"
          value={`EUR ${totalConsultancyCost.toLocaleString()}`}
          change={{ value: "+5.2% vs last month", trend: "up" }}
          variant="purple"
        />

        <StatCard
          icon={<DollarSign className="w-5 h-5" strokeWidth={1.8} />}
          label="Total Monthly Cost"
          value={`EUR ${totalCost.toLocaleString()}`}
          change={{ value: "+4.1% vs last month", trend: "up" }}
          variant="cyan"
        />

        <StatCard
          icon={<EuroIcon className="w-5 h-5" strokeWidth={1.8} />}
          label="Chargeback Amount"
          value={`EUR ${totalChargeback.toLocaleString()}`}
          change={{ value: "Complete", trend: "neutral" }}
          variant="green"
        />

        <StatCard
          icon={<TrendingUp className="w-5 h-5" strokeWidth={1.8} />}
          label="Forecast (Next Month)"
          value={`EUR ${totalForecast.toLocaleString()}`}
          change={{ value: "+2.8% projected", trend: "up" }}
          variant="yellow"
        />

        <StatCard
          icon={<Users className="w-5 h-5" strokeWidth={1.8} />}
          label="Active Users"
          value={totalActiveUsers.toString()}
          change={{ value: `${organizations.length} orgs`, trend: "neutral" }}
          variant="mint"
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

      {/* Organizations Table with Consultancy Column */}
      <Card
        className="shadow-sm border chart-container"
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
                placeholder="Search by Organization ID (e.g., ORG03xx)..."
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
          <div className="relative w-full overflow-x-auto rounded-xl border border-[#EAF1F9]">
            <table className="w-full caption-bottom text-sm">
              <thead
                className="border-b border-[#EAF1F9]"
                style={{ backgroundColor: "var(--color-equans-dark-blue)" }}
              >
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
                  const bgColor = index % 2 === 0 ? "#ffffff" : "#F4F9FF";

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
                      <td className="p-4 align-middle whitespace-nowrap text-right">
                        <span className="font-medium text-neutral-700">
                          EUR {org.licenseCost.toLocaleString()}
                        </span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-right">
                        <span
                          className="font-medium"
                          style={{ color: "#C865FF" }}
                        >
                          EUR {org.consultancyCost.toLocaleString()}
                        </span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-right">
                        <span className="font-bold text-neutral-900">
                          EUR {org.monthlyCost.toLocaleString()}
                        </span>
                      </td>
                      <td className="p-4 align-middle whitespace-nowrap text-right">
                        <span
                          className="font-bold"
                          style={{ color: "var(--color-equans-dark-green)" }}
                        >
                          EUR {org.chargebackAmount.toLocaleString()}
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
      <Card
        className="shadow-sm"
        style={{
          borderRadius: "16px",
          backgroundColor: "#e1f2ea",
          borderColor: "var(--color-secondary-20)",
        }}
      >
        <div className="p-6">
          <h3
            className="text-lg font-bold mb-4"
            style={{
              color: "var(--color-text-primary)",
              fontFamily: "Roboto, sans-serif",
            }}
          >
            Top Cost Drivers
          </h3>
          <div className="space-y-3">
            {[...organizations]
              .sort((a, b) => b.monthlyCost - a.monthlyCost)
              .slice(0, 5)
              .map((org, index) => (
                <div
                  key={org.id}
                  className="flex items-center justify-between p-3 rounded-xl"
                  style={{ backgroundColor: "var(--color-equans-white)" }}
                >
                  <div className="flex items-center gap-3">
                    <div
                      className="w-8 h-8 rounded-full flex items-center justify-center font-bold text-sm"
                      style={{
                        backgroundColor: "var(--color-equans-dark-green)",
                        color: "var(--color-equans-white)",
                      }}
                    >
                      {index + 1}
                    </div>
                    <div>
                      <div
                        className="font-semibold"
                        style={{ color: "var(--color-text-primary)" }}
                      >
                        {org.name}
                      </div>
                      <div
                        className="text-xs"
                        style={{ color: "var(--color-text-secondary)" }}
                      >
                        {org.businessUnit}
                      </div>
                    </div>
                  </div>
                  <div className="flex items-center gap-6">
                    <div className="text-right">
                      <div className="text-xs text-neutral-500">Licenses</div>
                      <div className="font-medium text-neutral-700">
                        EUR {org.licenseCost.toLocaleString()}
                      </div>
                    </div>
                    <div className="text-right">
                      <div className="text-xs text-neutral-500">
                        Consultancy
                      </div>
                      <div className="font-medium" style={{ color: "#C865FF" }}>
                        EUR {org.consultancyCost.toLocaleString()}
                      </div>
                    </div>
                    <div className="text-right pl-4 border-l border-neutral-200">
                      <div
                        className="font-bold"
                        style={{ color: "var(--color-equans-dark-green)" }}
                      >
                        EUR {org.monthlyCost.toLocaleString()}
                      </div>
                      <div
                        className="text-xs"
                        style={{ color: "var(--color-text-secondary)" }}
                      >
                        {Math.round((org.monthlyCost / totalCost) * 100)}% of
                        total
                      </div>
                    </div>
                  </div>
                </div>
              ))}
          </div>
        </div>
      </Card>
    </div>
  );
}

function getBusinessUnitColor(businessUnit: string): string {
  const equansColors: { [key: string]: string } = {
    "Digital Services": "var(--color-equans-apple-green)",
    "Smart Energy": "var(--color-equans-turquoise)",
    "IT Operations": "var(--color-equans-dark-green)",
    "Building Solutions": "var(--color-equans-azure-blue)",
    "Field Operations": "var(--color-equans-dark-blue)",
  };
  return equansColors[businessUnit] || "var(--color-equans-dark-blue)";
}

function getBusinessUnitBackgroundColor(businessUnit: string): string {
  const equansBackgroundColors: { [key: string]: string } = {
    "Digital Services": "var(--equans-lightblue-20)",
    "Smart Energy": "var(--color-accent-20)",
    "IT Operations": "var(--equans-green-20)",
    "Building Solutions": "var(--equans-blue-20)",
    "Field Operations": "var(--color-primary-20)",
  };
  return equansBackgroundColors[businessUnit] || "var(--color-primary-20)";
}
