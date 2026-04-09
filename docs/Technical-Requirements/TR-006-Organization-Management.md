# TR-006: Organisatie Beheer - Technische Specificaties

**Status:** In Review
**Datum:** 2026-02-18
**Auteur(s):** Backend Engineer
**Implementeert:** [FR-006](../Functional-Requirements/FR-006-Organization-Management.md)
**Applies To:** Backend Rust applicatie, PostgreSQL database, React frontend

---

## Scope

Dit document definieert de technische specificaties voor het organisatie beheer systeem, inclusief:

- Database schema voor organisaties
- REST API endpoints voor CRUD operaties
- Rust backend implementatie
- React frontend componenten
- Hiërarchische structuur (parent-child relaties)
- Aggregaties en statistieken
- Personen koppeling per organisatie

---

## Architectuur Overzicht

```
┌────────────────────┐     ┌──────────────────┐     ┌────────────────────┐
│  React Frontend    │────▶│   Rust Backend   │────▶│   PostgreSQL       │
│  (Organizations)   │◀────│   (Axum)         │◀────│   (organizations)  │
└────────────────────┘     └──────────────────┘     └────────────────────┘
                                     │
                                     ▼
                           ┌──────────────────┐
                           │   PostgreSQL     │
                           │   (persons)      │
                           └──────────────────┘
```

### Data Flow

1. Frontend vraagt organisatie lijst of detail op
2. Backend voert query uit met JOINs voor statistieken
3. Voor hiërarchie: recursive CTE query
4. Voor personen verdeling: GROUP BY country/billing_location
5. Response wordt getransformeerd naar DTO's
6. Frontend rendert data in tabel, tree view of detail cards

---

## Componenten Structuur

```
backend/
├── src/
│   ├── organizations/
│   │   ├── mod.rs                    # Module exports
│   │   ├── types.rs                  # Data types en DTOs
│   │   ├── repository.rs             # Database queries
│   │   ├── service.rs                # Business logic
│   │   └── error.rs                  # Error handling
│   ├── routes/
│   │   └── organizations.rs          # HTTP route handlers
│   └── main.rs
└── migrations/
    └── 002_persons_organizations.sql

frontend/
├── src/
│   ├── pages/
│   │   ├── OrganizationsList.tsx     # Overzicht pagina
│   │   ├── OrganizationDetail.tsx    # Detail pagina
│   │   └── OrganizationHierarchy.tsx # Hiërarchie tree view
│   ├── components/
│   │   ├── OrganizationTable.tsx     # Tabel component
│   │   ├── OrganizationCard.tsx      # Card component
│   │   └── TreeView.tsx              # Hiërarchie component
│   └── api/
│       └── organizations.ts          # API client calls
```

---

## Database Schema

### Organizations Tabel

```sql
CREATE TABLE IF NOT EXISTS organizations (
    id SERIAL PRIMARY KEY,
    org_id VARCHAR(20) NOT NULL UNIQUE,  -- e.g., ORG0042
    name VARCHAR(255) NOT NULL,
    description TEXT,
    parent_org_id VARCHAR(20) REFERENCES organizations(org_id) ON DELETE SET NULL,
    cost_center VARCHAR(50),
    manager VARCHAR(255),
    budget FLOAT,
    org_type VARCHAR(50) NOT NULL DEFAULT 'Business Unit',
    status VARCHAR(20) NOT NULL DEFAULT 'Active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Indexes

```sql
CREATE INDEX idx_organizations_parent ON organizations(parent_org_id);
CREATE INDEX idx_organizations_status ON organizations(status);
CREATE INDEX idx_organizations_cost_center ON organizations(cost_center);
CREATE INDEX idx_organizations_name ON organizations(name);
```

### Hiërarchie Query (Recursive CTE)

```sql
WITH RECURSIVE org_hierarchy AS (
    -- Base case: start with root organizations
    SELECT 
        org_id,
        name,
        parent_org_id,
        0 as level,
        ARRAY[org_id] as path
    FROM organizations
    WHERE parent_org_id IS NULL
    
    UNION ALL
    
    -- Recursive case: find children
    SELECT 
        o.org_id,
        o.name,
        o.parent_org_id,
        oh.level + 1,
        oh.path || o.org_id
    FROM organizations o
    INNER JOIN org_hierarchy oh ON o.parent_org_id = oh.org_id
)
SELECT * FROM org_hierarchy
ORDER BY path;
```

### Person Count View

```sql
CREATE OR REPLACE VIEW organization_stats AS
SELECT 
    o.org_id,
    o.name,
    COUNT(DISTINCT p.id) as person_count,
    COUNT(DISTINCT p.country) as country_count,
    COUNT(DISTINCT CASE WHEN p.status = 'Active' THEN p.id END) as active_person_count,
    COUNT(DISTINCT CASE WHEN p.status = 'Inactive' THEN p.id END) as inactive_person_count
