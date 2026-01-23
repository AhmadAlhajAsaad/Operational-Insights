import React, { useState } from 'react';

interface BusinessUnitData {
  businessUnit: string;
  cost: number;
  activeUsers: number;
  percentage: number;
  color: string;
}

interface MonthData {
  month: string;
  totalCost: number;
  businessUnits: BusinessUnitData[];
}

interface CylindricalMonthlyChartProps {
  data: MonthData[];
}

const BUSINESS_UNIT_COLORS: { [key: string]: string } = {
  "Digital Services": "#3B82F6",
  "IT Operations": "#10B981",
  "Smart Energy": "#F59E0B",
  "Building Solutions": "#8B5CF6",
  "Field Operations": "#EC4899",
};

interface HoveredSegment {
  month: string;
  businessUnit: string;
  cost: number;
  activeUsers: number;
  percentage: number;
  color: string;
}

interface MousePosition {
  x: number;
  y: number;
}

export function CylindricalMonthlyChart({ data }: CylindricalMonthlyChartProps) {
  const [hoveredSegment, setHoveredSegment] = useState<HoveredSegment | null>(null);
  const [mousePosition, setMousePosition] = useState<MousePosition>({ x: 0, y: 0 });

  const maxCost = Math.max(...data.map(d => d.totalCost));

  const handleMouseMove = (e: React.MouseEvent) => {
    const rect = e.currentTarget.getBoundingClientRect();
    setMousePosition({
      x: e.clientX - rect.left,
      y: e.clientY - rect.top
    });
  };

  return (
    <div className="w-full py-4 relative" onMouseMove={handleMouseMove}>
      {/* Floating Tooltip - follows mouse cursor */}
      {hoveredSegment && (
        <div 
          style={{
            position: "absolute",
            left: `${mousePosition.x + 15}px`,
            top: `${mousePosition.y - 10}px`,
            zIndex: 1000,
            background: "white",
            border: "2px solid #10b981",
            borderRadius: "12px",
            padding: "14px 18px",
            boxShadow: "0 8px 32px rgba(0,0,0,0.18)",
            minWidth: "220px",
            textAlign: "left",
            pointerEvents: "none",
            transform: "translateY(-50%)"
          }}
        >
          <div style={{ fontSize: "15px", fontWeight: 700, color: "#111827", marginBottom: "10px" }}>
            {hoveredSegment.month}
          </div>
          <div style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "10px" }}>
            <div style={{ width: "14px", height: "14px", borderRadius: "4px", background: hoveredSegment.color }} />
            <span style={{ fontSize: "13px", fontWeight: 600, color: "#374151" }}>{hoveredSegment.businessUnit}</span>
          </div>
          <div style={{ fontSize: "14px", color: "#10b981", fontWeight: 600 }}>
            Cost: €{hoveredSegment.cost.toLocaleString()}
          </div>
          <div style={{ fontSize: "13px", color: "#276FD1", fontWeight: 600, marginTop: "4px" }}>
            Active Users: {hoveredSegment.activeUsers.toLocaleString()}
          </div>
          <div style={{ fontSize: "12px", color: "#6b7280", marginTop: "4px" }}>
            Share: {hoveredSegment.percentage.toFixed(1)}%
          </div>
        </div>
      )}

      <div className="flex items-end justify-around gap-6 px-8 pb-12" style={{ height: "400px" }}>
        {data.map((monthData) => {
          const heightPercentage = (monthData.totalCost / maxCost) * 100;
          const cylinderHeight = Math.max((heightPercentage / 100) * 300, 80);

          return (
            <div key={monthData.month} className="flex flex-col items-center justify-end flex-1" style={{ height: "100%" }}>
              <div className="mb-2 text-center">
                <div className="text-sm font-bold text-neutral-900">€{monthData.totalCost.toLocaleString()}</div>
              </div>

              <div className="relative" style={{ width: "88px", height: `${cylinderHeight}px` }}>
                {/* Top ellipse */}
                <div className="absolute top-0 left-0 right-0" style={{ height: "18px", borderRadius: "999px", background: "linear-gradient(180deg, rgba(255,255,255,0.55) 0%, rgba(200,200,200,0.25) 100%)", border: "1px solid rgba(0,0,0,0.08)", zIndex: 10 }} />

                {/* Cylinder body */}
                <div style={{ marginTop: "9px", marginBottom: "9px", height: "calc(100% - 18px)", borderRadius: "12px", background: "#f3f4f6", boxShadow: "inset -4px 0 10px rgba(0,0,0,0.22), 4px 0 10px rgba(0,0,0,0.10)", overflow: "hidden", border: "1px solid rgba(0,0,0,0.08)" }}>
                  {monthData.businessUnits.map((bu, buIndex) => {
                    const isHovered = hoveredSegment?.month === monthData.month && hoveredSegment?.businessUnit === bu.businessUnit;
                    return (
                      <div
                        key={bu.businessUnit}
                        className="cursor-pointer transition-all duration-200"
                        style={{
                          display: 'block',
                          width: '100%',
                          height: `${bu.percentage}%`,
                          background: bu.color,
                          borderTop: buIndex > 0 ? "1px solid rgba(255,255,255,0.35)" : "none",
                          filter: isHovered ? "brightness(1.3)" : "brightness(1)",
                          transform: isHovered ? "scaleX(1.08)" : "scaleX(1)",
                          outline: isHovered ? "3px solid #111827" : "none",
                        }}
                        onMouseEnter={() => setHoveredSegment({ 
                          month: monthData.month, 
                          businessUnit: bu.businessUnit,
                          cost: bu.cost,
                          activeUsers: bu.activeUsers || 0,
                          percentage: bu.percentage,
                          color: bu.color
                        })}
                        onMouseLeave={() => setHoveredSegment(null)}
                      />
                    );
                  })}
                </div>

                {/* Bottom ellipse */}
                <div className="absolute bottom-0 left-0 right-0" style={{ height: "18px", borderRadius: "999px", background: "linear-gradient(180deg, rgba(120,120,120,0.38) 0%, rgba(60,60,60,0.55) 100%)", border: "1px solid rgba(0,0,0,0.12)", zIndex: 10 }} />
              </div>

              <div className="mt-3 text-center"><div className="text-sm font-semibold text-neutral-700">{monthData.month}</div></div>
            </div>
          );
        })}
      </div>

      <div className="mt-6 flex flex-wrap justify-center gap-4 px-4">
        {Object.entries(BUSINESS_UNIT_COLORS).map(([unit, color]) => (
          <div key={unit} className="flex items-center gap-2">
            <div className="w-4 h-4 rounded" style={{ background: color }} />
            <span className="text-xs font-medium text-neutral-700">{unit}</span>
          </div>
        ))}
      </div>
    </div>
  );
}