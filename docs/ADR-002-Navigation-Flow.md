# Navigation Flow Diagram - ADR-002 Implementation

## Before (User-Centric)
```

  Login   

     
     

   Dashboard      Primary Entry (Generic metrics)

     
     
                    
                    
    
Products      Users    User-focused
    
                    
                    
              
               User Detail  
              
```

## After (Organization-Centric) - ADR-002 Compliant
```

  Login   

     
     

   Organization Overview              PRIMARY ENTRY POINT
                                     
   Total Cost: €60,000/month       
   5 Organizations                 
   Cost Trends Chart               
   Clickable Organization Table    

             
              Click Organization Row
             

   Organization Detail               
   (e.g., "SLS Digital Platform")   
                                     
   Org Cost: €18,450/month         
   Product Breakdown Table          Interactive
   Cost Trend for This Org         
   User List (Secondary)           

             
              Click Product Row
             

   Product Detail                    
   (e.g., "Jira" in "SLS Digital")  
                                     
   Product Licenses                
   User List for This Product       Interactive

             
              Click User Row
             

   User Detail                        FINAL DRILL-DOWN
   (Individual User Info)                (Secondary, Optional)
                                     
   User's Licenses                 
   Activity Data                   

```

## Key Differences

| Aspect | Before | After (ADR-002) |
|--------|--------|-----------------|
| **Entry Point** | Generic Dashboard | Organization Overview |
| **Primary Focus** | Users & Licenses | Cost & Organizations |
| **Hierarchy** | Flat (Dashboard  Details) | Layered (Org  Product  User) |
| **Cost Visibility** | Secondary metric | Primary metric at all levels |
| **User Access** | Direct from menu | Only through org/product context |
| **Business Unit** | Not emphasized | Core grouping principle |
| **Interactivity** | Limited | Drill-down at every level |
| **Decision Support** | Static numbers | Comparison, trends, analysis |

## Data Flow

```

  organizationData.ts (NEW)              
                                          
  organizations = [                       
    {                                     
      id: "sls-digital"                   
      monthlyCost: 18450                  
      products: [...]                     
      costTrend: [...]                    
    }                                     
  ]                                       

             
             

  OrganizationOverview.tsx (NEW)         
  - Aggregates all org costs             
  - Shows interactive table              
  - Renders cost trends                  

             
              User clicks org row
             

  OrganizationDetail.tsx (NEW)           
  - Shows single org details             
  - Product table (clickable)            
  - Cost trend for this org              

             
              User clicks product row
             

  ProductBreakdown.tsx                   
  - Filtered by org + product            
  - User list for this product           

```

## Implementation Principles Applied

1. **Organization-Centric** 
   - Organizations are the first thing users see
   - All data grouped by organization/BU

2. **Data-Driven** 
   - Data structure defined first
   - UI components built from data model

3. **Interactive Drill-Down** 
   - Every table row is clickable
   - Clear visual hierarchy
   - Back navigation at each level

4. **Cost as Primary Metric** 
   - Monthly cost shown at every level
   - Cost trends prominently displayed
   - Cost drives decision-making

5. **User Detail is Secondary** 
   - Users only in final drill-down
   - Not accessible from main menu
   - Context-dependent (org  product  user)