FROM organizations o
LEFT JOIN persons p ON o.org_id = p.org_id
GROUP BY o.org_id, o.name;
```

---

## API Specificaties

### Base URL

```
/api/organizations
```

### Authenticatie

Alle endpoints vereisen JWT Bearer token met `user` of `admin` rol.

---

### GET /api/organizations

**Beschrijving:** Haalt lijst van organisaties op met statistieken.

**Query Parameters:**

| Parameter | Type | Verplicht | Default | Beschrijving |
|-----------|------|-----------|---------|--------------|
| `search` | string | Nee | - | Zoektekst voor org_id of naam |
| `status` | string | Nee | `Active` | Filter op status |
| `org_type` | string | Nee | - | Filter op type |
| `parent_org_id` | string | Nee | - | Filter op parent (laat child orgs zien) |
| `page` | integer | Nee | `1` | Paginanummer |
| `per_page` | integer | Nee | `25` | Aantal resultaten per pagina |
| `sort_by` | string | Nee | `org_id` | Sorteer kolom |
| `sort_order` | string | Nee | `asc` | Sorteer richting |

**Response (200 OK):**

```json
{
  "data": [
    {
      "org_id": "ORG0042",
      "name": "Equans DACH",
      "primary_country": "Austria",
      "person_count": 187,
      "country_count": 4,
      "status": "Active"
    },
    {
      "org_id": "ORG0043",
      "name": "Equans France",
      "primary_country": "France",
      "person_count": 342,
      "country_count": 2,
      "status": "Active"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 25,
    "total": 156,
    "total_pages": 7
  }
}
```

---

### GET /api/organizations/:org_id

**Beschrijving:** Haalt volledige details van één organisatie op inclusief statistieken.

**Path Parameters:**

| Parameter | Type | Beschrijving |
|-----------|------|--------------|
| `org_id` | string | Unieke organisatie identifier (bijv. ORG0042) |

**Response (200 OK):**

```json
{
  "org_id": "ORG0042",
  "name": "Equans DACH",
  "description": "Equans operations in DACH region",
  "parent_org_id": "ORG0001",
  "cost_center": "CC-DACH-001",
  "manager": "Hans Mueller",
  "budget": 5000000.00,
  "org_type": "Business Unit",
  "status": "Active",
  "person_count": 187,
  
  "children": [
    {
      "org_id": "ORG0043",
      "name": "Equans Austria",
      "person_count": 120
    },
    {
      "org_id": "ORG0044",
      "name": "Equans Switzerland",
      "person_count": 67
    }
  ],
  
  "country_distribution": [
    {
      "country": "Austria",
      "count": 120,
      "percentage": 64.17
    },
    {
      "country": "Switzerland",
      "count": 50,
      "percentage": 26.74
    },
    {
      "country": "Germany",
      "count": 15,
      "percentage": 8.02
    },
    {
      "country": "Liechtenstein",
      "count": 2,
      "percentage": 1.07
    }
  ],
  
  "created_at": "2025-01-15T10:00:00Z",
  "updated_at": "2026-02-17T14:30:00Z"
}
```

---

### GET /api/organizations/:org_id/persons

**Beschrijving:** Haalt alle personen op die aan een organisatie gekoppeld zijn.

**Path Parameters:**

| Parameter | Type | Beschrijving |
|-----------|------|--------------|
| `org_id` | string | Unieke organisatie identifier |

**Query Parameters:**

| Parameter | Type | Default | Beschrijving |
|-----------|------|---------|--------------|
| `status` | string | - | Filter op persoon status (Active/Inactive) |
| `country` | string | - | Filter op land |
| `page` | integer | `1` | Paginanummer |
| `per_page` | integer | `25` | Aantal resultaten per pagina |

**Response (200 OK):**

```json
{
  "data": [
    {
      "person_id": "CCJ183",
      "name": "Thomas WAGENSONNER",
      "email": "thomas.wagensonner@equans.com",
      "country": "Austria",
      "billing_location": "AT",
      "status": "Active"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 25,
    "total": 187,
    "total_pages": 8
  }
}
```

---

### GET /api/organizations/hierarchy

**Beschrijving:** Haalt de volledige organisatie hiërarchie op als tree structuur.

**Query Parameters:**

| Parameter | Type | Default | Beschrijving |
|-----------|------|---------|--------------|
| `root_org_id` | string | - | Start bij specifieke root (optional) |
| `max_depth` | integer | `5` | Maximale diepte van tree |

**Response (200 OK):**

```json
{
  "data": [
    {
      "org_id": "ORG0001",
      "name": "Equans Global",
      "person_count": 1250,
      "children": [
        {
          "org_id": "ORG0042",
          "name": "Equans DACH",
          "person_count": 187,
          "children": [
            {
              "org_id": "ORG0043",
              "name": "Equans Austria",
              "person_count": 120,
              "children": []
            }
          ]
        }
      ]
    }
  ]
}
```

---

### GET /api/organizations/:org_id/export

**Beschrijving:** Exporteert personen van een organisatie naar CSV.

**Response (200 OK):**

```csv
person_id,first_name,last_name,email,country,billing_location,status
CCJ183,Thomas,WAGENSONNER,thomas.wagensonner@equans.com,Austria,AT,Active
DEI311,Jürg,RUPPANNER,juerg.ruppanner@equans.com,Switzerland,CH,Active
```

**Headers:**

```
Content-Type: text/csv; charset=utf-8
Content-Disposition: attachment; filename="org_ORG0042_persons_2026-02-18.csv"
```

---

### POST /api/organizations

**Beschrijving:** Maakt nieuwe organisatie aan (Admin only).

**Request Body:**

```json
{
  "org_id": "ORG0099",
  "name": "Equans Netherlands",
  "description": "Operations in Netherlands",
  "parent_org_id": "ORG0001",
  "cost_center": "CC-NL-001",
  "manager": "Jan de Vries",
  "budget": 3000000.00,
  "org_type": "Business Unit"
}
```

**Response (201 Created):**

```json
{
  "org_id": "ORG0099",
  "name": "Equans Netherlands",
  "description": "Operations in Netherlands",
  "parent_org_id": "ORG0001",
  "cost_center": "CC-NL-001",
  "manager": "Jan de Vries",
  "budget": 3000000.00,
  "org_type": "Business Unit",
  "status": "Active",
  "person_count": 0,
  "children": [],
  "country_distribution": [],
  "created_at": "2026-02-18T10:30:00Z",
  "updated_at": "2026-02-18T10:30:00Z"
}
```

---

### PUT /api/organizations/:org_id

**Beschrijving:** Wijzigt bestaande organisatie (Admin only).

**Request Body:**

```json
{
  "name": "Equans DACH Region",
  "description": "Updated description",
  "budget": 5500000.00
}
```

**Response (200 OK):**

Returns updated organization detail (same structure as GET).

---

### DELETE /api/organizations/:org_id

**Beschrijving:** Verwijdert organisatie (Admin only).

**Voorwaarden:**
- Organisatie mag geen personen hebben (person_count = 0)
- Organisatie mag geen child organisaties hebben

**Response (204 No Content):**

Geen body.

**Error Response (409 Conflict):**

```json
{
  "error": "Cannot delete organization with 187 active persons. Move persons first."
}
```

---

## Rust Implementatie Details

### Data Types

```rust
/// Organization database model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Organization {
    pub id: i32,
    pub org_id: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_org_id: Option<String>,
    pub cost_center: Option<String>,
    pub manager: Option<String>,
    pub budget: Option<f64>,
    pub org_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Organization summary for list view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationSummary {
    pub org_id: String,
    pub name: String,
    pub primary_country: Option<String>,
    pub person_count: i64,
    pub country_count: i64,
    pub status: String,
}

/// Organization tree node (for hierarchy view)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationTreeNode {
    pub org_id: String,
    pub name: String,
    pub person_count: i64,
    pub children: Vec<OrganizationTreeNode>,
}

/// Country distribution in organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryDistribution {
    pub country: String,
    pub count: i64,
    pub percentage: f64,
}
```

### Repository Pattern

```rust
pub struct OrganizationRepository {
    pool: PgPool,
}

