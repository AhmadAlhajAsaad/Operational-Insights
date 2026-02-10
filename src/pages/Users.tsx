import React, { useMemo, useState } from 'react';
import { StatCard } from '../components/ui/StatCard';
import { Card } from '../components/ui/card';
import { Table } from '../components/ui/table';
import { SimpleSelect } from '../components/ui/select';
import { Users as UsersIcon, TrendingUp, DollarSign, AlertTriangle, Search, Download } from 'lucide-react';

interface UsersPageProps {
  onNavigateToUser: (userId: string) => void;
}

const usersData = [
  { id: 'USR-001', name: 'John Smith', email: 'john.smith@equans.com', department: 'Engineering', licenses: 5, monthlyCost: 245, status: 'Active' },
  { id: 'USR-002', name: 'Sarah Johnson', email: 'sarah.johnson@equans.com', department: 'Marketing', licenses: 3, monthlyCost: 156, status: 'Active' },
  { id: 'USR-003', name: 'Mike Brown', email: 'mike.brown@equans.com', department: 'Sales', licenses: 4, monthlyCost: 198, status: 'Active' },
  { id: 'USR-004', name: 'Emily Davis', email: 'emily.davis@equans.com', department: 'HR', licenses: 2, monthlyCost: 89, status: 'Inactive' },
  { id: 'USR-005', name: 'David Wilson', email: 'david.wilson@equans.com', department: 'Finance', licenses: 3, monthlyCost: 167, status: 'Active' },
  { id: 'USR-006', name: 'Lisa Anderson', email: 'lisa.anderson@equans.com', department: 'Engineering', licenses: 6, monthlyCost: 312, status: 'Active' },
  { id: 'USR-007', name: 'Robert Taylor', email: 'robert.taylor@equans.com', department: 'Operations', licenses: 2, monthlyCost: 94, status: 'Inactive' },
  { id: 'USR-008', name: 'Jennifer Martinez', email: 'jennifer.martinez@equans.com', department: 'Legal', licenses: 4, monthlyCost: 201, status: 'Active' },
];

const statusOptions = [
  { value: 'all', label: 'All Status' },
  { value: 'Active', label: 'Active' },
  { value: 'Inactive', label: 'Inactive' },
];

