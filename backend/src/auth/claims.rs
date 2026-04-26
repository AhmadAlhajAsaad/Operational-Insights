//! JWT Claims types for Azure AD tokens

use serde::{Deserialize, Serialize};

/// Azure AD JWT Claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AzureAdClaims {
    /// Subject (user ID)
    pub sub: String,

    /// User principal name (email)
    #[serde(default)]
    pub upn: Option<String>,

    /// Preferred username
    #[serde(default)]
    pub preferred_username: Option<String>,

    /// Display name
    #[serde(default)]
    pub name: Option<String>,

    /// Email address
    #[serde(default)]
    pub email: Option<String>,

    /// Azure AD Object ID
    pub oid: String,

    /// Tenant ID
    pub tid: String,

    /// Audience (must match our app)
    pub aud: String,

    /// Issuer
    pub iss: String,

    /// Issued at (Unix timestamp)
    pub iat: i64,

    /// Not before (Unix timestamp)
    pub nbf: i64,

    /// Expiration (Unix timestamp)
    pub exp: i64,

    /// Azure AD groups (for RBAC)
    #[serde(default)]
    pub groups: Vec<String>,

    /// Roles assigned in app registration
    #[serde(default)]
    pub roles: Vec<String>,
}

impl AzureAdClaims {
    /// Get user identifier for logging (no PII in logs)
    pub fn user_id(&self) -> &str {
        self.upn
            .as_deref()
            .or(self.preferred_username.as_deref())
            .or(self.email.as_deref())
            .unwrap_or(&self.sub)
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r.eq_ignore_ascii_case(role))
    }

    /// Check if user is member of a specific group
    pub fn in_group(&self, group_id: &str) -> bool {
        self.groups.iter().any(|g| g == group_id)
    }

    /// Check if user has Admin role or is in admin group
    pub fn is_admin(&self, admin_group_id: Option<&str>) -> bool {
        self.has_role("Admin") || admin_group_id.map(|g| self.in_group(g)).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_claims() -> AzureAdClaims {
        AzureAdClaims {
            sub: "test-subject".to_string(),
            upn: Some("user@equans.com".to_string()),
            preferred_username: None,
            name: Some("Test User".to_string()),
            email: None,
            oid: "object-id".to_string(),
            tid: "tenant-id".to_string(),
            aud: "api://test".to_string(),
            iss: "https://login.microsoftonline.com/tenant/v2.0".to_string(),
            iat: 0,
            nbf: 0,
            exp: 9999999999,
            groups: vec!["group-1".to_string(), "admin-group".to_string()],
            roles: vec!["Viewer".to_string()],
        }
    }

    #[test]
    fn test_user_id_prefers_upn() {
        let claims = create_test_claims();
        assert_eq!(claims.user_id(), "user@equans.com");
    }

    #[test]
    fn test_has_role() {
        let claims = create_test_claims();
        assert!(claims.has_role("Viewer"));
        assert!(claims.has_role("viewer")); // case insensitive
        assert!(!claims.has_role("Admin"));
    }

    #[test]
    fn test_in_group() {
        let claims = create_test_claims();
        assert!(claims.in_group("group-1"));
        assert!(claims.in_group("admin-group"));
        assert!(!claims.in_group("unknown-group"));
    }

    #[test]
    fn test_is_admin_via_group() {
        let claims = create_test_claims();
        assert!(claims.is_admin(Some("admin-group")));
        assert!(!claims.is_admin(Some("other-group")));
        assert!(!claims.is_admin(None));
    }

    #[test]
    fn test_is_admin_via_role() {
        let mut claims = create_test_claims();
        claims.roles = vec!["Admin".to_string()];
        assert!(claims.is_admin(None));
    }
}