impl OrganizationRepository {
    // Get paginated list with statistics
    pub async fn list(
        &self,
        params: OrganizationListParams,
    ) -> Result<(Vec<OrganizationSummary>, i64), sqlx::Error> {
        // Join with persons table to get counts
        // Apply filters
        // Return summaries + total count
    }
    
    // Get single organization with full details
    pub async fn get_by_id(
        &self,
        org_id: &str,
    ) -> Result<Option<OrganizationDetail>, sqlx::Error> {
        // Get organization
        // Get child organizations
        // Get country distribution
        // Build detail DTO
    }
    
    // Get organization hierarchy
    pub async fn get_hierarchy(
        &self,
        root_org_id: Option<&str>,
        max_depth: i32,
    ) -> Result<Vec<OrganizationTreeNode>, sqlx::Error> {
        // Use recursive CTE
        // Build tree structure
    }
    
    // Get persons for organization
    pub async fn get_persons(
        &self,
        org_id: &str,
        params: PersonListParams,
    ) -> Result<(Vec<PersonSummary>, i64), sqlx::Error> {
        // Join persons where org_id matches
        // Apply filters and pagination
    }
    
    // Create new organization
    pub async fn create(
        &self,
        org: OrganizationCreate,
    ) -> Result<Organization, sqlx::Error> {
        // Validate parent_org_id exists if provided
        // INSERT organization
    }
    
