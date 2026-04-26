-- Fix inconsistent Atlassian linking data
-- Problem: Some persons have a link status (e.g., 'linked_auto_local_id')
-- but atlassian_account_id is NULL. This causes the UI to show a warning.
--
-- Solution: Reset these persons to 'unlinked' status so they can be relinked.

BEGIN;

-- Show affected records before fix
SELECT COUNT(*) AS affected_records
FROM persons
WHERE atlassian_link_status NOT IN ('unlinked', 'no_atlassian_account')
  AND atlassian_account_id IS NULL;

-- Reset inconsistent links to unlinked
UPDATE persons
SET
    atlassian_link_status = 'unlinked',
    atlassian_linked_at = NULL,
    atlassian_link_method = NULL
WHERE atlassian_link_status NOT IN ('unlinked', 'no_atlassian_account')
  AND atlassian_account_id IS NULL;

-- Show fixed records
SELECT COUNT(*) AS fixed_records
FROM persons
WHERE atlassian_link_status = 'unlinked'
  AND atlassian_account_id IS NULL;

COMMIT;

-- After running this script, trigger a relink via API:
-- curl -X POST -H "Content-Type: application/json" -d '{}' http://localhost:8080/api/atlassian/link-persons
