# Dashboard User Flow Documentation

## Overview

This document provides comprehensive user flow diagrams for the Equans Dashboard application, emphasizing GDPR compliance, role-based access, and Microsoft SSO authentication. User-level data is restricted to administrators; non-admins see only aggregated business data.

---

## 1. Authentication & Role-Based Access Flow

**Authentication:**

Only via Microsoft SSO (Azure AD / Entra ID).
No custom login or user database.
**Authorization:**

- Role-based access determines data visibility.
  Admins can access user-level details; non-admins (e.g., BU-managers) cannot.

- Role-based access determines data visibility.
  Admins can access user-level details; non-admins (e.g., BU-managers) cannot.

\\\mermaid
flowchart TD
Start([User Opens Application]) --> Login[Login Screen]
Login --> Auth{Microsoft SSO<br/>Authentication}
Auth -->|No| Login
Auth -->|Yes| RoleCheck{Role Check}
RoleCheck -->|Admin| AdminDash[Admin Dashboard]
RoleCheck -->|Non-Admin| BUManagerDash[BU Manager Dashboard]

AdminDash --> AdminFeatures[Full Access to All Features]
BUManagerDash --> BUFeatures[Access to Aggregated Business Data Only]

style Start fill:#e1f5ff
style Login fill:#e1f5ff
style Auth fill:#e1f5ff
style RoleCheck fill:#cfe2ff
style AdminDash fill:#d4edda
style BUManagerDash fill:#fff3cd

\\\

**Key Entry Points:**

- **Login Screen**: Initial entry point requiring authentication
- **Sidebar Navigation**: Primary navigation method with 4 main sections
- **Topbar**: Displays current location with breadcrumbs

---

## 2. Main Navigation Flow (Role-Based)

This shows the interactions available from the main dashboard screen.

\\\mermaid
flowchart TD
AdminDash[Admin Dashboard] --> NavA{Navigation}
BUManagerDash[BU Manager Dashboard] --> NavB{Navigation}

NavA -->|Dashboard| AdminDash
NavA -->|Products| ProductsA[Product Analysis]
NavA -->|Users| UserMgmtA[User Management]
NavA -->|Settings| SettingsA[Settings]
NavA -->|Logout| LogoutA[Logout]

NavB -->|Dashboard| BUManagerDash
NavB -->|Products| ProductsB[Product Analysis]
NavB -->|Settings| SettingsB[Settings]
NavB -->|Logout| LogoutB[Logout]

style AdminDash fill:#d4edda
style BUManagerDash fill:#fff3cd
style ProductsA fill:#fff3cd
style UserMgmtA fill:#cfe2ff
style SettingsA fill:#f8d7da
style ProductsB fill:#fff3cd
style SettingsB fill:#f8d7da

\\\

**Key Difference:**
Admins have access to the "Users" section for user-level management, while BU Managers do not. Both roles can access the Dashboard and Product Analysis sections, but BU Managers see only aggregated data.

---

## 3. Drill-Down Flow (Admin vs. Non-Admin)

This diagram illustrates the user journey through product-specific insights.

\\\mermaid
flowchart TD
Auth[Authenticated] --> RoleCheck{Role Check}
RoleCheck -->|Admin| DrillAdmin[Drill-down: BU → Org → Product → User]
RoleCheck -->|Non-Admin| DrillBU[Drill-down: BU → Org → Product]
DrillAdmin --> UserDetail[User Detail Page]
DrillBU -.-> NoUserDetail[No Access to User Details]

style DrillAdmin fill:#d4edda
style DrillBU fill:#fff3cd
style UserDetail fill:#cfe2ff
style NoUserDetail fill:#f8d7da

\\\
**Key Entry Points:**

- **Dashboard KPI Cards**: Clickable cards that lead to product-specific insights
- **Product Analysis**: Clickable product rows that lead to user details (admin only)

---

## 4. GDPR & Compliance Flow

This shows how users navigate through the user management section.

\\\mermaid
flowchart TD
Auth[Authenticated] --> RoleCheck{Role Check}
RoleCheck -->|Admin| DataAccessA[Access: Aggregated & User-Level Data]
RoleCheck -->|Non-Admin| DataAccessB[Access: Aggregated Data Only]