    // Update organization
    pub async fn update(
        &self,
        org_id: &str,
        updates: OrganizationUpdate,
    ) -> Result<Organization, sqlx::Error> {
        // UPDATE with partial fields
    }
    
    // Delete organization
    pub async fn delete(
        &self,
        org_id: &str,
    ) -> Result<(), sqlx::Error> {
        // Check no persons attached
        // Check no child orgs
        // DELETE
    }
    
    // Get country distribution
    pub async fn get_country_distribution(
        &self,
        org_id: &str,
    ) -> Result<Vec<CountryDistribution>, sqlx::Error> {
        // GROUP BY country with percentages
    }
}
```

### Service Layer

```rust
pub struct OrganizationService {
    repository: OrganizationRepository,
}

impl OrganizationService {
    // List organizations with business logic
    pub async fn list_organizations(
        &self,
        params: OrganizationListParams,
    ) -> Result<OrganizationListResponse, AppError> {
        // Validate params
        // Call repository
        // Transform to DTOs
        // Build pagination metadata
    }
    
    // Get organization detail
    pub async fn get_organization_detail(
        &self,
        org_id: &str,
    ) -> Result<OrganizationDetail, AppError> {
        // Get from repository
        // Return 404 if not found
    }
    
    // Get hierarchy tree
    pub async fn get_hierarchy(
        &self,
        root_org_id: Option<&str>,
        max_depth: i32,
    ) -> Result<Vec<OrganizationTreeNode>, AppError> {
        // Validate max_depth (1-10)
        // Call repository
        // Build tree structure recursively
    }
    
    // Create organization
    pub async fn create_organization(
        &self,
        org: OrganizationCreate,
    ) -> Result<OrganizationDetail, AppError> {
        // Validate org_id format (ORGXXXX)
        // Validate parent_org_id exists
        // Create in repository
        // Return detail
    }
    
    // Update organization
    pub async fn update_organization(
        &self,
        org_id: &str,
        updates: OrganizationUpdate,
    ) -> Result<OrganizationDetail, AppError> {
        // Validate updates
        // Update in repository
        // Return updated detail
    }
    
    // Delete organization
    pub async fn delete_organization(
        &self,
        org_id: &str,
    ) -> Result<(), AppError> {
        // Check person_count = 0
        // Check no children
        // Delete from repository
    }
    
