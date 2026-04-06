# ADR-004: API Authentication Strategy for Backend Access


## Context
The Equans Operational Insights backend exposes API endpoints that provide
sensitive data such as user information, license usage, and cost-related metrics
retrieved from Atlassian Cloud and GitHub Enterprise APIs.

**Security Problem:**
Without authentication, any user within the Equans network could directly call
these backend endpoints and retrieve data they are not authorized to see in the
source systems themselves. This creates security and GDPR risks.

**Why Simple Token Authentication is Insecure:**
A shared Bearer token embedded in the frontend application is **not secure** because:
- Frontend JavaScript is publicly accessible in the browser
- Anyone can inspect the code and extract the token
- The token can be copied and used by any malicious actor within the network
- This provides no user-level authentication or authorization

Therefore, we need a proper enterprise-grade authentication mechanism.

## Decision
The backend API will enforce authentication using **Azure AD / Entra ID** with JWT token validation.

### Authentication Flow:
1. **User Login:** Users authenticate via Azure AD (Entra ID) using their Equans credentials
2. **Token Issuance:** Azure AD issues a JWT token containing user identity and claims
3. **Frontend → Backend:** Frontend includes this JWT in `Authorization: Bearer <jwt>` header
4. **Backend Validation:** Rust backend validates the JWT token:
   - Verifies signature using Azure AD public keys
   - Checks token expiration
   - Validates issuer and audience
   - Extracts user identity and roles
5. **Authorization:** Backend enforces role-based access control (RBAC) based on Azure AD groups

### Why This Approach:
- **Enterprise Standard:** Aligns with Equans infrastructure and security policies
- **SSO Integration:** Users authenticate with existing Equans credentials
- **User-Level Security:** Each request is tied to an authenticated user
- **RBAC Support:** Can enforce different access levels (viewer, admin, etc.)
- **Audit Trail:** All API calls are traceable to specific users
- **Token Security:** Tokens are short-lived and cannot be forged

## Implementation Overview

### Backend (Rust)
- Use `jsonwebtoken` crate for JWT validation
- Configure Azure AD tenant ID, client ID, and audience in environment variables
- Implement middleware to validate JWT on every protected endpoint
- Extract user claims for authorization decisions
- Return HTTP `401 Unauthorized` for invalid/missing tokens
- Return HTTP `403 Forbidden` for authenticated but unauthorized requests

### Frontend (React)
- Use Microsoft Authentication Library (MSAL) for React
- Implement Azure AD login flow
- Store JWT tokens securely (memory only, not localStorage)
- Automatically include JWT in API requests
- Handle token refresh automatically
- Redirect to login on token expiration

### Environment Variables
```bash
# Azure AD Configuration (Backend)
AZURE_AD_TENANT_ID=<equans-tenant-id>
AZURE_AD_CLIENT_ID=<backend-app-registration-id>
AZURE_AD_AUDIENCE=api://equans-operational-insights

# Frontend Configuration
REACT_APP_AZURE_AD_CLIENT_ID=<frontend-app-registration-id>
REACT_APP_AZURE_AD_AUTHORITY=https://login.microsoftonline.com/<tenant-id>