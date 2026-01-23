# Dashboard User Flow Documentation

## Overview
This document provides comprehensive user flow diagrams for the Equans Dashboard application. These diagrams illustrate how users navigate through the dashboard, access key information, and interact with various components.

---

## 1. Authentication & Main Navigation Flow

This diagram shows the initial user journey from login through the main dashboard sections.

\\\mermaid
flowchart TD
    Start([User Opens Application]) --> Login[Login Screen]
    Login --> Auth{Valid<br/>Credentials?}
    Auth -->|No| Login
    Auth -->|Yes| Dashboard[Dashboard Overview]
    
    Dashboard --> Nav{User Selects<br/>Navigation}
    
    Nav -->|Sidebar: Dashboard| Dashboard
    Nav -->|Sidebar: Products| Products[Product Analysis Page]
    Nav -->|Sidebar: Users| UserList[User Management Page]
    Nav -->|Sidebar: Settings| Settings[Settings Page]
    Nav -->|Logout| Logout[Logout Action]
    
    Logout --> Login
    
    style Start fill:#e1f5ff
    style Dashboard fill:#d4edda
    style Products fill:#fff3cd
    style UserList fill:#fff3cd
    style Settings fill:#f8d7da
    style Login fill:#e1f5ff
\\\

**Key Entry Points:**
- **Login Screen**: Initial entry point requiring authentication
- **Sidebar Navigation**: Primary navigation method with 4 main sections
- **Topbar**: Displays current location with breadcrumbs

---

## 2. Dashboard Overview - Detailed User Journey

This shows the interactions available from the main dashboard screen.

\\\mermaid
flowchart TD
    Dashboard[ Dashboard Overview] --> ViewKPIs[View Key Metrics]
    
    ViewKPIs --> KPI1[Total Licenses: 775]
    ViewKPIs --> KPI2[Active Users: 510]
    ViewKPIs --> KPI3[Monthly Cost: €53,000]
    ViewKPIs --> KPI4[Utilization Rate: 76.8%]
    
    Dashboard --> Charts[View Charts & Trends]
    Charts --> LicenseChart[Licenses per Product Chart]
    Charts --> TrendChart[Cost & User Trends Chart]
    
    Dashboard --> TopCustomers[View Top Customers Table]
    TopCustomers --> CustomerDetails[Customer Details]
    
    Dashboard --> ClickKPI{Click on<br/>Stat Card}
    ClickKPI -->|Total Licenses Card| NavProducts[Navigate to Product Analysis]
    
    Dashboard --> TableAction{Click Customer<br/>in Table}
    TableAction -->|View User| NavUserDetail[Navigate to User Detail]
    
    NavProducts --> ProductPage[Product Analysis Page]
    NavUserDetail --> UserDetailPage[User Detail Page]
    
    style Dashboard fill:#d4edda
    style NavProducts fill:#fff3cd
    style NavUserDetail fill:#cfe2ff
    style ProductPage fill:#fff3cd
    style UserDetailPage fill:#cfe2ff
\\\

**User Actions:**
- View real-time KPI metrics with trend indicators
- Click "Total Licenses" card to drill into product details
- Explore interactive charts for visual insights
- Click on any customer in the table to view user details
- Access "View All" button for complete customer list

---

## 3. Product Analysis Flow

This diagram illustrates the user journey through product-specific insights.

\\\mermaid
flowchart TD
    Entry1[From Dashboard] --> ProductPage[ Product Analysis Page]
    Entry2[From Sidebar] --> ProductPage
    
    ProductPage --> SelectProduct[Product Dropdown Selector]
    SelectProduct --> Products{Select Product}
    
    Products -->|Option 1| Jira[Jira Analysis]
    Products -->|Option 2| Confluence[Confluence Analysis]
    Products -->|Option 3| MSTeams[MS Teams Analysis]
    Products -->|Option 4| MSOffice[MS Office Analysis]
    Products -->|Option 5| Slack[Slack Analysis]
    
    ProductPage --> ViewMetrics[View Product-Specific KPIs]
    ViewMetrics --> ProdKPI1[Total Licenses for Product]
    ViewMetrics --> ProdKPI2[Active Users for Product]
    ViewMetrics --> ProdKPI3[Product Monthly Cost]
    ViewMetrics --> ProdKPI4[Product Utilization Rate]
    
    ProductPage --> ViewCharts[View Product Charts]
    ViewCharts --> UsageDonut[Usage Distribution Chart]
    ViewCharts --> ProductTrend[Product Trend Chart]
    
    ProductPage --> ViewUsers[View Users Table]
    ViewUsers --> FilteredUsers[Users with This License]
    
    FilteredUsers --> ClickUser{Click on<br/>User Row}
    ClickUser --> UserDetail[Navigate to User Detail Page]
    
    UserDetail --> BackButton[Click Back Button]
    BackButton --> ProductPage
    
    style ProductPage fill:#fff3cd
    style UserDetail fill:#cfe2ff
    style Entry1 fill:#d4edda
    style Entry2 fill:#e1f5ff