    // Export persons to CSV
    pub async fn export_persons(
        &self,
        org_id: &str,
    ) -> Result<String, AppError> {
        // Get all persons for org (no pagination)
        // Generate CSV string
    }
}
```

---

## Frontend Implementatie

### OrganizationsList Component

```typescript
export const OrganizationsList: React.FC = () => {
  const [organizations, setOrganizations] = useState<OrganizationSummary[]>([]);
  const [searchTerm, setSearchTerm] = useState('');
  const [pagination, setPagination] = useState<Pagination>();
  
  useEffect(() => {
    fetchOrganizations({ search: searchTerm, page: pagination?.page });
  }, [searchTerm, pagination?.page]);
  
  return (
    <div>
      <SearchBar value={searchTerm} onChange={setSearchTerm} />
      <OrganizationTable 
        organizations={organizations} 
        onRowClick={handleOrgClick}
      />
      <Pagination {...pagination} onPageChange={handlePageChange} />
    </div>
  );
};
```

### OrganizationDetail Component

```typescript
interface OrganizationDetailProps {
  orgId: string;
}

export const OrganizationDetail: React.FC<OrganizationDetailProps> = ({ orgId }) => {
  const [org, setOrg] = useState<OrganizationDetail | null>(null);
  const [activeTab, setActiveTab] = useState<'info' | 'persons' | 'structure'>('info');
  
  useEffect(() => {
    fetchOrganizationDetail(orgId);
  }, [orgId]);
  
  return (
    <div>
      <OrganizationHeader org={org} />
      <Tabs activeTab={activeTab} onTabChange={setActiveTab}>
        <Tab name="info">
          <InfoCard org={org} />
          <CountryDistribution data={org?.country_distribution} />
        </Tab>
        <Tab name="persons">
          <PersonsTable orgId={orgId} />
        </Tab>
        <Tab name="structure">
          <ChildOrganizations children={org?.children} />
        </Tab>
      </Tabs>
    </div>
  );
};
```

### OrganizationHierarchy Component

```typescript
export const OrganizationHierarchy: React.FC = () => {
  const [hierarchyData, setHierarchyData] = useState<OrganizationTreeNode[]>([]);
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set());
  
  useEffect(() => {
    fetchHierarchy();
  }, []);
  
  const toggleNode = (orgId: string) => {
    setExpandedNodes(prev => {
      const newSet = new Set(prev);
      if (newSet.has(orgId)) {
        newSet.delete(orgId);
      } else {
        newSet.add(orgId);
      }
      return newSet;
    });
  };
  
  return (
    <TreeView 
      data={hierarchyData} 
      expanded={expandedNodes}
      onToggle={toggleNode}
    />
  );
};
```

---

## Hiërarchie Implementatie

### Recursive CTE Query

```rust
pub async fn get_hierarchy(
    &self,
    root_org_id: Option<&str>,
    max_depth: i32,
) -> Result<Vec<OrganizationTreeNode>, sqlx::Error> {
    let query = r#"
        WITH RECURSIVE org_hierarchy AS (
            -- Base case
            SELECT 
                o.org_id,
                o.name,
                o.parent_org_id,
                0 as level,
                COALESCE(COUNT(p.id), 0) as person_count,
                ARRAY[o.org_id] as path
            FROM organizations o
            LEFT JOIN persons p ON o.org_id = p.org_id
            WHERE ($1::VARCHAR IS NULL OR o.org_id = $1)
              AND ($1::VARCHAR IS NOT NULL OR o.parent_org_id IS NULL)
            GROUP BY o.org_id, o.name, o.parent_org_id
            
            UNION ALL
            
            -- Recursive case
            SELECT 
                o.org_id,
                o.name,
                o.parent_org_id,
                oh.level + 1,
                COALESCE(COUNT(p.id), 0) as person_count,
                oh.path || o.org_id
            FROM organizations o
            INNER JOIN org_hierarchy oh ON o.parent_org_id = oh.org_id
            LEFT JOIN persons p ON o.org_id = p.org_id
            WHERE oh.level < $2
            GROUP BY o.org_id, o.name, o.parent_org_id, oh.level, oh.path
        )
        SELECT 
            org_id,
            name,
            parent_org_id,
            level,
            person_count,
            path
        FROM org_hierarchy
        ORDER BY path
    "#;
    
    let rows = sqlx::query_as::<_, HierarchyRow>(query)
        .bind(root_org_id)
        .bind(max_depth)
        .fetch_all(&self.pool)
        .await?;
    
    // Build tree structure
    Ok(build_tree(rows))
}

