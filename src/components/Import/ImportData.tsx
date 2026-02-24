import React, { useState } from 'react';
import * as XLSX from 'xlsx';

interface ImportDataProps {
  type: 'organizations' | 'users';
}

export function ImportData({ type }: ImportDataProps) {
  const [previewRows, setPreviewRows] = useState<Record<string, any>[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const requiredColumns = type === 'organizations'
    ? ['id', 'name', 'domain']
    : ['id', 'firstName', 'lastName', 'email', 'organizationId'];

  const handleFile = async (file?: File) => {
    setError(null);
    if (!file) return;

    try {
      const data = await file.arrayBuffer();
      const workbook = XLSX.read(data, { type: 'array' });
      const firstSheetName = workbook.SheetNames[0];
      const worksheet = workbook.Sheets[firstSheetName];
      const json = XLSX.utils.sheet_to_json<Record<string, any>>(worksheet, { defval: '' });

      if (!Array.isArray(json) || json.length === 0) {
        setError('Geen rijen gevonden in het bestand.');
        setPreviewRows([]);
        return;
      }

      // normalize keys: trim + camelCase-ish (simple)
      const normalized = json.map((row) => {
        const out: Record<string, any> = {};
        Object.keys(row).forEach((k) => {
          const key = String(k).trim();
          out[key] = (row as any)[k];
        });
        return out;
      });

      // check required columns
      const headerKeys = Object.keys(normalized[0]).map((k) => k.trim());
      const missing = requiredColumns.filter((c) => !headerKeys.includes(c));
      if (missing.length) {
        setError(`Ontbrekende kolommen: ${missing.join(', ')}`);
        setPreviewRows(normalized.slice(0, 10));
        return;
      }

      setPreviewRows(normalized.slice(0, 100));
    } catch (e: any) {
      setError(e?.message || String(e));
      setPreviewRows([]);
    }
  };

  const handleInputChange: React.ChangeEventHandler<HTMLInputElement> = (e) => {
    const f = e.target.files?.[0];
    handleFile(f);
  };

  const handleSubmit = async () => {
    setLoading(true);
    setError(null);
    try {
      // In this repo there may be no API; attempt to POST to /api/import
      const payload = { type, rows: previewRows };
      const res = await fetch('/api/import', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });
      if (!res.ok) {
        const txt = await res.text();
        throw new Error(txt || `Request failed: ${res.status}`);
      }
      alert('Import succesvol gestart.');
    } catch (e: any) {
      setError(e?.message || String(e));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-4 bg-card rounded-md">
      <div className="mb-4">
        <label className="block text-sm font-medium mb-2">Bestand (CSV / XLSX)</label>
        <input type="file" accept=".csv, application/vnd.ms-excel, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" onChange={handleInputChange} />
      </div>

      <div className="mb-4">
        <p className="text-sm">Verwachte kolommen: {requiredColumns.join(', ')}</p>
      </div>

      {error && (
        <div className="mb-4 text-sm text-destructive">Fout: {error}</div>
      )}

      {previewRows.length > 0 && (
        <div className="mb-4">
          <p className="text-sm mb-2">Voorbeeld van maximaal 100 rijen</p>
          <div className="overflow-auto border rounded">
            <table className="w-full text-sm">
              <thead>
                <tr>
                  {Object.keys(previewRows[0]).map((h) => (
                    <th key={h} className="px-2 py-1 text-left border-b">{h}</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {previewRows.map((r, i) => (
                  <tr key={i} className={i % 2 === 0 ? 'bg-muted' : ''}>
                    {Object.keys(previewRows[0]).map((k) => (
                      <td key={k} className="px-2 py-1 align-top border-b">{String(r[k] ?? '')}</td>
                    ))}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      <div className="flex gap-2">
        <button onClick={handleSubmit} disabled={previewRows.length === 0 || loading} className="btn btn-primary">
          {loading ? 'Verzenden…' : 'Verstuur naar API'}
        </button>
        <button onClick={() => { setPreviewRows([]); setError(null); }} className="btn">
          Reset
        </button>
      </div>
    </div>
  );
}

export default ImportData;
