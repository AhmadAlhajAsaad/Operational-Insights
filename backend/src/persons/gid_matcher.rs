//! GID Matching Service (FR-005)
//!
//! Extracts and matches Global IDs (GID) for persons based on email addresses
//! and vendor identifiers. Calculates confidence scores for automatic matching.

use chrono::Utc;
use regex::Regex;
use serde_json::json;
#[allow(unused_imports)]
use tracing::{info, warn};

use super::types::Person;

/// GID extraction result
#[derive(Debug, Clone)]
pub struct GidMatch {
    pub gid: String,
    pub confidence: i32,
    pub extraction_method: String,
    pub matching_metadata: serde_json::Value,
}

/// GID Matcher service
pub struct GidMatcher {
    /// Regex for email prefix extraction
    email_regex: Regex,
}

impl GidMatcher {
    #[allow(clippy::unwrap_used)]
    pub fn new() -> Self {
        Self {
            email_regex: Regex::new(r"^([a-zA-Z0-9._-]+)@").unwrap(),
        }
    }

    /// Extract and match GID for a person
    ///
    /// Extracts GID from email prefix and calculates confidence based on:
    /// - Email prefix extraction (+50 base)
    /// - Local ID match (+30)
    /// - Vendor identifiers match (+20)
    pub fn match_person(&self, person: &Person) -> Option<GidMatch> {
        // Extract GID from email
        let gid = self.extract_gid_from_email(&person.email)?;

        // Calculate confidence score
        let confidence = self.calculate_confidence(person, &gid);

        // Determine extraction method
        let extraction_method = self.determine_method(person, &gid);

        // Build metadata
        let matching_metadata = self.build_metadata(person, &gid, confidence);

        Some(GidMatch {
            gid,
            confidence,
            extraction_method,
            matching_metadata,
        })
    }