fn build_tree(rows: Vec<HierarchyRow>) -> Vec<OrganizationTreeNode> {
    let mut nodes: HashMap<String, OrganizationTreeNode> = HashMap::new();
    let mut root_ids: Vec<String> = Vec::new();
    
    // Create all nodes
    for row in &rows {
        nodes.insert(
            row.org_id.clone(),
            OrganizationTreeNode {
                org_id: row.org_id.clone(),
                name: row.name.clone(),
                person_count: row.person_count,
                children: Vec::new(),
            },
        );
        
        if row.parent_org_id.is_none() {
            root_ids.push(row.org_id.clone());
        }
    }
    
    // Build parent-child relationships
    for row in &rows {
        if let Some(parent_id) = &row.parent_org_id {
            if let Some(parent) = nodes.get_mut(parent_id) {
                if let Some(child) = nodes.get(&row.org_id).cloned() {
                    parent.children.push(child);
                }
            }
        }
    }
    
    // Return root nodes
    root_ids
        .into_iter()
        .filter_map(|id| nodes.remove(&id))
        .collect()
}
```

---

## Statistieken Berekening

### Country Distribution

```rust
pub async fn get_country_distribution(
    &self,
    org_id: &str,
) -> Result<Vec<CountryDistribution>, sqlx::Error> {
    let query = r#"
        WITH total AS (
            SELECT COUNT(*) as total_count
            FROM persons
            WHERE org_id = $1
        )
        SELECT 
            p.country,
            COUNT(*) as count,
            ROUND((COUNT(*)::FLOAT / t.total_count * 100)::NUMERIC, 2) as percentage
        FROM persons p
        CROSS JOIN total t
        WHERE p.org_id = $1 AND p.country IS NOT NULL
        GROUP BY p.country, t.total_count
        ORDER BY count DESC
    "#;
    
    sqlx::query_as::<_, CountryDistribution>(query)
        .bind(org_id)
        .fetch_all(&self.pool)
        .await
}
```

---

## Validatie

### Organization ID Format

```rust
pub fn validate_org_id(org_id: &str) -> Result<(), ValidationError> {
    let regex = Regex::new(r"^ORG\d{4}$").unwrap();
    
    if !regex.is_match(org_id) {
        return Err(ValidationError::InvalidOrgId(
            "Organization ID must follow format ORG0000".to_string()
        ));
    }
    
    Ok(())
}
```

### Parent Organization Validation

```rust
pub async fn validate_parent_org(
    &self,
    parent_org_id: &str,
) -> Result<(), ValidationError> {
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM organizations WHERE org_id = $1)"
    )
    .bind(parent_org_id)
    .fetch_one(&self.pool)
    .await?;
    
    if !exists {
        return Err(ValidationError::ParentOrgNotFound(
            parent_org_id.to_string()
        ));
    }
    
    Ok(())
}
```

### Deletion Validation

```rust
pub async fn can_delete_organization(
    &self,
    org_id: &str,
) -> Result<(), OrganizationError> {
    // Check person count
    let person_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM persons WHERE org_id = $1"
    )
    .bind(org_id)
    .fetch_one(&self.pool)
    .await?;
    
    if person_count > 0 {
        return Err(OrganizationError::HasPersons(person_count));
    }
    
    // Check child organizations
    let child_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM organizations WHERE parent_org_id = $1"
    )
    .bind(org_id)
    .fetch_one(&self.pool)
    .await?;
    
    if child_count > 0 {
        return Err(OrganizationError::HasChildren(child_count));
    }
    
    Ok(())
}
```

---

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum OrganizationError {
    #[error("Organization not found: {0}")]
    NotFound(String),
    
    #[error("Invalid organization ID format: {0}")]
    InvalidId(String),
    
    #[error("Organization already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Parent organization not found: {0}")]
    ParentNotFound(String),
    
    #[error("Cannot delete organization with {0} persons")]
    HasPersons(i64),
    
    #[error("Cannot delete organization with {0} child organizations")]
    HasChildren(i64),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for OrganizationError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            OrganizationError::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Organization {} not found", id)
            ),
            OrganizationError::InvalidId(id) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid organization ID: {}", id)
            ),
            OrganizationError::AlreadyExists(id) => (
                StatusCode::CONFLICT,
                format!("Organization {} already exists", id)
            ),
            OrganizationError::ParentNotFound(id) => (
                StatusCode::BAD_REQUEST,
                format!("Parent organization {} not found", id)
            ),
            OrganizationError::HasPersons(count) => (
                StatusCode::CONFLICT,
                format!("Cannot delete organization with {} active persons", count)
            ),
            OrganizationError::HasChildren(count) => (
                StatusCode::CONFLICT,
                format!("Cannot delete organization with {} child organizations", count)
            ),
            OrganizationError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error occurred".to_string()
            ),
        };
        
        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

---

## Performance Optimalisatie

### Materialized View voor Statistieken

```sql
CREATE MATERIALIZED VIEW organization_statistics AS
SELECT 
    o.org_id,
    COUNT(DISTINCT p.id) as person_count,
    COUNT(DISTINCT p.country) as country_count,
    COUNT(DISTINCT CASE WHEN p.status = 'Active' THEN p.id END) as active_count,
    MAX(p.updated_at) as last_person_update