\\\

**User Actions:**
- Select specific product from dropdown (Jira, Confluence, MS Teams, etc.)
- View product-specific metrics and KPIs
- Analyze usage patterns through donut and trend charts
- View filtered list of users assigned to selected product
- Click on any user to view detailed profile
- Use back navigation to return to product view

---

## 4. User Management Flow

This shows how users navigate through the user management section.

\\\mermaid
flowchart TD
    Entry1[From Sidebar] --> UserMgmt[ User Management Page]
    Entry2[From Dashboard Table] --> UserMgmt
    
    UserMgmt --> ViewStats[View User Statistics]
    ViewStats --> Stat1[Total Users]
    ViewStats --> Stat2[Active Users]
    ViewStats --> Stat3[Inactive Users]
    ViewStats --> Stat4[Avg Licenses per User]
    
    UserMgmt --> SearchFilter[Search & Filter Users]
    SearchFilter --> SearchBox[Search by Name/Email/Dept]
    SearchFilter --> StatusFilter[Filter by Status]
    SearchFilter --> DeptFilter[Filter by Department]
    
    SearchFilter --> FilteredList[Filtered User List]
    
    UserMgmt --> UserTable[View Users Table]
    UserTable --> UserActions{User Actions}
    
    UserActions -->|Click View Icon| ViewUser[View User Detail]
    UserActions -->|Click Edit Icon| EditUser[Edit User Dialog]
    UserActions -->|Click Email Icon| SendEmail[Send Email to User]
    UserActions -->|Click More Menu| MoreOptions[Additional Options]
    
    ViewUser --> UserDetailPage[User Detail Page]
    
    UserMgmt --> AddUser[Click "Add User" Button]
    AddUser --> NewUserForm[New User Creation Form]
    
    UserMgmt --> Export[Click "Export" Button]
    Export --> ExportData[Download User Data]
    
    style UserMgmt fill:#fff3cd
    style UserDetailPage fill:#cfe2ff
    style Entry1 fill:#e1f5ff
    style Entry2 fill:#d4edda
\\\

**User Actions:**
- Search users by name, email, or department
- Filter by active/inactive status
- Filter by department
- View comprehensive user statistics
- Quick actions: View, Edit, Email user
- Add new users to the system
- Export user data for reporting

---

## 5. User Detail Page Flow

Detailed view of individual user interactions and information access.

\\\mermaid
flowchart TD
    Entry1[From Dashboard] --> UserDetail[ User Detail Page]
    Entry2[From Product Page] --> UserDetail
    Entry3[From User Management] --> UserDetail
    
    UserDetail --> BackNav[Back Button Navigation]
    BackNav --> PrevPage{Return to<br/>Previous Page}
    PrevPage -->|From Dashboard| Dashboard[Dashboard]
    PrevPage -->|From Products| Products[Product Analysis]
    PrevPage -->|From Users| UserMgmt[User Management]
    
    UserDetail --> ViewProfile[View User Profile]
    ViewProfile --> BasicInfo[Name, Email, Department]
    ViewProfile --> StatusBadge[Active/Inactive Status]
    ViewProfile --> LastActive[Last Activity Date]
    
    UserDetail --> ViewLicenses[View Assigned Licenses]
    ViewLicenses --> LicenseList[List of All Licenses]
    LicenseList --> LicenseDetails[License Details & Dates]
    
    UserDetail --> ViewActivity[View Activity History]
    ViewActivity --> ActivityChart[Activity Trend Chart]
    ViewActivity --> UsageMetrics[Usage Statistics]
    
    UserDetail --> Actions{User Actions}
    Actions -->|Edit User Button| EditForm[Edit User Information]
    Actions -->|Manage Licenses Button| LicenseMgmt[License Management Dialog]
    
    EditForm --> SaveChanges[Save Changes]
    SaveChanges --> UserDetail
    
    LicenseMgmt --> AddRemove[Add/Remove Licenses]
    AddRemove --> UserDetail
    
    style UserDetail fill:#cfe2ff
    style Dashboard fill:#d4edda
    style Products fill:#fff3cd
    style UserMgmt fill:#fff3cd