    /// Extract GID from email prefix
    ///
    /// Examples:
    /// - thomas.wagensonner@equans.com -> thomas.wagensonner
    /// - john.doe@gmail.com -> john.doe
    fn extract_gid_from_email(&self, email: &str) -> Option<String> {
        self.email_regex
            .captures(email)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_lowercase())
    }

    /// Calculate confidence score (0-100)
    ///
    /// "Matched" betekent: we hebben een bestaand ID voor deze persoon.
    ///
    /// Scoring:
    /// - Person heeft een niet-auto-generated ID (person_id zonder "AUTO_" prefix): 100 (MATCHED)
    /// - Person heeft auto-generated ID ("AUTO_" prefix): 50 (PENDING - moet later matched worden)
    /// - Email prefix extracted: +30 (kan helpen voor toekomstige matching)
    /// - Local ID matches: +20
    /// - Vendor identifier matches: +10 per vendor
    ///
    /// Status thresholds:
    /// - >= 100: MATCHED (heeft bestaand ID)
    /// - 50-99: PENDING (auto-generated ID, moet later matched worden)
    /// - < 50: UNMATCHED (geen ID, geen matching gegevens)
    fn calculate_confidence(&self, person: &Person, gid: &str) -> i32 {
        // Check if person has a real ID (not auto-generated)
        // Matched GID = heeft een daadwerkelijk ID helemaal links
        if !person.person_id.starts_with("AUTO_") && !person.person_id.is_empty() {
            // Person heeft een bestaand ID ? MATCHED
            return 100;
        }

        // Person heeft AUTO_ ID of geen ID ? moet nog gematched worden
        let mut confidence = 0;

        // Base score: we kunnen GID extraheren uit email
        if !gid.is_empty() {
            confidence += 30;
        }

        // Local ID match (+20)
        if let Some(local_id) = &person.local_id {
            if local_id.to_lowercase().contains(gid) {
                confidence += 20;
            }
        }

        // Vendor identifier matches (meer vendors = hogere confidence)
        if let Some(vendor_ids) = &person.vendor_identifiers {
            // Check GitHub username
            if let Some(github_username) = vendor_ids
                .get("github")
                .and_then(|g| g.get("username"))
                .and_then(|u| u.as_str())
            {
                if github_username.to_lowercase() == gid {
                    confidence += 10;
                }
            }

            // Check Atlassian email
            if let Some(atlassian_email) = vendor_ids
                .get("atlassian")
                .and_then(|a| a.get("email"))
                .and_then(|e| e.as_str())
            {
                if let Some(atlassian_gid) = self.extract_gid_from_email(atlassian_email) {
                    if atlassian_gid == gid {
                        confidence += 10;
                    }
                }
            }

            // Check Jira/Confluence identifiers (als object keys bestaan)
            if vendor_ids.get("jira").is_some() {
                confidence += 5;
            }
            if vendor_ids.get("confluence").is_some() {
                confidence += 5;
            }
        }

        confidence.min(99) // Cap at 99 - alleen echte IDs krijgen 100
    }

    /// Determine extraction method used
    fn determine_method(&self, person: &Person, gid: &str) -> String {
        // Check various matching sources
        let mut sources = vec!["email_prefix"];

        if let Some(local_id) = &person.local_id {
            if local_id.to_lowercase().contains(gid) {
                sources.push("local_id");
            }
        }

        if let Some(vendor_ids) = &person.vendor_identifiers {
            if vendor_ids.get("github").is_some() {
                sources.push("github");
            }
            if vendor_ids.get("atlassian").is_some() {
                sources.push("atlassian");
            }
        }

        sources.join("+")
    }

    /// Build matching metadata for debugging and audit
    fn build_metadata(&self, person: &Person, gid: &str, confidence: i32) -> serde_json::Value {
        json!({
            "gid": gid,
            "confidence": confidence,
            "email": person.email,
            "local_id": person.local_id,
            "matched_at": Utc::now().to_rfc3339(),
            "sources": {
                "email_match": self.extract_gid_from_email(&person.email) == Some(gid.to_string()),
                "local_id_match": person.local_id.as_ref().is_some_and(|lid| lid.to_lowercase().contains(gid)),
                "vendor_match": person.vendor_identifiers.is_some(),
            }
        })
    }

    /// Batch process multiple persons
    pub fn match_batch(&self, persons: &[Person]) -> Vec<(String, Option<GidMatch>)> {
        persons
            .iter()
            .map(|person| {
                let person_id = person.person_id.clone();
                let gid_match = self.match_person(person);
                (person_id, gid_match)
            })
            .collect()
    }

    /// Get statistics about GID matching results
    ///
    /// Status classificatie:
    /// - MATCHED (100): Heeft bestaand ID (niet AUTO_)
    /// - PENDING (30-99): Heeft AUTO_ ID maar wel matching informatie
    /// - UNMATCHED (<30): Geen bruikbare matching informatie
    pub fn get_match_stats(&self, persons: &[Person]) -> GidMatchStats {
        let matches = self.match_batch(persons);

        let total = matches.len();
        let matched = matches
            .iter()
            .filter(|(_, m)| m.as_ref().is_some_and(|gm| gm.confidence >= 100))
            .count();
        let pending = matches
            .iter()
            .filter(|(_, m)| {
                m.as_ref()
                    .is_some_and(|gm| gm.confidence >= 30 && gm.confidence < 100)
            })
            .count();
        let unmatched = total - matched - pending;

        GidMatchStats {
            total,
            matched,
            pending,
            unmatched,
            match_rate: if total > 0 {
                (matched as f64 / total as f64) * 100.0
            } else {
                0.0
            },
        }
    }
}