FROM organizations o
LEFT JOIN persons p ON o.org_id = p.org_id
GROUP BY o.org_id;

CREATE UNIQUE INDEX idx_org_stats_org_id ON organization_statistics(org_id);

-- Refresh strategy: nightly or on-demand
REFRESH MATERIALIZED VIEW CONCURRENTLY organization_statistics;
```

### Indexes Optimalisatie

```sql
-- For person count queries
CREATE INDEX idx_persons_org_status ON persons(org_id, status);

-- For country distribution
CREATE INDEX idx_persons_org_country ON persons(org_id, country) 
WHERE country IS NOT NULL;

-- For hierarchy queries
CREATE INDEX idx_organizations_parent_status ON organizations(parent_org_id, status);
```

---

## Testing Strategie

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_org_id_validation() {
        assert!(validate_org_id("ORG0042").is_ok());
        assert!(validate_org_id("ORG9999").is_ok());
        assert!(validate_org_id("ORG042").is_err());  // Too short
        assert!(validate_org_id("ORGABCD").is_err()); // Not numeric
        assert!(validate_org_id("org0042").is_err()); // Lowercase
    }
    
    #[test]
    fn test_tree_building() {
        let rows = vec![
            HierarchyRow {
                org_id: "ORG0001".to_string(),
                name: "Root".to_string(),
                parent_org_id: None,
                level: 0,
                person_count: 10,
                path: vec!["ORG0001".to_string()],
            },
            HierarchyRow {
                org_id: "ORG0002".to_string(),
                name: "Child".to_string(),
                parent_org_id: Some("ORG0001".to_string()),
                level: 1,
                person_count: 5,
                path: vec!["ORG0001".to_string(), "ORG0002".to_string()],
            },
        ];
        
        let tree = build_tree(rows);
        
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].org_id, "ORG0001");
        assert_eq!(tree[0].children.len(), 1);
        assert_eq!(tree[0].children[0].org_id, "ORG0002");
    }
}
```

### Integration Tests

```rust
#[sqlx::test]
async fn test_organization_hierarchy(pool: PgPool) {
    let repo = OrganizationRepository::new(pool);
    
    // Insert test data
    sqlx::query("INSERT INTO organizations (org_id, name) VALUES ('ORG0001', 'Root')")
        .execute(&repo.pool)
        .await
        .unwrap();
    
    sqlx::query("INSERT INTO organizations (org_id, name, parent_org_id) VALUES ('ORG0002', 'Child', 'ORG0001')")
        .execute(&repo.pool)
        .await
        .unwrap();
    
    // Test hierarchy
    let hierarchy = repo.get_hierarchy(None, 5).await.unwrap();
    
    assert_eq!(hierarchy.len(), 1);
    assert_eq!(hierarchy[0].org_id, "ORG0001");
    assert_eq!(hierarchy[0].children.len(), 1);
}
```

---

## Gerelateerde Documenten

- Functional Requirement: [FR-006](../Functional-Requirements/FR-006-Organization-Management.md)
- Technical Requirement: [TR-005](TR-005-Person-Management.md)
- Technical Requirement: [TR-007](TR-007-Data-Import.md)
- Business Requirement: [BR-002](../Business-Requirements/BR-002-Person-Organization-Management.md)
