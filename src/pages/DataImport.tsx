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
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1
            className="text-2xl font-bold"
            style={{
              color: "var(--color-equans-dark-blue)",
              fontFamily: "Roboto, sans-serif",
            }}
          >
            Data Import
          </h1>
          <p
            className="text-sm mt-1"
            style={{
              color: "var(--color-equans-dark-blue-60)",
              fontFamily: "Roboto, sans-serif",
            }}
          >
            Import organization and personnel data from CSV or Excel files
          </p>
        </div>
      </div>

      <Card>
        <CardHeader>
          <CardTitle
            className="text-lg"
            style={{
              color: "var(--color-equans-dark-blue)",
              fontFamily: "Roboto, sans-serif",
              fontWeight: 500,
            }}
          >
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
        <CardContent>
          <div className="flex gap-4">
            <button
              onClick={() => setImportType("organization")}
              className="flex-1 p-4 rounded-lg border-2 transition-all flex items-center gap-3"
              style={{
                backgroundColor:
                  importType === "organization"
                    ? "var(--color-equans-turquoise-20)"
                    : "var(--color-equans-white)",
                borderColor:
                  importType === "organization"
                    ? "var(--color-equans-turquoise)"
                    : "var(--color-equans-dark-blue-20)",
                fontFamily: "Roboto, sans-serif",
              }}
            >
              <Building2
                size={24}
                style={{
                  color:
                    importType === "organization"
                      ? "var(--color-equans-dark-green)"
                      : "var(--color-equans-dark-blue-60)",
                }}
              />
              <div className="text-left">
                <p
                  className="font-medium"
                  style={{
                    color: "var(--color-equans-dark-blue)",
                    fontWeight: 500,
                  }}
                >
                  Organization Data
                </p>
                <p
                  className="text-sm"
                  style={{ color: "var(--color-equans-dark-blue-60)" }}
                >
                  Import organizations and business units
                </p>
              </div>
            </button>
            <button
              onClick={() => setImportType("personnel")}
              className="flex-1 p-4 rounded-lg border-2 transition-all flex items-center gap-3"
              style={{
                backgroundColor:
                  importType === "personnel"
                    ? "var(--color-equans-turquoise-20)"
                    : "var(--color-equans-white)",
                borderColor:
                  importType === "personnel"
                    ? "var(--color-equans-turquoise)"
                    : "var(--color-equans-dark-blue-20)",
                fontFamily: "Roboto, sans-serif",
              }}
            >
              <Users
                size={24}
                style={{
                  color:
                    importType === "personnel"
                      ? "var(--color-equans-dark-green)"
                      : "var(--color-equans-dark-blue-60)",
                }}
              />
              <div className="text-left">
                <p
                  className="font-medium"
                  style={{
                    color: "var(--color-equans-dark-blue)",
                    fontWeight: 500,
                  }}
                >
                  Personnel Data
                </p>
                <p
                  className="text-sm"
                  style={{ color: "var(--color-equans-dark-blue-60)" }}
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

      <Card>
        <CardHeader>
          <CardTitle
            className="text-lg"
            style={{
              color: "var(--color-equans-dark-blue)",
              fontFamily: "Roboto, sans-serif",
              fontWeight: 500,
            }}
          >
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
            className="flex flex-col items-center justify-center w-full h-48 border-2 border-dashed rounded-xl cursor-pointer transition-all"
            style={{
              borderColor: isDragOver
                ? "var(--color-equans-turquoise)"
                : "var(--color-equans-dark-blue-20)",
              backgroundColor: isDragOver
                ? "var(--color-equans-turquoise-20)"
                : "var(--color-equans-white)",
            }}
          >
            <div className="flex flex-col items-center justify-center pt-5 pb-6">
              <Upload
                size={40}
                className="mb-3"
                style={{
                  color: isDragOver
                    ? "var(--color-equans-dark-green)"
                    : "var(--color-equans-dark-blue-60)",
                }}
              />
              <p
                className="mb-2 text-sm"
                style={{
                  color: "var(--color-equans-dark-blue)",
                  fontFamily: "Roboto, sans-serif",
                  fontWeight: 500,
                }}
              >
                <span>Click to upload</span> or drag and drop
              </p>
              <p
                className="text-xs"
                style={{
                  color: "var(--color-equans-dark-blue-60)",
                  fontFamily: "Roboto, sans-serif",
                }}
              >
                CSV, XLSX or XLS (max 50MB)
              </p>
            </div>
          </label>
          {selectedFile && (
            <div
              className="mt-4 p-3 rounded-lg flex items-center justify-between"
              style={{ backgroundColor: "var(--color-equans-turquoise-20)" }}
            >
              <div className="flex items-center gap-3">
                <FileSpreadsheet
                  size={24}
                  style={{ color: "var(--color-equans-dark-green)" }}
                />
                <div>
                  <p
                    className="text-sm font-medium"
                    style={{
                      color: "var(--color-equans-dark-blue)",
                      fontFamily: "Roboto, sans-serif",
                    }}
                  >
                    {selectedFile.name}
                  </p>
                  <p
                    className="text-xs"
                    style={{
                      color: "var(--color-equans-dark-blue-60)",
                      fontFamily: "Roboto, sans-serif",
                    }}
                  >
                    {(selectedFile.size / 1024).toFixed(1)} KB
                  </p>
                </div>
              </div>
              <button
                onClick={handleClear}
                className="p-1 rounded-full hover:bg-white/50 transition-colors"
              >
                <X
                  size={20}
                  style={{ color: "var(--color-equans-dark-blue-60)" }}
                />
              </button>
            </div>
          )}
          {parseError && (
            <Alert variant="destructive" className="mt-4">
              <AlertCircle size={16} />
              <AlertTitle>Error</AlertTitle>
              <AlertDescription>{parseError}</AlertDescription>
            </Alert>
          )}
        </CardContent>
      </Card>

      {parsedData && (
        <Card>
          <CardHeader>
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
                  <Eye size={20} />
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
              <div className="mt-6 space-y-2">
                <div className="flex justify-between text-sm">
                  <span
                    style={{
                      color: "var(--color-equans-dark-blue)",
                      fontFamily: "Roboto, sans-serif",
                    }}
                  >
                    Importing records...
                  </span>
                  <span
                    style={{
                      color: "var(--color-equans-dark-blue-60)",
                      fontFamily: "Roboto, sans-serif",
                    }}
                  >
                    {importProgress}%
                  </span>
                </div>
                <Progress value={importProgress} />
              </div>
            )}
            {importResult && (
              <Alert
                className="mt-6"
                style={{
                  backgroundColor: importResult.success
                    ? "var(--color-equans-turquoise-20)"
                    : "var(--color-equans-orange)",
                  borderColor: importResult.success
                    ? "var(--color-equans-dark-green)"
                    : "var(--color-equans-orange)",
                }}
              >
                {importResult.success ? (
                  <CheckCircle2
                    size={16}
                    style={{ color: "var(--color-equans-dark-green)" }}
                  />
                ) : (
                  <AlertCircle
                    size={16}
                    style={{ color: "var(--color-equans-dark-blue)" }}
                  />
                )}
                <AlertTitle
                  style={{
                    color: "var(--color-equans-dark-blue)",
                    fontFamily: "Roboto, sans-serif",
                  }}
                >
                  {importResult.success
                    ? "Import Successful"
                    : "Import Completed with Errors"}
                </AlertTitle>
                <AlertDescription
                  style={{
                    color: "var(--color-equans-dark-blue-60)",
                    fontFamily: "Roboto, sans-serif",
                  }}
                >
                  {importResult.message}
                  {importResult.errors.length > 0 && (
                    <ul className="mt-2 list-disc list-inside">
                      {importResult.errors.slice(0, 3).map((error, index) => (
                        <li key={index}>{error}</li>
                      ))}
                      {importResult.errors.length > 3 && (
                        <li>
                          ...and {importResult.errors.length - 3} more errors
                        </li>
                      )}
                    </ul>
                  )}
                </AlertDescription>
              </Alert>
            )}
            <div className="mt-6 flex gap-3 justify-end">
              <Button
                variant="outline"
                onClick={handleClear}
                style={{
                  borderColor: "var(--color-equans-dark-blue-60)",
                  color: "var(--color-equans-dark-blue)",
                  fontFamily: "Roboto, sans-serif",
                }}
              >
                Clear
              </Button>
              <Button
                onClick={handleImport}
                disabled={isImporting || !parsedData}
                style={{
                  backgroundColor: "var(--color-equans-dark-green)",
                  color: "var(--color-equans-white)",
                  fontFamily: "Roboto, sans-serif",
                  fontWeight: 500,
                }}
              >
                {isImporting
                  ? "Importing..."
                  : "Import " +
                    (importType === "organization"
                      ? "Organizations"
                      : "Personnel")}
              </Button>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
