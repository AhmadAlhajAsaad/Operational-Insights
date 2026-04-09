# GDPR & Data Protection Testing

**Status:** Draft  
**Date:** 2026-02-11  
**Author(s):** DP-DevEx-Platform Team

---

## 1. Overview

This document defines test cases for ensuring GDPR compliance and data protection requirements are met across the Equans Operational Insights platform.

---

## 2. GDPR Compliance Requirements

| GDPR Article | Requirement            | Test Approach                              |
| ------------ | ---------------------- | ------------------------------------------ |
| Art. 5       | Data minimization      | Verify only necessary data collected       |
| Art. 17      | Right to erasure       | Test data deletion workflows               |
| Art. 20      | Data portability       | Test data export functionality             |
| Art. 25      | Privacy by design      | Review data handling in code               |
| Art. 32      | Security of processing | Penetration testing, encryption validation |

---

## 3. Data Protection Test Cases

| Test ID  | Category  | Test Case                        | Expected Result                 |
| -------- | --------- | -------------------------------- | ------------------------------- |
| GDPR-001 | Logging   | Check logs for PII               | No unmasked email addresses     |
| GDPR-002 | Logging   | Check error messages             | No sensitive data exposed       |
| GDPR-003 | Storage   | Verify encryption at rest        | Database encryption enabled     |
| GDPR-004 | Transit   | Verify encryption in transit     | HTTPS only, TLS 1.2+            |
| GDPR-005 | Retention | Data older than retention period | Automatically purged            |
| GDPR-006 | Export    | User data export request         | Complete data package generated |
| GDPR-007 | Deletion  | User deletion request            | All user data removed           |

---

## 4. Data Masking Validation

### Email Masking Test

```rust
#[test]
fn test_email_masking() {
    let email = "user@example.com";
    let masked = mask_email(email);
    assert_eq!(masked, "u***@e***.com");
    assert!(!masked.contains("user"));
}
```

### Log Output Validation

| Data Type  | Raw Value             | Expected Masked Output |
| ---------- | --------------------- | ---------------------- |
| Email      | `john.doe@equans.com` | `j***@e***.com`        |
| IP Address | `192.168.1.100`       | `192.168.x.x`          |
| User ID    | `user-12345`          | `user-*****`           |
| API Token  | `ghp_xxxxxxxxxxxx`    | `ghp_***`              |

---

## 5. Right to Erasure Testing

### Test Scenario: Complete User Data Deletion

**Preconditions:**

- User account exists with associated data
- User has license allocation history

**Steps:**

1. Submit deletion request for user
2. Wait for processing (async operation)
3. Verify data removal

**Verification Checklist:**

- [ ] User record removed from `users` table
- [ ] User removed from `team_members` table
- [ ] License history anonymized or deleted
- [ ] Audit logs preserved (with anonymized user reference)
- [ ] External systems notified (if applicable)

---

## 6. Data Portability Testing

### Test Scenario: User Data Export

**Steps:**

1. User requests data export
2. System generates export package

**Expected Export Contents:**

```json
{
  "user": {
    "id": "user-12345",
    "email": "user@example.com",
    "created_at": "2025-01-01T00:00:00Z"
  },
  "team_memberships": [...],
  "license_allocations": [...],
  "activity_log": [...]
}
```

**Validation:**

- [ ] All user data included
- [ ] Format is machine-readable (JSON)
- [ ] Export generated within 30 days of request
- [ ] Download link expires after use

---

## 7. Encryption Validation

### At-Rest Encryption

| Component    | Encryption Method | Verification                       |
| ------------ | ----------------- | ---------------------------------- |
| PostgreSQL   | TDE/AES-256       | Check `pg_settings` for encryption |
| Backups      | AES-256           | Verify backup encryption settings  |
| File Storage | AES-256           | Check volume encryption            |

### In-Transit Encryption

| Connection         | Protocol | Verification                    |
| ------------------ | -------- | ------------------------------- |
| Client → Frontend  | TLS 1.2+ | SSL Labs scan                   |
| Frontend → Backend | TLS 1.2+ | Certificate validation          |
| Backend → Database | TLS 1.2  | `sslmode=require` in connection |

---

## 8. Data Minimization Testing

### Collection Audit

| Data Point Collected | Business Justification  | Retention Period |
| -------------------- | ----------------------- | ---------------- |
| User email           | Identity, notifications | Account lifetime |
| License usage        | Billing, analytics      | 2 years          |
| Login history        | Security audit          | 1 year           |
| IP addresses         | Security monitoring     | 90 days          |

### Validation Tests

| Test ID | Scenario                               | Expected Result           |
| ------- | -------------------------------------- | ------------------------- |
| MIN-001 | API response contains only needed data | No extra PII in responses |
| MIN-002 | Database stores only required fields   | Schema matches spec       |
| MIN-003 | Logs contain minimal PII               | Only masked identifiers   |

---

## 9. Consent Management Testing

| Test ID | Scenario                   | Expected Result                           |
| ------- | -------------------------- | ----------------------------------------- |
| CON-001 | First login consent prompt | User must accept before proceeding        |
| CON-002 | Consent withdrawal         | Data processing stopped                   |
| CON-003 | Consent audit trail        | All consent changes logged with timestamp |

---

## 10. Third-Party Data Sharing

### Vendor Data Handling

| Vendor    | Data Shared     | Purpose            | DPA Status |
| --------- | --------------- | ------------------ | ---------- |
| Atlassian | User IDs, usage | License management | Signed     |
| GitHub    | User IDs, usage | License management | Signed     |
| JFrog     | Usage metrics   | License management | Signed     |

**DPA = Data Processing Agreement**

---

## Related Documents

- [Auth Testing](auth-testing.md)
- [TR-001-Performance-Security-Standards](../../Technical-Requirements/TR-001-Performance-Security-Standards.md)
- [Test Strategy](../strategy/test-strategy.md)
