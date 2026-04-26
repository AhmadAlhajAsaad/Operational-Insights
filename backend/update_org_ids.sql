-- Update org_ids for persons based on CSV data
-- This script extracts org_id from the Persons CSV and updates the database

-- Create temporary table to hold CSV data
CREATE TEMP TABLE temp_person_org_mapping (
    person_id VARCHAR(50),
    org_id VARCHAR(50)
);

-- Load data from CSV (run this with psql \copy command)
-- \copy temp_person_org_mapping FROM '/workspace/Excel/Persons Feb 17 2026.csv' WITH (FORMAT csv, HEADER true, DELIMITER ',');

-- Update persons table with org_id from CSV
UPDATE persons p
SET org_id = t.org_id
FROM temp_person_org_mapping t
WHERE p.person_id = t.person_id
  AND t.org_id IS NOT NULL
  AND t.org_id != '';

-- Verify the update
SELECT
    COUNT(*) as total,
    COUNT(org_id) as with_org_id,
    COUNT(*) - COUNT(org_id) as without_org_id
FROM persons;

-- Show distribution by org_id
SELECT
    o.org_id,
    o.name as org_name,
    COUNT(p.person_id) as person_count
FROM organizations o
LEFT JOIN persons p ON p.org_id = o.org_id
GROUP BY o.org_id, o.name
ORDER BY person_count DESC
LIMIT 20;

-- Cleanup
DROP TABLE temp_person_org_mapping;
