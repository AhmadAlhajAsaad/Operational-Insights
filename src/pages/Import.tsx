import React, { useState } from 'react';
import ImportData from '../components/Import/ImportData';

export function ImportPage() {
  const [type, setType] = useState<'organizations' | 'users'>('organizations');

  return (
    <div>
      <div className="mb-4 flex items-center gap-4">
        <h2 className="text-lg font-semibold">Data Import</h2>
        <div className="ml-2">
          <label className="mr-2">Type:</label>
          <select value={type} onChange={(e) => setType(e.target.value as any)} className="px-2 py-1 border rounded">
            <option value="organizations">Organizations</option>
            <option value="users">Users</option>
          </select>
        </div>
      </div>

      <ImportData type={type} />
    </div>
  );
}

export default ImportPage;