\\\

**User Actions:**
- Navigate back to previous page using context-aware back button
- View complete user profile information
- Review all assigned licenses with activation dates
- Analyze user activity and usage patterns over time
- Edit user information directly
- Manage license assignments (add/remove)

---

## 6. Complete Application Navigation Map

High-level view of all navigation paths and relationships.

\\\mermaid
graph TD
    Login([ Login]) -->|Authenticate| App[Application Shell]
    
    App --> Sidebar[ Sidebar Navigation]
    App --> Topbar[ Topbar with Breadcrumbs]
    
    Sidebar --> S1[Dashboard]
    Sidebar --> S2[Products]
    Sidebar --> S3[Users]
    Sidebar --> S4[Settings]
    Sidebar --> S5[Logout]
    
    S1 --> D[ Dashboard Page]
    S2 --> P[ Product Analysis]
    S3 --> U[ User Management]
    S4 --> Set[ Settings]
    S5 --> Login
    
    D -->|Click Licenses Card| P
    D -->|Click User in Table| UD[ User Detail]
    
    P -->|Select Product| P
    P -->|Click User Row| UD
    
    U -->|Click View User| UD
    U -->|Search & Filter| U
    
    UD -->|Back Button| BackContext{Previous Context}
    BackContext --> D
    BackContext --> P
    BackContext --> U
    
    Topbar --> Breadcrumb[Contextual Breadcrumbs]
    Breadcrumb --> NavHierarchy[Shows Current Location]
    
    style Login fill:#e1f5ff
    style D fill:#d4edda
    style P fill:#fff3cd
    style U fill:#fff3cd
    style UD fill:#cfe2ff
    style Set fill:#f8d7da
    style App fill:#f0f0f0
\\\

---

## 7. User Interaction Patterns

### Primary Interaction Types

\\\mermaid
flowchart LR
    User([User]) --> I1[Click Navigation]
    User --> I2[Click Cards/Stats]
    User --> I3[Click Table Rows]
    User --> I4[Use Filters/Search]
    User --> I5[Click Action Buttons]
    
    I1 --> Nav[Sidebar Menu Items]
    I2 --> Drill[Drill-down to Details]
    I3 --> Detail[View Detail Pages]
    I4 --> Filter[Refined Data Views]
    I5 --> Action[Perform Actions]
    
    Nav --> Pages[Different Pages]
    Drill --> ProductPage[Product Analysis]
    Drill --> UserPage[User Details]
    Detail --> UserPage
    Filter --> UpdatedView[Updated View]
    Action --> Dialogs[Modals/Dialogs]
    
    style User fill:#e1f5ff
\\\

### Navigation Hierarchy

\\\
Application

 Login (Entry Point)
    Authentication Gate

 Authenticated Application
    
     Sidebar (Primary Navigation)
        Dashboard (Home)
        Products (Analysis)
        Users (Management)
        Settings (Configuration)
        Logout (Exit)
    
     Topbar (Context & Breadcrumbs)
        Shows: Home > Section > Subsection
    
     Main Content Area
        
         Dashboard Overview
            4 KPI Cards (clickable)
            2 Charts (visual insights)
            Top Customers Table (clickable rows)
        
         Product Analysis
            Product Selector Dropdown
            Product-specific KPIs
            Usage Charts
            User List Table (clickable rows)
        
         User Management
            Search Box
            Status Filter
            Department Filter
            User Statistics
            Users Table (action buttons)
        
         User Detail (Contextual)
            Back Button (context-aware)
            User Profile Info
            License List
            Activity Chart
            Action Buttons
        
         Settings
             Under Development
\\\

---

## Expected User Journeys

### Journey 1: Executive Overview (Quick Insights)
**Goal**: Get a quick snapshot of license usage and costs

1. **Login**  Authenticate
2. **Dashboard**  View KPIs at a glance
3. **Review Charts**  Understand trends
4. **Check Top Customers**  Identify high-usage accounts
5. **Done**  Quick 2-minute overview complete

**Time**: 2-3 minutes

---

### Journey 2: Product License Audit
**Goal**: Analyze specific product usage and identify optimization opportunities

