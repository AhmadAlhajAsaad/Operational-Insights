// New: Organization/Business Unit data with Consultancy Costs & Forecast
export const organizations = [
  {
    id: "21959ca7-236b-11j7-k470",
    name: "SLS Digital Platform",
    businessUnit: "Digital Services",
    totalLicenses: 245,
    activeUsers: 210,
    licenseCost: 18450,
    consultancyCost: 8500,
    monthlyCost: 26950,
    chargebackAmount: 26950,
    forecast: {
      nextMonth: 27800,
      threeMonths: 29500,
      trend: "up" as const,
      percentageChange: 3.2
    },
    products: [
      { name: "Jira", licenses: 100, cost: 6500, activeUsers: 95 },
      { name: "Confluence", licenses: 80, cost: 5200, activeUsers: 75 },
      { name: "GitHub", licenses: 45, cost: 4800, activeUsers: 40 },
      { name: "Copilot", licenses: 20, cost: 1950, activeUsers: 15 }
    ],
    consultancyServices: [
      { type: "Development Support", hours: 32, hourlyRate: 150, cost: 4800 },
      { type: "Architecture Review", hours: 16, hourlyRate: 175, cost: 2800 },
      { type: "Training & Enablement", hours: 6, hourlyRate: 150, cost: 900 }
    ],
    costTrend: [
      { month: "Jan", licenseCost: 16000, consultancyCost: 6500, cost: 22500, users: 195 },
      { month: "Feb", licenseCost: 16800, consultancyCost: 7200, cost: 24000, users: 200 },
      { month: "Mar", licenseCost: 17200, consultancyCost: 7500, cost: 24700, users: 202 },
      { month: "Apr", licenseCost: 17800, consultancyCost: 8000, cost: 25800, users: 205 },
      { month: "May", licenseCost: 18100, consultancyCost: 8200, cost: 26300, users: 208 },
      { month: "Jun", licenseCost: 18450, consultancyCost: 8500, cost: 26950, users: 210 }
    ]
  },
  {
    id: "8f3d2a1b-789c-4de5-a123",
    name: "Infrastructure Team",
    businessUnit: "IT Operations",
    totalLicenses: 180,
    activeUsers: 165,
    licenseCost: 14200,
    consultancyCost: 5200,
    monthlyCost: 19400,
    chargebackAmount: 19400,
    forecast: {
      nextMonth: 19800,
      threeMonths: 20500,
      trend: "up" as const,
      percentageChange: 2.1
    },
    products: [
      { name: "Jira", licenses: 75, cost: 4900, activeUsers: 70 },
      { name: "Confluence", licenses: 60, cost: 3900, activeUsers: 55 },
      { name: "Trello", licenses: 30, cost: 3200, activeUsers: 28 },
      { name: "JForg", licenses: 15, cost: 2200, activeUsers: 12 }
    ],
    consultancyServices: [
      { type: "Infrastructure Consulting", hours: 24, hourlyRate: 160, cost: 3840 },
      { type: "Security Review", hours: 8, hourlyRate: 170, cost: 1360 }
    ],
    costTrend: [
      { month: "Jan", licenseCost: 13000, consultancyCost: 4200, cost: 17200, users: 155 },
      { month: "Feb", licenseCost: 13400, consultancyCost: 4500, cost: 17900, users: 158 },
      { month: "Mar", licenseCost: 13700, consultancyCost: 4700, cost: 18400, users: 160 },
      { month: "Apr", licenseCost: 14000, consultancyCost: 4900, cost: 18900, users: 162 },
      { month: "May", licenseCost: 14100, consultancyCost: 5000, cost: 19100, users: 164 },
      { month: "Jun", licenseCost: 14200, consultancyCost: 5200, cost: 19400, users: 165 }
    ]
  },
  {
    id: "c5a9b3f7-4e2d-8a1c-b456",
    name: "Energy Solutions",
    businessUnit: "Smart Energy",
    totalLicenses: 150,
    activeUsers: 135,
    licenseCost: 11800,
    consultancyCost: 3800,
    monthlyCost: 15600,
    chargebackAmount: 15600,
    forecast: {
      nextMonth: 16100,
      threeMonths: 17200,
      trend: "up" as const,
      percentageChange: 3.2
    },
    products: [
      { name: "Jira", licenses: 60, cost: 3900, activeUsers: 55 },
      { name: "Confluence", licenses: 50, cost: 3250, activeUsers: 45 },
      { name: "GitHub", licenses: 30, cost: 3200, activeUsers: 27 },
      { name: "Trello", licenses: 10, cost: 1450, activeUsers: 8 }
    ],
    consultancyServices: [
      { type: "Energy Platform Advisory", hours: 20, hourlyRate: 150, cost: 3000 },
      { type: "Data Integration Support", hours: 5, hourlyRate: 160, cost: 800 }
    ],
    costTrend: [
      { month: "Jan", licenseCost: 10500, consultancyCost: 2800, cost: 13300, users: 125 },
      { month: "Feb", licenseCost: 10800, consultancyCost: 3000, cost: 13800, users: 128 },
      { month: "Mar", licenseCost: 11200, consultancyCost: 3200, cost: 14400, users: 130 },
      { month: "Apr", licenseCost: 11400, consultancyCost: 3400, cost: 14800, users: 132 },
      { month: "May", licenseCost: 11600, consultancyCost: 3600, cost: 15200, users: 134 },
      { month: "Jun", licenseCost: 11800, consultancyCost: 3800, cost: 15600, users: 135 }
    ]
  },
  {
    id: "3b7e4d9a-12f5-6c8e-d789",
    name: "Smart Buildings",
    businessUnit: "Building Solutions",
    totalLicenses: 120,
    activeUsers: 98,
    licenseCost: 9400,
    consultancyCost: 2400,
    monthlyCost: 11800,
    chargebackAmount: 11800,
    forecast: {
      nextMonth: 12000,
      threeMonths: 12500,
      trend: "neutral" as const,
      percentageChange: 1.7
    },
    products: [
      { name: "Jira", licenses: 50, cost: 3250, activeUsers: 42 },
      { name: "Confluence", licenses: 40, cost: 2600, activeUsers: 35 },
      { name: "Trello", licenses: 25, cost: 2700, activeUsers: 18 },
      { name: "GitHub", licenses: 5, cost: 850, activeUsers: 3 }
    ],
    consultancyServices: [
      { type: "Building Systems Integration", hours: 12, hourlyRate: 160, cost: 1920 },
      { type: "Training", hours: 3, hourlyRate: 160, cost: 480 }
    ],
    costTrend: [
      { month: "Jan", licenseCost: 8800, consultancyCost: 1800, cost: 10600, users: 92 },
      { month: "Feb", licenseCost: 8900, consultancyCost: 1900, cost: 10800, users: 93 },
      { month: "Mar", licenseCost: 9100, consultancyCost: 2000, cost: 11100, users: 95 },
      { month: "Apr", licenseCost: 9200, consultancyCost: 2100, cost: 11300, users: 96 },
      { month: "May", licenseCost: 9300, consultancyCost: 2200, cost: 11500, users: 97 },
      { month: "Jun", licenseCost: 9400, consultancyCost: 2400, cost: 11800, users: 98 }
    ]
  },
  {
    id: "7d2f8c1a-5b3e-9a4d-e012",
    name: "Maintenance Services",
    businessUnit: "Field Operations",
    totalLicenses: 80,
    activeUsers: 72,
    licenseCost: 6150,
    consultancyCost: 1500,
    monthlyCost: 7650,
    chargebackAmount: 7650,
    forecast: {
      nextMonth: 7800,
      threeMonths: 8100,
      trend: "neutral" as const,
      percentageChange: 2.0
    },
    products: [
      { name: "Jira", licenses: 35, cost: 2275, activeUsers: 32 },
      { name: "Confluence", licenses: 30, cost: 1950, activeUsers: 27 },
      { name: "Trello", licenses: 15, cost: 1925, activeUsers: 13 }
    ],
    consultancyServices: [
      { type: "Field Service Enablement", hours: 8, hourlyRate: 150, cost: 1200 },
      { type: "Process Optimization", hours: 2, hourlyRate: 150, cost: 300 }
    ],
    costTrend: [
      { month: "Jan", licenseCost: 5600, consultancyCost: 1000, cost: 6600, users: 68 },
      { month: "Feb", licenseCost: 5800, consultancyCost: 1100, cost: 6900, users: 69 },
      { month: "Mar", licenseCost: 5900, consultancyCost: 1200, cost: 7100, users: 70 },
      { month: "Apr", licenseCost: 6000, consultancyCost: 1300, cost: 7300, users: 71 },
      { month: "May", licenseCost: 6050, consultancyCost: 1400, cost: 7450, users: 71 },
      { month: "Jun", licenseCost: 6150, consultancyCost: 1500, cost: 7650, users: 72 }
    ]
  }
];

// Aggregate summary data for dashboard
export const dashboardSummary = {
  totalLicenseCost: organizations.reduce((sum, org) => sum + org.licenseCost, 0),
  totalConsultancyCost: organizations.reduce((sum, org) => sum + org.consultancyCost, 0),
  totalMonthlyCost: organizations.reduce((sum, org) => sum + org.monthlyCost, 0),
  totalChargeback: organizations.reduce((sum, org) => sum + org.chargebackAmount, 0),
  forecastNextMonth: organizations.reduce((sum, org) => sum + org.forecast.nextMonth, 0),
  forecastThreeMonths: organizations.reduce((sum, org) => sum + org.forecast.threeMonths, 0)
};
