import React, { useState } from 'react';
import { Building2, Users, Upload } from 'lucide-react';
import ImportData from '../components/Import/ImportData';

export function ImportPage() {
  const [type, setType] = useState<'organizations' | 'users'>('organizations');

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 p-8">
      <div className="max-w-4xl mx-auto">
        {/* Header Section */}
        <div className="mb-8">
          <div className="flex items-center gap-3 mb-2">
            <Upload className="w-8 h-8 text-ui-blue-primary" />
            <h1 className="text-4xl font-bold text-slate-900">Data Import</h1>
          </div>
          <p className="text-slate-600 text-lg">Import your organizations or users data from Excel or CSV files</p>
        </div>

        {/* Type Selector Cards */}
        <div className="grid grid-cols-2 gap-4 mb-8">
          <button
            onClick={() => setType('organizations')}
            className={`p-6 rounded-lg transition-all duration-200 ${
              type === 'organizations'
                ? 'bg-ui-blue-primary text-white ring-2 ring-ui-blue-primary ring-offset-2'
                : 'bg-white text-slate-700 border-2 border-slate-200 hover:border-ui-blue-primary hover:shadow-md'
            }`}
          >
            <div className="flex items-center gap-3 justify-center">
              <Building2 className="w-6 h-6" />
              <div className="text-left">
                <div className="font-semibold text-base">Organizations</div>
                <div className={`text-sm ${type === 'organizations' ? 'text-blue-100' : 'text-slate-500'}`}>
                  Import company data
                </div>
              </div>
            </div>
          </button>

          <button
            onClick={() => setType('users')}
            className={`p-6 rounded-lg transition-all duration-200 ${
              type === 'users'
                ? 'bg-ui-blue-primary text-white ring-2 ring-ui-blue-primary ring-offset-2'
                : 'bg-white text-slate-700 border-2 border-slate-200 hover:border-ui-blue-primary hover:shadow-md'
            }`}
          >
            <div className="flex items-center gap-3 justify-center">
              <Users className="w-6 h-6" />
              <div className="text-left">
                <div className="font-semibold text-base">Users</div>
                <div className={`text-sm ${type === 'users' ? 'text-blue-100' : 'text-slate-500'}`}>
                  Import user accounts
                </div>
              </div>
            </div>
          </button>
        </div>

        {/* Import Component */}
        <ImportData type={type} />
      </div>
    </div>
  );
}

export default ImportPage;