1. **Login**  Authenticate
2. **Navigate to Products**  Click Products in sidebar
3. **Select Product**  Choose Jira from dropdown
4. **Review Product KPIs**  Check licenses, users, cost, utilization
5. **Analyze Charts**  Identify usage patterns
6. **Review User List**  See who has licenses
7. **Click on Low-Usage User**  Investigate underutilization
8. **View User Detail**  Check activity history
9. **Consider Action**  Decide if license should be reassigned
10. **Navigate Back**  Return to product view or dashboard

**Time**: 5-10 minutes

---

### Journey 3: User License Management
**Goal**: Add licenses to a new team member

1. **Login**  Authenticate
2. **Navigate to Users**  Click Users in sidebar
3. **Search for User**  Enter name in search box
4. **Filter by Department**  Narrow down results
5. **Click View User**  Open user detail page
6. **Review Current Licenses**  Check what they already have
7. **Click "Manage Licenses"**  Open license management
8. **Add New License**  Assign required product
9. **Save Changes**  Confirm assignment
10. **Verify Update**  Check license appears in list
11. **Navigate Back**  Return to user management

**Time**: 3-5 minutes

---

### Journey 4: Cost Optimization Analysis
**Goal**: Identify cost-saving opportunities

1. **Login**  Authenticate
2. **Dashboard**  Review monthly cost KPI
3. **Click "Total Licenses" Card**  Drill into products
4. **Iterate Through Products**  Check each product's utilization
5. **Identify Low Utilization**  Find products under 60% usage
6. **View User List**  See inactive users
7. **Click Inactive Users**  Review their details
8. **Check Last Active Date**  Confirm inactivity
9. **Document Findings**  Note users for license removal
10. **Export Data**  Download report for management
11. **Navigate to Dashboard**  Plan optimization strategy

**Time**: 15-20 minutes

---

### Journey 5: Department Usage Review
**Goal**: Understand how a specific department uses licenses

1. **Login**  Authenticate
2. **Navigate to Users**  Click Users in sidebar
3. **Filter by Department**  Select target department
4. **Review Department Stats**  Check active/inactive users
5. **Sort by License Count**  Identify power users
6. **Click on Users**  Review individual details
7. **Note Product Preferences**  Understand department needs
8. **Navigate to Products**  Cross-reference product data
9. **Select Department's Top Product**  View product details
10. **Analyze Department Usage**  Compare to other departments
11. **Export Report**  Download for stakeholder review

**Time**: 10-15 minutes

---

## Key Features for Navigation

###  Contextual Back Navigation
- Back button remembers where you came from
- Returns to Dashboard, Products, or Users based on previous location
- Maintains user context and reduces clicks

###  Smart Filtering
- Real-time search across multiple fields
- Multiple filter combinations (status + department)
- Immediate results without page reload

###  Interactive Drill-Down
- Click KPI cards to explore details
- Click table rows to view individual records
- Charts provide visual context before drilling down

###  Persistent Breadcrumbs
- Always shows current location
- Hierarchical path: Home > Section > Subsection
- Helps users understand their position in the app

###  Quick Actions
- Action buttons directly in tables (View, Edit, Email)
- Modal dialogs for quick edits
- Export functionality for reporting

---

## Navigation Best Practices for Users

###  Do's
-  Use sidebar for major section changes
-  Use back button to return to previous context
-  Click KPI cards to drill into details
-  Use search and filters to narrow results
-  Check breadcrumbs to understand location
-  Click table rows to view details

###  Don'ts
-  Don't use browser back button (use app's back button)
-  Don't open multiple tabs (single-page application)
-  Don't refresh page during work (may lose context)

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
1. **Advanced Search**  Global search across all data
2. **Favorites/Bookmarks**  Quick access to frequent views
3. **Recent Items**  History of viewed users/products
4. **Notifications**  Alerts for important events
5. **Customizable Dashboard**  User-specific widget layouts

---

## Summary

The Equans Dashboard provides an intuitive navigation structure with:

- **5 Main Sections**: Dashboard, Products, Users, Settings, Logout
- **3 Detail Views**: User Detail (contextual), Product Analysis, User Management
- **Multiple Entry Points**: Direct access via sidebar, drill-down from cards and tables
- **Contextual Navigation**: Smart back button, breadcrumbs, filters
- **Quick Actions**: In-table actions, modal dialogs, export functions

All navigation paths are designed to minimize clicks and provide immediate access to relevant information for operational decision-making.

