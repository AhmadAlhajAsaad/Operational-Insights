// Mock data for Equans Operational Insights

export const licenseData = [
  { name: "Jira", total: 1150, active: 1120, unused: 30 },
  { name: "Confluence", total: 1150, active: 1095, unused: 55 },
  { name: "Trello", total: 375, active: 150, unused: 25 },
  { name: "GitHub", total: 200, active: 180, unused: 20 },
  { name: "Copilot", total: 100, active: 65, unused: 35 },
  { name: "JForg", total: 10, active: 6, unused: 4 },
  
];

export const trendData = [
  { month: "Jan", cost: 45000, users: 420 },
  { month: "Feb", cost: 47000, users: 445 },
  { month: "Mar", cost: 48500, users: 465 },
  { month: "Apr", cost: 50000, users: 480 },
  { month: "May", cost: 51500, users: 495 },
  { month: "Jun", cost: 53000, users: 510 },
];

export const usageDistribution = [
  { name: "Jira", value: 120 },
  { name: "Confluence", value: 95 },
  { name: "Trello", value: 150 },
  { name: "GitHub", value: 180 },
  { name: "Copilot", value: 65 },
  { name: "JForg", value: 6 },
];

export const topCustomers = [
  {
    customer: "SLS Digital Platform",
    licenses: 245,
    activeUsers: 210,
    cost: "€18,450",
  },
  {
    customer: "Infrastructure Team",
    licenses: 180,
    activeUsers: 165,
    cost: "€14,200",
  },
  {
    customer: "Energy Solutions",
    licenses: 150,
    activeUsers: 135,
    cost: "€11,800",
  },
  {
    customer: "Smart Buildings",
    licenses: 120,
    activeUsers: 98,
    cost: "€9,400",
  },
  {
    customer: "Maintenance Services",
    licenses: 80,
    activeUsers: 72,
    cost: "€6,150",
  },
];

export const products = [
  {
    id: "jira",
    name: "Jira",
    category: "Project Management",
    totalLicenses: 1150,
    activeLicenses: 1120,
    unusedLicenses: 30,
    monthlyCost: 12500,
    utilizationRate: 80,
  },
  {
    id: "confluence",
    name: "Confluence",
    category: "Documentation",
    totalLicenses: 1150,
    activeLicenses: 1095,
    unusedLicenses: 55,
    monthlyCost: 9800,
    utilizationRate: 63,
  },
  {
    id: "Trello",
    name: "Trello",
    category: "DevOps",
    totalLicenses: 375,
    activeLicenses: 350,
    unusedLicenses: 25,
    monthlyCost: 8600,
    utilizationRate: 86,
  },
  {
    id: "github",
    name: "GitHub",
    category: "Version Control",
    totalLicenses: 200,
    activeLicenses: 180,
    unusedLicenses: 20,
    monthlyCost: 15600,
    utilizationRate: 90,
  },
  {
    id: "copilot",
    name: "GitHub Copilot",
    category: "AI Development",
    totalLicenses: 100,
    activeLicenses: 65,
    unusedLicenses: 35,
    monthlyCost: 6500,
    utilizationRate: 65,
  },
  {
    id: "JForg",
    name: "JForg Artifactory",
    category: "AI Development",
    totalLicenses: 10,
    activeLicenses: 6,
    unusedLicenses: 4,
    monthlyCost: 650,
    utilizationRate: 60,
  },
  
];

export const users = [
  {
    id: 1,
    name: "Jan Vermeulen",
    email: "jan.vermeulen@equans.com",
    department: "Engineering",
    licenses: ["Jira", "GitHub", "Copilot", "Trello"],
    lastActive: "2024-12-07",
    status: "active",
  },
  {
    id: 2,
    name: "Sophie De Vries",
    email: "sophie.devries@equans.com",
    department: "Product Management",
    licenses: ["Jira", "Confluence", "GitHub"],
    lastActive: "2024-12-06",
    status: "active",
  },
  {
    id: 3,
    name: "Michael Peeters",
    email: "michael.peeters@equans.com",
    department: "Engineering",
    licenses: ["GitHub", "Copilot", "Trello"],
    lastActive: "2024-12-07",
    status: "active",
  },
  {
    id: 4,
    name: "Emma Janssen",
    email: "emma.janssen@equans.com",
    department: "Design",
    licenses: ["Jira", "Confluence"],
    lastActive: "2024-11-28",
    status: "inactive",
  },
  {
    id: 5,
    name: "Lucas Martens",
    email: "lucas.martens@equans.com",
    department: "Engineering",
    licenses: ["Jira", "GitHub", "Copilot", "Trello"],
    lastActive: "2024-12-05",
    status: "active",
  },
];

export const userActivityData = [
  { date: "2024-11-07", commits: 12, reviews: 8, issues: 5 },
  { date: "2024-11-14", commits: 15, reviews: 10, issues: 7 },
  { date: "2024-11-21", commits: 18, reviews: 12, issues: 6 },
  { date: "2024-11-28", commits: 20, reviews: 14, issues: 8 },
  { date: "2024-12-05", commits: 16, reviews: 11, issues: 9 },
];