DataAccessA --> UserTableA[View User Table]
DataAccessA --> UserDetailA[View User Details]
DataAccessB -.-> NoUserTable[No Access to User Table]
DataAccessB -.-> NoUserDetail[No Access to User Details]

style DataAccessA fill:#d4edda
style DataAccessB fill:#fff3cd
style UserTableA fill:#cfe2ff
style UserDetailA fill:#cfe2ff
style NoUserTable fill:#f8d7da
style NoUserDetail fill:#f8d7da

\\\

**GDPR Principle:**

- Only authorized users (Admins) may view personal/user-level data.
- Non-admins see only aggregated business data.

---

## 5. User Management Flow (Admin Only)

Detailed view of individual user interactions and information access.

\\\mermaid
flowchart TD
AdminDash[Admin Dashboard] --> UserMgmtA[User Management]
UserMgmtA --> SearchFilterA[Search & Filter Users]
UserMgmtA --> UserTableA[View Users Table]
UserTableA --> UserActionsA{User Actions}
UserActionsA -->|View| UserDetailA[User Detail]
UserActionsA -->|Email| EmailUserA[Send Email]
UserActionsA -->|Export| ExportA[Export Data]

style AdminDash fill:#d4edda
style UserMgmtA fill:#cfe2ff
style UserTableA fill:#cfe2ff
style UserDetailA fill:#cfe2ff
style EmailUserA fill:#cfe2ff
style ExportA fill:#cfe2ff

\\\

**User Actions:**

- **View**: Access detailed user information
- **Export**: Download user data for reporting

---

## 6. Product Analysis Flow (Both Roles)

High-level view of all navigation paths and relationships.

\\\mermaid
flowchart TD
Entry[From Dashboard] --> ProductPage[Product Analysis]
ProductPage --> SelectProduct[Select Product]
SelectProduct --> Products{Product Options}
Products --> Jira[Jira]
Products --> Confluence[Confluence]
Products --> MSTeams[MS Teams]
Products --> MSOffice[MS Office]
Products --> Slack[Slack]
ProductPage --> ViewMetrics[View KPIs]
ProductPage --> ViewCharts[View Charts]

style ProductPage fill:#fff3cd
style Entry fill:#d4edda

\\\
**Note:** Both Admins and BU Managers can access the Product Analysis page, but only Admins can drill down to user-level details from this page.

---

## 7. Summary of Access Control

**Admins:**
Can view and manage user-level data.
Can drill down to individual user details.
**Non-Admins (e.g., BU-managers):**
Can view only aggregated business data.
Cannot access user-level details.

## Technical Requirements

**Authentication:** Microsoft SSO (Azure AD / Entra ID)
**Authorization:** Role-based access (Admin vs. Non-Admin)

**GDPR Compliance:**

- User-level data is restricted to Admins.
- Aggregated data is default for all users.

**UI:**

- Hide user-level data and navigation for non-admins.
- Show only relevant sections based on user role.
  **Future Enhancements:**
- Advanced role management
- Audit logs for data access
- Customizable dashboard widgets per role

---

---

### Journey 2: Product License Audit

**Goal**: Analyze specific product usage and identify optimization opportunities

1. **Login** Authenticate
2. **Navigate to Products** Click Products in sidebar
3. **Select Product** Choose Jira from dropdown
4. **Review Product KPIs** Check licenses, users, cost, utilization
5. **Analyze Charts** Identify usage patterns
6. **Review User List** See who has licenses
7. **Click on Low-Usage User** Investigate underutilization
8. **View User Detail** Check activity history
9. **Consider Action** Decide if license should be reassigned
10. **Navigate Back** Return to product view or dashboard

**Time**: 5-10 minutes

---

### Journey 3: User License Management

**Goal**: Add licenses to a new team member

1. **Login** Authenticate
2. **Navigate to Users** Click Users in sidebar
3. **Search for User** Enter name in search box
4. **Filter by Department** Narrow down results
5. **Click View User** Open user detail page
6. **Review Current Licenses** Check what they already have
7. **Click "Manage Licenses"** Open license management
8. **Add New License** Assign required product
9. **Save Changes** Confirm assignment
10. **Verify Update** Check license appears in list
11. **Navigate Back** Return to user management

