import React, { useState } from 'react';
import * as XLSX from 'xlsx';
import { AlertCircle, CheckCircle, Upload, Eye, RotateCcw } from 'lucide-react';

interface ImportDataProps {
  type: 'organizations' | 'users';
}

export function ImportData({ type }: ImportDataProps) {
  const [previewRows, setPreviewRows] = useState<Record<string, any>[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [success, setSuccess] = useState(false);
  const [fileName, setFileName] = useState<string | null>(null);

  const requiredColumns = type === 'organizations'
    ? ['id', 'name', 'domain']
    : ['id', 'firstName', 'lastName', 'email', 'organizationId'];

  const handleFile = async (file?: File) => {
    setError(null);
    setSuccess(false);
    if (!file) return;

    try {
      setFileName(file.name);
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
      setSuccess(true);
      setPreviewRows([]);
      setFileName(null);
    } catch (e: any) {
      setError(e?.message || String(e));
    } finally {
      setLoading(false);
    }
  };

  const handleReset = () => {
    setPreviewRows([]);
    setError(null);
    setSuccess(false);
    setFileName(null);
  };

  return (
    <div className="bg-white rounded-xl shadow-lg overflow-hidden">
      {/* File Upload Section */}
      <div className="p-8 bg-gradient-to-r from-ui-blue-lighter to-slate-50 border-b border-slate-200">
        <div className="mb-6">
          <h2 className="text-xl font-semibold text-slate-900 mb-2">Upload File</h2>
          <p className="text-slate-600">Drag and drop or click to select</p>
        </div>

        <label className="block">
          <div className="relative group">
            <input
              type="file"
              accept=".csv,application/vnd.ms-excel,application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
              onChange={handleInputChange}
              className="sr-only"
            />
            <div className="border-2 border-dashed border-ui-blue-primary rounded-lg p-8 text-center hover:bg-ui-blue-light transition-colors cursor-pointer">
              <Upload className="w-10 h-10 text-ui-blue-primary mx-auto mb-3" />
              <p className="text-sm font-medium text-slate-900">Select a file or drag and drop</p>
              <p className="text-xs text-slate-500 mt-1">CSV or Excel (XLSX) files are supported</p>
              {fileName && <p className="text-sm text-ui-blue-primary font-semibold mt-3">{fileName}</p>}
            </div>
          </div>
        </label>

        <div className="mt-6 p-4 bg-slate-50 rounded-lg border border-slate-200">
          <p className="text-sm font-medium text-slate-700 mb-2">Required columns:</p>
          <div className="flex flex-wrap gap-2">
            {requiredColumns.map((col) => (
              <span key={col} className="px-3 py-1 bg-ui-blue-primary text-white text-xs font-medium rounded-full">
                {col}
              </span>
            ))}
          </div>
        </div>
      </div>

      {/* Error Alert */}
      {error && (
        <div className="p-6 bg-ui-red-light border-l-4 border-ui-red-DEFAULT">
          <div className="flex items-start gap-4">
            <AlertCircle className="w-6 h-6 text-ui-red-DEFAULT flex-shrink-0 mt-0.5" />
            <div>
              <h3 className="font-semibold text-slate-900">Error</h3>
              <p className="text-sm text-slate-700 mt-1">{error}</p>
            </div>
          </div>
        </div>
      )}

      {/* Success Alert */}
      {success && (
        <div className="p-6 bg-ui-green-light border-l-4 border-ui-green-DEFAULT">
          <div className="flex items-start gap-4">
            <CheckCircle className="w-6 h-6 text-ui-green-DEFAULT flex-shrink-0 mt-0.5" />
            <div>
              <h3 className="font-semibold text-slate-900">Success!</h3>
              <p className="text-sm text-slate-700 mt-1">Your data has been imported successfully.</p>
            </div>
          </div>
        </div>
      )}

      {/* Preview Section */}
      {previewRows.length > 0 && (
        <div className="p-8 border-t border-slate-200">
          <div className="flex items-center gap-2 mb-4">
            <Eye className="w-5 h-5 text-slate-700" />
            <h3 className="text-lg font-semibold text-slate-900">Preview</h3>
            <span className="text-sm text-slate-600">(showing up to 100 rows)</span>
          </div>

          <div className="overflow-x-auto rounded-lg border border-slate-200">
            <table className="w-full text-sm">
              <thead>
                <tr className="bg-slate-100 border-b border-slate-200">
                  {Object.keys(previewRows[0]).map((h) => (
                    <th key={h} className="px-4 py-3 text-left font-semibold text-slate-900">
                      {h}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {previewRows.map((r, i) => (
                  <tr key={i} className={`${i % 2 === 0 ? 'bg-white' : 'bg-slate-50'} border-b border-slate-200 hover:bg-ui-blue-lighter transition-colors`}>
                    {Object.keys(previewRows[0]).map((k) => (
                      <td key={k} className="px-4 py-3 text-slate-700">
                        {String(r[k] ?? '')}
                      </td>
                    ))}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>

          <p className="text-xs text-slate-500 mt-3">
            Total rows to import: <span className="font-semibold text-slate-700">{previewRows.length}</span>
          </p>
        </div>
      )}

      {/* Action Buttons */}
      <div className="p-8 bg-slate-50 border-t border-slate-200 flex gap-3">
        <button
          onClick={handleSubmit}
          disabled={previewRows.length === 0 || loading}
          className={`flex items-center gap-2 px-6 py-3 rounded-lg font-semibold transition-all duration-200 ${
            previewRows.length === 0 || loading
              ? 'bg-slate-300 text-slate-500 cursor-not-allowed'
              : 'bg-ui-blue-primary text-white hover:shadow-lg hover:scale-105'
          }`}
        >
          <Upload className="w-5 h-5" />
          {loading ? 'Uploading...' : previewRows.length > 0 ? 'Import Data' : 'Select a file first'}
        </button>

        <button
          onClick={handleReset}
          className="flex items-center gap-2 px-6 py-3 rounded-lg font-semibold text-slate-700 bg-white border-2 border-slate-300 hover:border-slate-400 hover:shadow-md transition-all duration-200"
        >
          <RotateCcw className="w-5 h-5" />
          Reset
        </button>
      </div>
    </div>
  );
}

export default ImportData;