impl Default for GidMatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for GID matching results
#[derive(Debug, Clone, serde::Serialize)]
pub struct GidMatchStats {
    pub total: usize,
    pub matched: usize,
    pub pending: usize,
    pub unmatched: usize,
    pub match_rate: f64,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn create_test_person(email: &str, local_id: Option<String>) -> Person {
        Person {
            id: 1,
            person_id: "TEST001".to_string(),
            first_name: "Test".to_string(),
            last_name: "Person".to_string(),
            email: email.to_string(),
            local_id,
            language: None,
            billing_location: None,
            country: None,
            job_title: None,
            department: None,
            manager: None,
            start_date: None,
            org_id: None,
            status: "Active".to_string(),
            source: None,
            gid: None,
            gid_confidence: None,
            gid_extraction_method: None,
            last_matched_at: None,
            matching_metadata: None,
            vendor_identifiers: None,
            atlassian_account_id: None,
            atlassian_link_status: None,
            atlassian_linked_at: None,
            atlassian_link_method: None,
            github_login: None,
            github_account_id: None,
            github_username: None,
            github_link_status: None,
            github_linked_at: None,
            github_linked_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_extract_gid_from_email() {
        let matcher = GidMatcher::new();

        assert_eq!(
            matcher.extract_gid_from_email("thomas.wagensonner@equans.com"),
            Some("thomas.wagensonner".to_string())
        );

        assert_eq!(
            matcher.extract_gid_from_email("john.doe@gmail.com"),
            Some("john.doe".to_string())
        );

        assert_eq!(
            matcher.extract_gid_from_email("Test_User-123@example.org"),
            Some("test_user-123".to_string())
        );
    }

    #[test]
    fn test_confidence_calculation() {
        let matcher = GidMatcher::new();

        // Person met bestaand ID ? MATCHED (100)
        let mut person1 = create_test_person("thomas.wagensonner@equans.com", None);
        person1.person_id = "CCJ183".to_string(); // Bestaand ID, niet AUTO_
        let match1 = matcher.match_person(&person1).unwrap();
        assert_eq!(match1.confidence, 100); // Heeft bestaand ID ? matched

        // Person met AUTO_ ID ? PENDING (< 100)
        let mut person2 = create_test_person("thomas.wagensonner@equans.com", None);
        person2.person_id = "AUTO_12345".to_string(); // Auto-generated ID
        let match2 = matcher.match_person(&person2).unwrap();
        assert!(match2.confidence < 100); // Auto ID ? not fully matched
        assert!(match2.confidence >= 30); // Heeft wel email ? pending

        // Person met bestaand ID + local_id ? MATCHED (100)
        let mut person3 = create_test_person(
            "thomas.wagensonner@equans.com",
            Some("thomas.wagensonner@equans.com".to_string()),
        );
        person3.person_id = "TW001".to_string(); // Real ID
        let match3 = matcher.match_person(&person3).unwrap();
        assert_eq!(match3.confidence, 100); // Real ID = matched
    }

    #[test]
    fn test_gid_status_thresholds() {
        let matcher = GidMatcher::new();

        // MATCHED: Person heeft bestaand ID
        let mut matched_person =
            create_test_person("test@equans.com", Some("test@equans.com".to_string()));
        matched_person.person_id = "TS001".to_string(); // Real ID
        let matched = matcher.match_person(&matched_person).unwrap();
        assert_eq!(matched.confidence, 100); // Full match door bestaand ID

        // PENDING: Person heeft AUTO_ ID
        let mut pending_person = create_test_person("test@equans.com", None);
        pending_person.person_id = "AUTO_test".to_string(); // Auto-generated
        let pending = matcher.match_person(&pending_person).unwrap();
        assert!(pending.confidence >= 30 && pending.confidence < 100); // Pending range

        // UNMATCHED: Person heeft geen matching info
        let mut unmatched_person = create_test_person("unknown@example.com", None);
        unmatched_person.person_id = "AUTO_xyz".to_string();
        let unmatched = matcher.match_person(&unmatched_person).unwrap();
        assert!(unmatched.confidence < 50); // Low confidence
    }
}
