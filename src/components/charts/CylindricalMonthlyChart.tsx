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

// EQUANS CORPORATE COLORS - Following ADR-00X Guidelines
const EQUANS_BUSINESS_UNIT_COLORS: { [key: string]: string } = {
  "Digital Services": " #008163",     // Main Corporate:Light Blue
  "IT Operations": " #00DEE8",       // Main Corporate: Dark Green  
  "Smart Energy": "#70BD95",        // Main Corporate: Turquoise
  "Building Solutions": "#0059CE",   // Accompanying: Azure Blue
  "Field Operations": "#FF9600",     // Accompanying: Orange
};

// Secondary colors with opacity for hierarchy
const EQUANS_COLORS_WITH_OPACITY = {
  primary: "rgba(0, 36, 57, 1)",      // 100% - Primary data
  primarySecondary: "rgba(0, 36, 57, 0.6)", // 60% - Secondary data  
  primaryBackground: "rgba(0, 36, 57, 0.2)", // 20% - Background/contextual
  white: "#FFFFFF"
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
    <div 
      className="w-full py-4 relative chart-container" 
      onMouseMove={handleMouseMove}
      style={{ fontFamily: 'Roboto, sans-serif' }}
    >
      {/* Enhanced Tooltip with Equans Corporate Styling */}
      {hoveredSegment && (
        <div 
          style={{
            position: "absolute",
            left: `${mousePosition.x + 15}px`,
            top: `${mousePosition.y - 10}px`,
            zIndex: 1000,
            background: EQUANS_COLORS_WITH_OPACITY.white,
            border: `2px solid ${EQUANS_BUSINESS_UNIT_COLORS["IT Operations"]}`, // Dark Green border
            borderRadius: "12px",
            padding: "16px 20px",
            boxShadow: "0 8px 32px rgba(0, 36, 57, 0.15)", // Dark blue shadow with opacity
            minWidth: "240px",
            textAlign: "left",
            pointerEvents: "none",
            transform: "translateY(-50%)",
            fontFamily: 'Roboto, sans-serif'
          }}
        >
          <div style={{ 
            fontSize: "16px", 
            fontWeight: 700, 
            color: EQUANS_BUSINESS_UNIT_COLORS["Digital Services"], // Pink
            marginBottom: "12px",
            fontFamily: 'Roboto, sans-serif'
          }}>
            {hoveredSegment.month}
          </div>
          <div style={{ 
            display: "flex", 
            alignItems: "center", 
            gap: "12px", 
            marginBottom: "12px" 
          }}>
            <div style={{ 
              width: "16px", 
              height: "16px", 
              borderRadius: "4px", 
              background: hoveredSegment.color 
            }} />
            <span style={{ 
              fontSize: "14px", 
              fontWeight: 500, 
              color: EQUANS_BUSINESS_UNIT_COLORS["Digital Services"],
              fontFamily: 'Roboto, sans-serif'
            }}>
              {hoveredSegment.businessUnit}
            </span>
          </div>
          <div style={{ 
            fontSize: "15px", 
            color: EQUANS_BUSINESS_UNIT_COLORS["IT Operations"], // Dark Green
            fontWeight: 600,
            fontFamily: 'Roboto, sans-serif'
          }}>
            Cost: €{hoveredSegment.cost.toLocaleString()}
          </div>
          <div style={{ 
            fontSize: "14px", 
            color: EQUANS_BUSINESS_UNIT_COLORS["Smart Energy"], // Turquoise
            fontWeight: 500, 
            marginTop: "6px",
            fontFamily: 'Roboto, sans-serif'
          }}>
            Active Users: {hoveredSegment.activeUsers.toLocaleString()}
          </div>
          <div style={{ 
            fontSize: "13px", 
            color: EQUANS_COLORS_WITH_OPACITY.primarySecondary, // 60% opacity secondary data
            marginTop: "6px",
            fontFamily: 'Roboto, sans-serif'
          }}>
            Share: {hoveredSegment.percentage.toFixed(1)}%
          </div>
        </div>
      )}

      <div 
        className="flex items-end justify-around gap-6 px-8 pb-12" 
        style={{ height: "400px" }}
      >
        {data.map((monthData) => {
          const heightPercentage = (monthData.totalCost / maxCost) * 100;
          const cylinderHeight = Math.max((heightPercentage / 100) * 300, 80);

          return (
            <div 
              key={monthData.month} 
              className="flex flex-col items-center justify-end flex-1" 
              style={{ height: "100%" }}
            >
              {/* Cost Label with Equans Typography */}
              <div className="mb-2 text-center">
                <div 
                  className="text-sm data-primary"
                  style={{ 
                    fontWeight: 600,
                    color: EQUANS_BUSINESS_UNIT_COLORS["Digital Services"],
                    fontFamily: 'Roboto, sans-serif'
                  }}
                >
                  €{monthData.totalCost.toLocaleString()}
                </div>
              </div>

              <div className="relative" style={{ width: "88px", height: `${cylinderHeight}px` }}>
                {/* Enhanced Top Ellipse with Equans styling */}
                <div 
                  className="absolute top-0 left-0 right-0" 
                  style={{ 
                    height: "18px", 
                    borderRadius: "999px", 
                    background: `linear-gradient(180deg, ${EQUANS_COLORS_WITH_OPACITY.white} 0%, ${EQUANS_COLORS_WITH_OPACITY.primaryBackground} 100%)`,
                    border: `1px solid ${EQUANS_COLORS_WITH_OPACITY.primaryBackground}`,
                    zIndex: 10 
                  }} 
                />

                {/* Enhanced Cylinder Body */}
                <div 
                  style={{ 
                    marginTop: "9px", 
                    marginBottom: "9px", 
                    height: "calc(100% - 18px)", 
                    borderRadius: "12px", 
                    background: EQUANS_COLORS_WITH_OPACITY.primaryBackground, // 20% opacity background
                    boxShadow: `inset -4px 0 10px ${EQUANS_COLORS_WITH_OPACITY.primarySecondary}, 4px 0 10px ${EQUANS_COLORS_WITH_OPACITY.primaryBackground}`,
                    overflow: "hidden", 
                    border: `1px solid ${EQUANS_COLORS_WITH_OPACITY.primarySecondary}`
                  }}
                >
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
                          background: EQUANS_BUSINESS_UNIT_COLORS[bu.businessUnit] || bu.color,
                          borderTop: buIndex > 0 ? `1px solid ${EQUANS_COLORS_WITH_OPACITY.white}` : "none",
                          filter: isHovered ? "brightness(1.2)" : "brightness(1)",
                          transform: isHovered ? "scaleX(1.05)" : "scaleX(1)",
                          outline: isHovered ? `3px solid ${EQUANS_BUSINESS_UNIT_COLORS["Digital Services"]}` : "none",
                        }}
                        onMouseEnter={() => setHoveredSegment({ 
                          month: monthData.month, 
                          businessUnit: bu.businessUnit,
                          cost: bu.cost,
                          activeUsers: bu.activeUsers || 0,
                          percentage: bu.percentage,
                          color: EQUANS_BUSINESS_UNIT_COLORS[bu.businessUnit] || bu.color
                        })}
                        onMouseLeave={() => setHoveredSegment(null)}
                      />
                    );
                  })}
                </div>

                {/* Enhanced Bottom Ellipse */}
                <div 
                  className="absolute bottom-0 left-0 right-0" 
                  style={{ 
                    height: "18px", 
                    borderRadius: "999px", 
                    background: `linear-gradient(180deg, ${EQUANS_COLORS_WITH_OPACITY.primarySecondary} 0%, ${EQUANS_COLORS_WITH_OPACITY.primary} 100%)`,
                    border: `1px solid ${EQUANS_COLORS_WITH_OPACITY.primary}`,
                    zIndex: 10 
                  }} 
                />
              </div>

              {/* Month Label with Equans Typography */}
              <div className="mt-3 text-center">
                <div 
                  className="text-sm data-secondary"
                  style={{ 
                    fontWeight: 500,
                    color: EQUANS_COLORS_WITH_OPACITY.primarySecondary,
                    fontFamily: 'Roboto, sans-serif'
                  }}
                >
                  {monthData.month}
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {/* Enhanced Legend with Equans Corporate Colors */}
      <div className="mt-6 flex flex-wrap justify-center gap-6 px-4">
        {Object.entries(EQUANS_BUSINESS_UNIT_COLORS).map(([unit, color]) => (
          <div key={unit} className="flex items-center gap-3">
            <div 
              className="w-4 h-4 rounded" 
              style={{ background: color, border: `1px solid ${EQUANS_COLORS_WITH_OPACITY.primaryBackground}` }} 
            />
            <span 
              className="text-xs data-secondary"
              style={{ 
                fontWeight: 500,
                color: EQUANS_COLORS_WITH_OPACITY.primarySecondary,
                fontFamily: 'Roboto, sans-serif'
              }}
            >
              {unit}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}