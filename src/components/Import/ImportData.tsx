import React, { useState, useRef } from 'react';
import ExcelJS from 'exceljs';
import { AlertCircle, CheckCircle, Upload, Eye, RotateCcw, FileText, Download, Loader2 } from 'lucide-react';

interface ImportDataProps {
  type: 'organizations' | 'users';
}

export function ImportData({ type }: ImportDataProps) {
  const [previewRows, setPreviewRows] = useState<Record<string, any>[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [success, setSuccess] = useState(false);
  const [fileName, setFileName] = useState<string | null>(null);
  const [isDragOver, setIsDragOver] = useState(false);
  const [uploadProgress, setUploadProgress] = useState(0);
  const [parsing, setParsing] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const requiredColumns = type === 'organizations'
    ? ['id', 'name', 'domain']
    : ['id', 'firstName', 'lastName', 'email', 'organizationId'];

  const handleFile = async (file?: File) => {
    setError(null);
    setSuccess(false);
    setParsing(true);
    if (!file) {
      setParsing(false);
      return;
    }

    try {
      setFileName(file.name);
      const isCsv = file.name.toLowerCase().endsWith('.csv');
      let json: Record<string, any>[];

      if (isCsv) {
        const text = await file.text();
        const lines = text.split(/\r?\n/).filter((l) => l.trim());
        const headers = lines[0].split(',').map((h) => h.trim().replace(/^"|"$/g, ''));
        json = lines.slice(1).map((line) => {
          const values = line.split(',').map((v) => v.trim().replace(/^"|"$/g, ''));
          const row: Record<string, any> = {};
          headers.forEach((h, i) => { row[h] = values[i] ?? ''; });
          return row;
        });
      } else {
        const buffer = await file.arrayBuffer();
        const workbook = new ExcelJS.Workbook();
        await workbook.xlsx.load(buffer);
        const worksheet = workbook.worksheets[0];
        const headers: string[] = [];
        worksheet.getRow(1).eachCell({ includeEmpty: false }, (cell) => {
          headers.push(String(cell.value ?? '').trim());
        });
        json = [];
        worksheet.eachRow((row, rowNumber) => {
          if (rowNumber === 1) return;
          const rowData: Record<string, any> = {};
          row.eachCell({ includeEmpty: true }, (cell, colNumber) => {
            const header = headers[colNumber - 1];
            if (header) rowData[header] = cell.value ?? '';
          });
          if (Object.keys(rowData).length > 0) json.push(rowData);
        });
      }

      if (!Array.isArray(json) || json.length === 0) {
        setError('Geen rijen gevonden in het bestand.');
        setPreviewRows([]);
        setParsing(false);
        return;
      }

      const headerKeys = Object.keys(json[0]).map((k) => k.trim());
      const missing = requiredColumns.filter((c) => !headerKeys.includes(c));
      if (missing.length) {
        setError(`Ontbrekende kolommen: ${missing.join(', ')}`);
        setPreviewRows(json.slice(0, 10));
        setParsing(false);
        return;
      }

      setPreviewRows(json.slice(0, 100));
    } catch (e: any) {
      setError(e?.message || String(e));
      setPreviewRows([]);
    } finally {
      setParsing(false);
    }
  };

  const handleInputChange: React.ChangeEventHandler<HTMLInputElement> = (e) => {
    const f = e.target.files?.[0];
    handleFile(f);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
    const files = e.dataTransfer.files;
    if (files.length > 0) {
      handleFile(files[0]);
    }
  };

  const handleSubmit = async () => {
    setLoading(true);
    setError(null);
    setUploadProgress(0);

    try {
      const payload = { type, rows: previewRows };
      const res = await fetch('/api/import', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });

      // Simulate progress
      const progressInterval = setInterval(() => {
        setUploadProgress(prev => {
          if (prev >= 90) {
            clearInterval(progressInterval);
            return 90;
          }
          return prev + 10;
        });
      }, 200);

      if (!res.ok) {
        const txt = await res.text();
        throw new Error(txt || `Request failed: ${res.status}`);
      }

      setUploadProgress(100);
      setTimeout(() => {
        setSuccess(true);
        setPreviewRows([]);
        setFileName(null);
        setUploadProgress(0);
      }, 500);
    } catch (e: any) {
      setError(e?.message || String(e));
      setUploadProgress(0);
    } finally {
      setLoading(false);
    }
  };

  const downloadTemplate = async () => {
    const exampleData = type === 'organizations'
      ? [
          { id: '1', name: 'Example Corp', domain: 'example.com' },
          { id: '2', name: 'Another Company', domain: 'another.com' }
        ]
      : [
          { id: '1', firstName: 'John', lastName: 'Doe', email: 'john@example.com', organizationId: '1' },
          { id: '2', firstName: 'Jane', lastName: 'Smith', email: 'jane@example.com', organizationId: '2' }
        ];

    const workbook = new ExcelJS.Workbook();
    const sheet = workbook.addWorksheet(type);
    sheet.columns = requiredColumns.map((h) => ({ header: h, key: h, width: 20 }));
    sheet.addRows(exampleData);
    const buffer = await workbook.xlsx.writeBuffer();
    const blob = new Blob([buffer], { type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${type}_template.xlsx`;
    a.click();
    URL.revokeObjectURL(url);
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
              ref={fileInputRef}
              type="file"
              accept=".csv,application/vnd.ms-excel,application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
              onChange={handleInputChange}
              className="sr-only"
            />
            <div
              onDragOver={handleDragOver}
              onDragLeave={handleDragLeave}
              onDrop={handleDrop}
              className={`border-2 border-dashed rounded-lg p-8 text-center transition-all duration-200 cursor-pointer ${
                isDragOver
                  ? 'border-ui-blue-primary bg-ui-blue-light scale-105'
                  : 'border-ui-blue-primary hover:bg-ui-blue-light'
              }`}
            >
              {parsing ? (
                <div className="flex flex-col items-center">
                  <Loader2 className="w-10 h-10 text-ui-blue-primary animate-spin mb-3" />
                  <p className="text-sm font-medium text-slate-900">Processing file...</p>
                  <p className="text-xs text-slate-500 mt-1">Please wait</p>
                </div>
              ) : (
                <>
                  <Upload className="w-10 h-10 text-ui-blue-primary mx-auto mb-3" />
                  <p className="text-sm font-medium text-slate-900">
                    {isDragOver ? 'Drop your file here' : 'Select a file or drag and drop'}
                  </p>
                  <p className="text-xs text-slate-500 mt-1">CSV or Excel (XLSX) files are supported</p>
                  {fileName && (
                    <div className="flex items-center justify-center gap-2 mt-3 p-2 bg-ui-blue-lighter rounded">
                      <FileText className="w-4 h-4 text-ui-blue-primary" />
                      <p className="text-sm text-ui-blue-primary font-semibold">{fileName}</p>
                    </div>
                  )}
                </>
              )}
            </div>
          </div>
        </label>

        <div className="mt-6 p-4 bg-slate-50 rounded-lg border border-slate-200">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-slate-700 mb-2">Required columns:</p>
              <div className="flex flex-wrap gap-2">
                {requiredColumns.map((col) => (
                  <span key={col} className="px-3 py-1 bg-ui-blue-primary text-white text-xs font-medium rounded-full">
                    {col}
                  </span>
                ))}
              </div>
            </div>
            <button
              onClick={downloadTemplate}
              className="flex items-center gap-2 px-4 py-2 bg-white border border-slate-300 rounded-lg text-sm font-medium text-slate-700 hover:bg-slate-50 hover:border-slate-400 transition-colors"
            >
              <Download className="w-4 h-4" />
              Template
            </button>
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
        <div className="p-6 bg-ui-green-light border-l-4 border-ui-green-DEFAULT relative overflow-hidden animate-in zoom-in-95 duration-300">
          <div className="absolute inset-0 pointer-events-none">
            {[...Array(15)].map((_, i) => (
              <div
                key={i}
                className={`absolute w-1 h-1 rounded-full animate-bounce`}
                style={{
                  left: `${10 + Math.random() * 80}%`,
                  top: `${10 + Math.random() * 80}%`,
                  backgroundColor: ['#fbbf24', '#f59e0b', '#10b981', '#06b6d4', '#8b5cf6'][i % 5],
                  animationDelay: `${Math.random() * 1}s`,
                  animationDuration: `${0.5 + Math.random() * 0.5}s`
                }}
              />
            ))}
          </div>
          <div className="flex items-start gap-4 relative z-10">
            <CheckCircle className="w-6 h-6 text-ui-green-DEFAULT flex-shrink-0 mt-0.5 animate-pulse" />
            <div>
              <h3 className="font-semibold text-slate-900">🎉 Success!</h3>
              <p className="text-sm text-slate-700 mt-1">Your data has been imported successfully.</p>
            </div>
          </div>
        </div>
      )}

      {/* Preview Section */}
      {previewRows.length > 0 && (
        <div className="p-8 border-t border-slate-200 animate-in fade-in-0 slide-in-from-bottom-4 duration-500">
          <div className="flex items-center gap-2 mb-4">
            <Eye className="w-5 h-5 text-slate-700" />
            <h3 className="text-lg font-semibold text-slate-900">Preview</h3>
            <span className="text-sm text-slate-600">(showing up to 100 rows)</span>
          </div>

          <div className="overflow-x-auto rounded-lg border border-slate-200 shadow-sm">
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
                  <tr
                    key={i}
                    className={`${i % 2 === 0 ? 'bg-white' : 'bg-slate-50'} border-b border-slate-200 hover:bg-ui-blue-lighter transition-colors animate-in fade-in-0 duration-300`}
                    style={{ animationDelay: `${i * 50}ms` }}
                  >
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

          <div className="flex items-center justify-between mt-4">
            <div className="flex items-center gap-4">
              <div className="text-xs text-slate-500">
                Total rows to import: <span className="font-semibold text-slate-700">{previewRows.length}</span>
              </div>
              <div className="text-xs text-slate-500">
                Columns: <span className="font-semibold text-slate-700">{Object.keys(previewRows[0]).length}</span>
              </div>
            </div>
            <div className="flex items-center gap-2 text-xs text-slate-500">
              <CheckCircle className="w-4 h-4 text-green-500" />
              Data validated successfully
            </div>
          </div>
        </div>
      )}

      {/* Action Buttons */}
      <div className="p-8 bg-slate-50 border-t border-slate-200">
        {loading && (
          <div className="mb-4">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium text-slate-700">Uploading data...</span>
              <span className="text-sm text-slate-600">{uploadProgress}%</span>
            </div>
            <div className="w-full bg-slate-200 rounded-full h-2">
              <div
                className="bg-ui-blue-primary h-2 rounded-full transition-all duration-300"
                style={{ width: `${uploadProgress}%` }}
              ></div>
            </div>
          </div>
        )}

        <div className="flex gap-3">
          <button
            onClick={handleSubmit}
            disabled={previewRows.length === 0 || loading || parsing}
            className={`flex items-center gap-2 px-6 py-3 rounded-lg font-semibold transition-all duration-200 ${
              previewRows.length === 0 || loading || parsing
                ? 'bg-slate-300 text-slate-500 cursor-not-allowed'
                : 'bg-ui-blue-primary text-white hover:shadow-lg hover:scale-105'
            }`}
          >
            <Upload className="w-5 h-5" />
            {loading ? 'Uploading...' : previewRows.length > 0 ? 'Import Data' : 'Select a file first'}
          </button>

          <button
            onClick={handleReset}
            disabled={loading || parsing}
            className="flex items-center gap-2 px-6 py-3 rounded-lg font-semibold text-slate-700 bg-white border-2 border-slate-300 hover:border-slate-400 hover:shadow-md transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <RotateCcw className="w-5 h-5" />
            Reset
          </button>
        </div>
      </div>
    </div>
  );
}

export default ImportData;