export function Users({ onNavigateToUser }: UsersPageProps) {
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [departmentFilter, setDepartmentFilter] = useState('all');

  const departmentOptions = useMemo(() => {
    const departments = Array.from(new Set(usersData.map((user) => user.department)));
    return [{ value: 'all', label: 'All Categories' }, ...departments.map((dept) => ({ value: dept, label: dept }))];
  }, []);

  const filteredUsers = usersData.filter((user) => {
    const matchesSearch = [
      user.name,
      user.email,
      user.department,
      user.id,
    ].some((field) => field.toLowerCase().includes(searchTerm.toLowerCase()));

    const matchesStatus = statusFilter === 'all' || user.status === statusFilter;
    const matchesDepartment = departmentFilter === 'all' || user.department === departmentFilter;
    return matchesSearch && matchesStatus && matchesDepartment;
  });

  const totalUsers = usersData.length;
  const activeUsers = usersData.filter((u) => u.status === 'Active').length;
  const totalCost = usersData.reduce((sum, u) => sum + u.monthlyCost, 0);
  const avgCostPerUser = Math.round(totalCost / totalUsers);

  const tableColumns = [
    { key: 'id', label: 'User ID', align: 'left' as const },
    { key: 'name', label: 'Name' },
    { key: 'email', label: 'Email' },
    { key: 'department', label: 'Category' },
    { key: 'licenses', label: 'Licenses', align: 'center' as const },
    { key: 'monthlyCost', label: 'Monthly Cost', align: 'right' as const },
    { key: 'status', label: 'Status', align: 'center' as const },
  ];

  const tableData = filteredUsers.map((user) => ({
    ...user,
    name: (
      <span className="inline-flex items-center font-semibold text-[#0059CE]">
        {user.name}
      </span>
    ),
    monthlyCost: `€${user.monthlyCost}`,
    status: (
      <span className={`inline-flex px-2.5 py-1 rounded-full text-xs font-semibold ${
        user.status === 'Active'
          ? 'bg-[#E9FDF2] text-[#059669]'
          : 'bg-[#FCECED] text-[#dc2626]'
      }`}>
        {user.status}
      </span>
    )
  }));

  const handleExport = () => {
    const headers = [
      'User ID',
      'Name',
      'Email',
      'Category',
      'Licenses',
      'Monthly Cost (€)',
      'Status',
    ];
    const rows = filteredUsers.map((user) => [
      user.id,
      user.name,
      user.email,
      user.department,
      user.licenses.toString(),
      user.monthlyCost.toString(),
      user.status,
    ]);
    const csvContent = [headers, ...rows]
      .map((row) => row.map((value) => `"${String(value).replace(/"/g, '""')}"`).join(','))
      .join('\n');
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = 'users-export.csv';
    link.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="space-y-6">
      <Card
        className="border border-[#D6E6F5] bg-gradient-to-r from-[#EAF1F9] via-white to-[#EDFCF5]"
        style={{ borderRadius: '20px' }}
      >
        <div className="p-6 flex flex-col gap-2">
          <span className="text-sm font-semibold text-[#008163] uppercase tracking-wide">User Management Hub</span>
          <h2 className="text-2xl font-bold text-neutral-900">Manage users with colorful insights</h2>
          <p className="text-neutral-600 max-w-2xl">
            Filter by status or category, explore license trends, and export your data directly to Excel.
          </p>
        </div>
      </Card>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard
          icon={<UsersIcon className="w-5 h-5" />}
          label="Total Users"
          value={totalUsers.toString()}
          variant="blue"
        />
        <StatCard
          icon={<TrendingUp className="w-5 h-5" />}
          label="Active Users"
          value={activeUsers.toString()}
          change={{ value: `${Math.round((activeUsers / totalUsers) * 100)}% active`, trend: 'up' }}
          variant="green"
        />
        <StatCard
          icon={<DollarSign className="w-5 h-5" />}
          label="Total Monthly Cost"
          value={`€${totalCost.toLocaleString()}`}
          variant="purple"
        />
        <StatCard
          icon={<AlertTriangle className="w-5 h-5" />}
          label="Avg Cost per User"
          value={`€${avgCostPerUser}`}
          variant="mint"
        />
      </div>

      <Card className="bg-[#F1F8FE] border-[#D6E6F5]" style={{ borderRadius: '18px' }}>
        <div className="p-6 space-y-4">
          <div className="flex flex-col gap-4">
            <div>
              <h3 className="text-lg font-bold text-neutral-900 mb-1">User Directory</h3>
              <p className="text-sm text-neutral-500">Click a user to view more details.</p>
            </div>
            <div className="flex justify-center">
              <button
                type="button"
                onClick={handleExport}
                className="inline-flex items-center justify-center gap-2 rounded-full px-8 py-2.5 text-sm font-semibold shadow-sm transition focus:outline-none focus:ring-2 focus:ring-[#70BD95]"
                style={{ backgroundColor: '#008163', color: '#FFFFFF', border: '1px solid #008163' }}
              >
                <Download className="h-4 w-4" />
                Export to Excel
              </button>
            </div>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
            <div className="lg:col-span-1">
              <div className="relative">
                <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-neutral-400" />
                <input
                  type="text"
                  placeholder="Search by name, email, or ID..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="w-full pl-11 pr-4 py-3 bg-white border border-[#D6E6F5] rounded-full focus:outline-none focus:ring-2 focus:ring-[#0059CE] transition-all"
                />
              </div>
              {searchTerm && (
                <p className="text-xs text-[#0059CE] mt-2 font-medium">
                  Showing {filteredUsers.length} of {usersData.length} users
                </p>
              )}
            </div>
            <SimpleSelect
              label="Status Filter"
              options={statusOptions}
              value={statusFilter}
              onChange={setStatusFilter}
              className="bg-white p-4 rounded-2xl border border-[#D6E6F5]"
            />
            <SimpleSelect
              label="Category Filter"
              options={departmentOptions}
              value={departmentFilter}
              onChange={setDepartmentFilter}
              className="bg-white p-4 rounded-2xl border border-[#D6E6F5]"
            />
          </div>
        </div>
      </Card>

      <Card className="bg-white shadow-sm border-[#EAF1F9]" style={{ borderRadius: '16px' }}>
        <div className="p-6">
          <Table
            columns={tableColumns}
            data={tableData}
            onRowClick={(row) => onNavigateToUser(row.id)}
          />
        </div>
      </Card>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card className="bg-[#F9F1FE] shadow-sm border-[#EAF1F9]" style={{ borderRadius: '16px' }}>
          <div className="p-6">
            <h3 className="text-lg font-bold text-neutral-900 mb-4">Users by Category</h3>
            <div className="space-y-3">
              {departmentOptions
                .filter((option) => option.value !== 'all')
                .map((dept, index) => {
                  const deptUsers = usersData.filter((u) => u.department === dept.value);
                  const colors = ['#0059CE', '#76C512', '#FF9600', '#C865FF', '#00DEE8'];
                  return (
                    <div key={dept.value} className="flex items-center justify-between bg-white p-3 rounded-xl">
                      <div className="flex items-center gap-3">
                        <div
                          className="w-3 h-3 rounded-full"
                          style={{ backgroundColor: colors[index % colors.length] }}
                        />
                        <span className="font-medium text-neutral-900">{dept.label}</span>
                      </div>
                      <div className="flex items-center gap-4">
                        <span className="text-sm text-neutral-500">{deptUsers.length} users</span>
                        <span className="font-bold text-[#0059CE]">
                          €{deptUsers.reduce((sum, u) => sum + u.monthlyCost, 0)}
                        </span>
                      </div>
                    </div>
                  );
                })}
            </div>
          </div>
        </Card>

        <Card className="bg-[#EDFCF5] shadow-sm border-[#E9FDF2]" style={{ borderRadius: '16px' }}>
          <div className="p-6">
            <h3 className="text-lg font-bold text-neutral-900 mb-4">Top License Holders</h3>
            <div className="space-y-3">
              {[...usersData]
                .sort((a, b) => b.licenses - a.licenses)
                .slice(0, 5)
                .map((user, index) => (
                  <div key={user.id} className="flex items-center justify-between bg-white p-3 rounded-xl">
                    <div className="flex items-center gap-3">
                      <div className="w-8 h-8 bg-[#008163] text-white rounded-lg flex items-center justify-center font-bold text-sm">
                        {index + 1}
                      </div>
                      <div>
                        <div className="font-semibold text-neutral-900">{user.name}</div>
                        <div className="text-xs text-neutral-500">{user.department}</div>
                      </div>
                    </div>
                    <div className="text-right">
                      <div className="font-bold text-[#008163]">{user.licenses} licenses</div>
                      <div className="text-xs text-neutral-500">€{user.monthlyCost}/mo</div>
                    </div>
                  </div>
                ))}
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}