**Time**: 3-5 minutes

---

### Journey 4: Cost Optimization Analysis

**Goal**: Identify cost-saving opportunities

1. **Login** Authenticate
2. **Dashboard** Review monthly cost KPI
3. **Click "Total Licenses" Card** Drill into products
4. **Iterate Through Products** Check each product's utilization
5. **Identify Low Utilization** Find products under 60% usage
6. **View User List** See inactive users
7. **Click Inactive Users** Review their details
8. **Check Last Active Date** Confirm inactivity
9. **Document Findings** Note users for license removal
10. **Export Data** Download report for management
11. **Navigate to Dashboard** Plan optimization strategy

**Time**: 15-20 minutes

---

### Journey 5: Department Usage Review

**Goal**: Understand how a specific department uses licenses

1. **Login** Authenticate
2. **Navigate to Users** Click Users in sidebar
3. **Filter by Department** Select target department
4. **Review Department Stats** Check active/inactive users
5. **Sort by License Count** Identify power users
6. **Click on Users** Review individual details
7. **Note Product Preferences** Understand department needs
8. **Navigate to Products** Cross-reference product data
9. **Select Department's Top Product** View product details
10. **Analyze Department Usage** Compare to other departments
11. **Export Report** Download for stakeholder review

**Time**: 10-15 minutes

---

## Key Features for Navigation

### Contextual Back Navigation

- Back button remembers where you came from
- Returns to Dashboard, Products, or Users based on previous location
- Maintains user context and reduces clicks

### Smart Filtering

- Real-time search across multiple fields
- Multiple filter combinations (status + department)
- Immediate results without page reload

### Interactive Drill-Down

- Click KPI cards to explore details
- Click table rows to view individual records
- Charts provide visual context before drilling down

### Persistent Breadcrumbs

- Always shows current location
- Hierarchical path: Home > Section > Subsection
- Helps users understand their position in the app

### Quick Actions

- Action buttons directly in tables (View, Edit, Email)
- Modal dialogs for quick edits
- Export functionality for reporting

---

## Navigation Best Practices for Users

### Do's

- Use sidebar for major section changes
- Use back button to return to previous context
- Click KPI cards to drill into details
- Use search and filters to narrow results
- Check breadcrumbs to understand location
- Click table rows to view details

### Don'ts

- Don't use browser back button (use app's back button)
- Don't open multiple tabs (single-page application)
- Don't refresh page during work (may lose context)

---

## Technical Navigation Notes

### State Management

- Application maintains navigation state
- Previous view stored for contextual back navigation
- User selections preserved during session

### URL Structure

The application uses client-side routing without URL changes:

- Login state managed in app
- View state managed by React components
- No browser history manipulation

### Performance

- Instant navigation (no page reloads)
- Data cached during session
- Smooth transitions between views

---

## Accessibility Considerations

### Keyboard Navigation

- All interactive elements are keyboard accessible
- Tab order follows logical flow
- Enter key activates buttons and links

### Screen Readers

- Semantic HTML structure
- ARIA labels on interactive elements
- Breadcrumbs announce location changes

### Visual Indicators

- Active navigation item highlighted
- Hover states on clickable elements
- Status badges with color and text

---

## Future Enhancements

### Planned Features

1. **Advanced Search** Global search across all data
2. **Favorites/Bookmarks** Quick access to frequent views
3. **Recent Items** History of viewed users/products
4. **Notifications** Alerts for important events
5. **Customizable Dashboard** User-specific widget layouts

---

## Summary

The Equans Dashboard provides an intuitive navigation structure with:

- **5 Main Sections**: Dashboard, Products, Users, Settings, Logout
- **3 Detail Views**: User Detail (contextual), Product Analysis, User Management
- **Multiple Entry Points**: Direct access via sidebar, drill-down from cards and tables
- **Contextual Navigation**: Smart back button, breadcrumbs, filters
- **Quick Actions**: In-table actions, modal dialogs, export functions

All navigation paths are designed to minimize clicks and provide immediate access to relevant information for operational decision-making.
