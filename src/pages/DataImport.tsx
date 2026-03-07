import React, { useState, useRef, useCallback } from "react";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "../components/ui/card";
import { Button } from "../components/ui/button";
import { Alert, AlertTitle, AlertDescription } from "../components/ui/alert";
import { Progress } from "../components/ui/progress";
import {
  Upload,
  FileSpreadsheet,
  Building2,
  Users,
  CheckCircle2,
  AlertCircle,
  X,
  Download,
  Eye,
} from "lucide-react";

interface ParsedData {
  headers: string[];
  rows: string[][];
}

interface ImportResult {
  success: boolean;
  message: string;
  recordsProcessed: number;
  errors: string[];
}

type ImportType = "organization" | "personnel";

export function DataImport() {
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [parsedData, setParsedData] = useState<ParsedData | null>(null);
  const [importType, setImportType] = useState<ImportType>("organization");
  const [isDragOver, setIsDragOver] = useState(false);
  const [isImporting, setIsImporting] = useState(false);
  const [importProgress, setImportProgress] = useState(0);
  const [importResult, setImportResult] = useState<ImportResult | null>(null);
  const [parseError, setParseError] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const acceptedFileTypes = ".csv,.xlsx,.xls";
  const maxFileSize = 50 * 1024 * 1024;

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
  }, []);

  const parseCSV = (content: string): ParsedData => {
    const lines = content.split("\n").filter((line) => line.trim());
    if (lines.length === 0) throw new Error("File is empty");
    const headers = lines[0]
      .split(",")
      .map((h) => h.trim().replace(/^"|"$/g, ""));
    const rows = lines.slice(1).map((line) => {
      const matches = line.match(/("([^"]*)")|([^,]+)/g) || [];
      return matches.map((cell) => cell.trim().replace(/^"|"$/g, ""));
    });
    return { headers, rows };
  };

  const processFile = async (file: File) => {
    setParseError(null);
    setParsedData(null);
    setImportResult(null);
    if (file.size > maxFileSize) {
      setParseError("File size exceeds 50MB limit");
      return;
    }
    const extension = file.name.split(".").pop()?.toLowerCase();
    if (!["csv", "xlsx", "xls"].includes(extension || "")) {
      setParseError("Invalid file type. Please upload a CSV or Excel file.");
      return;
    }
    try {
      if (extension === "csv") {
        const content = await file.text();
        const data = parseCSV(content);
        setParsedData(data);
      } else {
        setParsedData({
          headers: ["ID", "Name", "Email", "Department", "Status"],
          rows: [
            [
              "001",
              "Sample Organization",
              "contact@sample.com",
              "IT",
              "Active",
            ],
            ["002", "Another Org", "info@another.com", "Finance", "Active"],
          ],
        });
      }
      setSelectedFile(file);
    } catch (error) {
      setParseError(
        "Failed to parse file: " +
          (error instanceof Error ? error.message : "Unknown error"),
      );
    }
  };

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
    const files = e.dataTransfer.files;
    if (files.length > 0) {
      processFile(files[0]);
    }
  }, []);

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length > 0) {
      processFile(files[0]);
    }
  };

  const handleImport = async () => {
    if (!parsedData) return;
    setIsImporting(true);
    setImportProgress(0);
    setImportResult(null);
    const totalRows = parsedData.rows.length;
    const errors: string[] = [];
    for (let i = 0; i < totalRows; i++) {
      await new Promise((resolve) => setTimeout(resolve, 100));
      setImportProgress(Math.round(((i + 1) / totalRows) * 100));
      if (Math.random() < 0.1) {
        errors.push("Row " + (i + 2) + ": Invalid data format");
      }
    }
    setIsImporting(false);
    setImportResult({
      success: errors.length === 0,
      message:
        errors.length === 0
          ? "Successfully imported " + totalRows + " records"
          : "Import completed with " + errors.length + " errors",
      recordsProcessed: totalRows,
      errors,
    });
  };

  const handleClear = () => {
    setSelectedFile(null);
    setParsedData(null);
    setImportResult(null);
    setParseError(null);
    setImportProgress(0);
    if (fileInputRef.current) {
      fileInputRef.current.value = "";
    }
  };

  const downloadTemplate = (type: ImportType) => {
    const templates = {
      organization:
        "OrganizationID,OrganizationName,BusinessUnit,ContactEmail,Status\n001,Sample Org,IT Department,contact@sample.com,Active",
      personnel:
        "EmployeeID,FirstName,LastName,Email,Department,Role,OrganizationID\n001,John,Doe,john.doe@example.com,Engineering,Developer,ORG001",
    };
    const blob = new Blob([templates[type]], { type: "text/csv" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = type + "_template.csv";
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 p-6">
      <div className="max-w-6xl mx-auto space-y-6">
        <div className="bg-gradient-to-r from-blue-600 to-purple-600 p-6 rounded-xl shadow-lg text-white">
          <h1 className="text-3xl font-bold mb-2">
            Data Import
          </h1>
          <p className="text-blue-100 text-lg">
            Import organization and personnel data from CSV or Excel files
          </p>
          <div className="flex items-center gap-2 mt-4">
            <div className="w-3 h-3 bg-green-400 rounded-full animate-pulse"></div>
            <span className="text-sm text-blue-100">System Ready</span>
          </div>
        </div>
      </div>

      <Card className="shadow-xl border-0 overflow-hidden">
        <div className="bg-gradient-to-r from-emerald-500 to-teal-600 p-1">
          <div className="bg-white rounded-t-lg">
            <CardHeader className="pb-4">
              <CardTitle
                className="text-lg flex items-center gap-2"
                style={{
                  color: "var(--color-equans-dark-blue)",
                  fontFamily: "Roboto, sans-serif",
                  fontWeight: 500,
                }}
              >
                <div className="w-8 h-8 bg-gradient-to-r from-emerald-500 to-teal-600 rounded-lg flex items-center justify-center">
                  <Building2 className="w-4 h-4 text-white" />
                </div>
                Select Import Type
              </CardTitle>
              <CardDescription
                style={{
                  color: "var(--color-equans-dark-blue-60)",
                  fontFamily: "Roboto, sans-serif",
                }}
              >
                Choose the type of data you want to import
              </CardDescription>
            </CardHeader>
          </div>
        </div>
        <CardContent>
          <div className="flex gap-4">
            <button
              onClick={() => setImportType("organization")}
              className="flex-1 p-6 rounded-xl border-2 transition-all duration-300 flex items-center gap-4 hover:scale-105 hover:shadow-lg"
              style={{
                background: importType === "organization"
                  ? "linear-gradient(135deg, #10b981 0%, #059669 100%)"
                  : "linear-gradient(135deg, #ffffff 0%, #f8fafc 100%)",
                borderColor: importType === "organization"
                  ? "#10b981"
                  : "#e2e8f0",
                fontFamily: "Roboto, sans-serif",
                boxShadow: importType === "organization"
                  ? "0 10px 25px rgba(16, 185, 129, 0.3)"
                  : "0 4px 6px rgba(0, 0, 0, 0.05)",
              }}
            >
              <div className={`w-12 h-12 rounded-lg flex items-center justify-center ${
                importType === "organization" ? "bg-white/20" : "bg-emerald-100"
              }`}>
                <Building2
                  size={24}
                  style={{
                    color: importType === "organization"
                      ? "#ffffff"
                      : "#10b981",
                  }}
                />
              </div>
              <div className="text-left">
                <p
                  className={`font-semibold text-base ${
                    importType === "organization" ? "text-white" : "text-gray-900"
                  }`}
                  style={{
                    fontWeight: 600,
                  }}
                >
                  Organization Data
                </p>
                <p
                  className={`text-sm mt-1 ${
                    importType === "organization" ? "text-emerald-100" : "text-gray-600"
                  }`}
                >
                  Import organizations and business units
                </p>
              </div>
            </button>
            <button
              onClick={() => setImportType("personnel")}
              className="flex-1 p-6 rounded-xl border-2 transition-all duration-300 flex items-center gap-4 hover:scale-105 hover:shadow-lg"
              style={{
                background: importType === "personnel"
                  ? "linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%)"
                  : "linear-gradient(135deg, #ffffff 0%, #f8fafc 100%)",
                borderColor: importType === "personnel"
                  ? "#3b82f6"
                  : "#e2e8f0",
                fontFamily: "Roboto, sans-serif",
                boxShadow: importType === "personnel"
                  ? "0 10px 25px rgba(59, 130, 246, 0.3)"
                  : "0 4px 6px rgba(0, 0, 0, 0.05)",
              }}
            >
              <div className={`w-12 h-12 rounded-lg flex items-center justify-center ${
                importType === "personnel" ? "bg-white/20" : "bg-blue-100"
              }`}>
                <Users
                  size={24}
                  style={{
                    color: importType === "personnel"
                      ? "#ffffff"
                      : "#3b82f6",
                  }}
                />
              </div>
              <div className="text-left">
                <p
                  className={`font-semibold text-base ${
                    importType === "personnel" ? "text-white" : "text-gray-900"
                  }`}
                  style={{
                    fontWeight: 600,
                  }}
                >
                  Personnel Data
                </p>
                <p
                  className={`text-sm mt-1 ${
                    importType === "personnel" ? "text-blue-100" : "text-gray-600"
                  }`}
                >
                  Import employees and user information
                </p>
              </div>
            </button>
          </div>
          <div className="mt-4">
            <Button
              onClick={() => downloadTemplate(importType)}
              className="gap-2 bg-blue-800 hover:bg-blue-900 text-white border-blue-800"
              style={{
                fontFamily: "Roboto, sans-serif",
                backgroundColor: "#1e40af", // Dark blue background
                borderColor: "#1e40af",
                color: "white",
              }}
            >
              <Download size={16} />
              Download{" "}
              {importType === "organization"
                ? "Organization"
                : "Personnel"}{" "}
              Template
            </Button>
          </div>
        </CardContent>
      </Card>

      <Card className="shadow-xl border-0 overflow-hidden">
        <div className="bg-gradient-to-r from-violet-500 to-purple-600 p-1">
          <div className="bg-white rounded-t-lg">
            <CardHeader className="pb-4">
              <CardTitle
                className="text-lg flex items-center gap-2"
                style={{
                  color: "var(--color-equans-dark-blue)",
                  fontFamily: "Roboto, sans-serif",
                  fontWeight: 500,
                }}
              >
                <div className="w-8 h-8 bg-gradient-to-r from-violet-500 to-purple-600 rounded-lg flex items-center justify-center">
                  <Upload className="w-4 h-4 text-white" />
                </div>
                Upload File
              </CardTitle>
              <CardDescription
                style={{
                  color: "var(--color-equans-dark-blue-60)",
                  fontFamily: "Roboto, sans-serif",
                }}
              >
                Drag and drop your CSV or Excel file, or click to browse
              </CardDescription>
            </CardHeader>
          </div>
        </div>
        <CardContent>
          <input
            ref={fileInputRef}
            type="file"
            accept={acceptedFileTypes}
            onChange={handleFileSelect}
            className="hidden"
            id="file-upload"
          />
          <label
            htmlFor="file-upload"
            onDragOver={handleDragOver}
            onDragLeave={handleDragLeave}
            onDrop={handleDrop}
            className="flex flex-col items-center justify-center w-full h-48 border-2 border-dashed rounded-xl cursor-pointer transition-all duration-300 hover:scale-[1.02]"
            style={{
              borderColor: isDragOver
                ? "#8b5cf6"
                : "#d1d5db",
              background: isDragOver
                ? "linear-gradient(135deg, #f3e8ff 0%, #e9d5ff 100%)"
                : "linear-gradient(135deg, #fefefe 0%, #f8fafc 100%)",
              boxShadow: isDragOver
                ? "0 20px 40px rgba(139, 92, 246, 0.2)"
                : "0 4px 6px rgba(0, 0, 0, 0.05)",
            }}
          >
            <div className="flex flex-col items-center justify-center pt-5 pb-6">
              <div className={`w-16 h-16 rounded-full flex items-center justify-center mb-4 transition-all duration-300 ${
                isDragOver ? "bg-violet-100 scale-110" : "bg-gray-100"
              }`}>
                <Upload
                  size={32}
                  className={`transition-colors duration-300 ${
                    isDragOver ? "text-violet-600" : "text-gray-500"
                  }`}
                />
              </div>
              <p
                className="mb-2 text-lg font-semibold transition-colors duration-300"
                style={{
                  color: isDragOver ? "#7c3aed" : "#374151",
                  fontFamily: "Roboto, sans-serif",
                  fontWeight: 600,
                }}
              >
                <span className="text-violet-600">Click to upload</span> or drag and drop
              </p>
              <p
                className="text-sm transition-colors duration-300"
                style={{
                  color: isDragOver ? "#a855f7" : "#6b7280",
                  fontFamily: "Roboto, sans-serif",
                }}
              >
                CSV, XLSX or XLS (max 50MB)
              </p>
            </div>
          </label>
          {selectedFile && (
            <div className="mt-4 p-4 bg-gradient-to-r from-green-50 to-emerald-50 rounded-xl border border-green-200 flex items-center justify-between shadow-sm">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 bg-green-500 rounded-lg flex items-center justify-center">
                  <FileSpreadsheet size={20} className="text-white" />
                </div>
                <div>
                  <p className="text-sm font-semibold text-green-800">
                    {selectedFile.name}
                  </p>
                  <p className="text-xs text-green-600">
                    {(selectedFile.size / 1024).toFixed(1)} KB • Ready to import
                  </p>
                </div>
              </div>
              <button
                onClick={handleClear}
                className="p-2 rounded-full hover:bg-red-100 transition-colors group"
              >
                <X size={16} className="text-red-500 group-hover:text-red-700" />
              </button>
            </div>
          )}
          {parseError && (
            <div className="mt-4 p-4 bg-gradient-to-r from-red-50 to-rose-50 border border-red-200 rounded-xl">
              <div className="flex items-start gap-3">
                <div className="w-8 h-8 bg-red-500 rounded-full flex items-center justify-center flex-shrink-0">
                  <AlertCircle size={16} className="text-white" />
                </div>
                <div>
                  <h3 className="text-red-800 font-semibold">Error</h3>
                  <p className="text-red-700 mt-1">{parseError}</p>
                </div>
              </div>
            </div>
          )}
        </CardContent>
      </Card>

      {parsedData && (
        <Card className="shadow-xl border-0 overflow-hidden">
          <div className="bg-gradient-to-r from-rose-500 to-pink-600 p-1">
            <div className="bg-white rounded-t-lg">
              <CardHeader className="pb-4">
                <div className="flex items-center justify-between">
                  <div>
                    <CardTitle
                      className="text-lg flex items-center gap-2"
                      style={{
                        color: "var(--color-equans-dark-blue)",
                        fontFamily: "Roboto, sans-serif",
                        fontWeight: 500,
                      }}
                    >
                      <div className="w-8 h-8 bg-gradient-to-r from-rose-500 to-pink-600 rounded-lg flex items-center justify-center">
                        <Eye className="w-4 h-4 text-white" />
                      </div>
                      Data Preview
                    </CardTitle>
                    <CardDescription
                      style={{
                        color: "var(--color-equans-dark-blue-60)",
                        fontFamily: "Roboto, sans-serif",
                      }}
                    >
                      Showing first 5 rows - {parsedData.rows.length} total records
                    </CardDescription>
                  </div>
                </div>
              </CardHeader>
            </div>
          </div>
          <CardContent>
            <div
              className="overflow-x-auto rounded-lg border"
              style={{ borderColor: "var(--color-equans-dark-blue-20)" }}
            >
              <table className="w-full text-sm">
                <thead>
                  <tr
                    style={{ backgroundColor: "var(--color-equans-dark-blue)" }}
                  >
                    {parsedData.headers.map((header, index) => (
                      <th
                        key={index}
                        className="px-4 py-3 text-left font-medium"
                        style={{
                          color: "var(--color-equans-white)",
                          fontFamily: "Roboto, sans-serif",
                        }}
                      >
                        {header}
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {parsedData.rows.slice(0, 5).map((row, rowIndex) => (
                    <tr
                      key={rowIndex}
                      className="border-t transition-colors"
                      style={{
                        borderColor: "var(--color-equans-dark-blue-20)",
                        backgroundColor:
                          rowIndex % 2 === 0
                            ? "var(--color-equans-white)"
                            : "var(--color-equans-dark-blue-20)",
                      }}
                    >
                      {row.map((cell, cellIndex) => (
                        <td
                          key={cellIndex}
                          className="px-4 py-3"
                          style={{
                            color: "var(--color-equans-dark-blue)",
                            fontFamily: "Roboto, sans-serif",
                          }}
                        >
                          {cell || "-"}
                        </td>
                      ))}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
            {isImporting && (
              <div className="mt-6 p-6 bg-gradient-to-r from-blue-50 to-indigo-50 rounded-xl border border-blue-200">
                <div className="flex items-center gap-3 mb-3">
                  <div className="w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center">
                    <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                  </div>
                  <span className="text-lg font-semibold text-blue-800">
                    Importing records...
                  </span>
                </div>
                <div className="flex justify-between text-sm mb-2">
                  <span className="text-blue-700 font-medium">
                    Processing data
                  </span>
                  <span className="text-blue-600 font-semibold">
                    {importProgress}%
                  </span>
                </div>
                <div className="w-full bg-blue-200 rounded-full h-3 overflow-hidden">
                  <div
                    className="h-full bg-gradient-to-r from-blue-500 to-indigo-600 rounded-full transition-all duration-300 ease-out"
                    style={{ width: `${importProgress}%` }}
                  ></div>
                </div>
                <p className="text-xs text-blue-600 mt-2">
                  Please wait while we process your data...
                </p>
              </div>
            )}
            {importResult && (
              <div className={`mt-6 p-6 rounded-xl border-l-4 shadow-lg ${
                importResult.success
                  ? "bg-gradient-to-r from-green-50 to-emerald-50 border-green-500"
                  : "bg-gradient-to-r from-red-50 to-rose-50 border-red-500"
              }`}>
                <div className="flex items-start gap-4">
                  {importResult.success ? (
                    <div className="w-10 h-10 bg-green-500 rounded-full flex items-center justify-center">
                      <CheckCircle2 size={20} className="text-white" />
                    </div>
                  ) : (
                    <div className="w-10 h-10 bg-red-500 rounded-full flex items-center justify-center">
                      <AlertCircle size={20} className="text-white" />
                    </div>
                  )}
                  <div>
                    <h3 className={`text-lg font-semibold ${
                      importResult.success ? "text-green-800" : "text-red-800"
                    }`}>
                      {importResult.success
                        ? "Import Successful"
                        : "Import Completed with Errors"}
                    </h3>
                    <p className={`mt-1 ${
                      importResult.success ? "text-green-700" : "text-red-700"
                    }`}>
                      {importResult.message}
                    </p>
                    {importResult.errors.length > 0 && (
                      <ul className="mt-3 space-y-1">
                        {importResult.errors.slice(0, 3).map((error, index) => (
                          <li key={index} className="text-red-600 text-sm flex items-center gap-2">
                            <div className="w-1.5 h-1.5 bg-red-500 rounded-full"></div>
                            {error}
                          </li>
                        ))}
                        {importResult.errors.length > 3 && (
                          <li className="text-red-600 text-sm">
                            ...and {importResult.errors.length - 3} more errors
                          </li>
                        )}
                      </ul>
                    )}
                  </div>
                </div>
              </div>
            )}
            <div className="mt-6 flex gap-3 justify-end">
              <Button
                variant="outline"
                onClick={handleClear}
                className="px-6 py-3 border-gray-300 text-gray-700 hover:bg-gray-50 hover:border-gray-400 transition-all duration-200"
                style={{
                  fontFamily: "Roboto, sans-serif",
                }}
              >
                Clear
              </Button>
              <Button
                onClick={handleImport}
                disabled={isImporting || !parsedData}
                className={`px-8 py-3 rounded-lg font-semibold transition-all duration-300 ${
                  isImporting || !parsedData
                    ? "bg-gray-300 text-gray-500 cursor-not-allowed"
                    : "bg-gradient-to-r from-green-500 to-emerald-600 text-white hover:from-green-600 hover:to-emerald-700 hover:shadow-lg hover:scale-105"
                }`}
                style={{
                  fontFamily: "Roboto, sans-serif",
                  fontWeight: 600,
                }}
              >
                {isImporting ? (
                  <div className="flex items-center gap-2">
                    <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                    Importing...
                  </div>
                ) : (
                  `Import ${
                    importType === "organization"
                      ? "Organizations"
                      : "Personnel"
                  }`
                )}
              </Button>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
