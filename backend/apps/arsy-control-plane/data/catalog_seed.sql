-- Deliberately disabled examples only. This is not evidence of owned capacity.
INSERT INTO regions (id, code, name, location, status)
VALUES (
  'a8aa0000-0000-4000-8000-000000000001',
  'example-unverified',
  'Example region (not assigned)',
  'Not assigned - placeholder only',
  'planned'
)
ON CONFLICT (code) DO UPDATE
SET name = EXCLUDED.name,
    location = EXCLUDED.location,
    status = EXCLUDED.status,
    updated_at = NOW();

INSERT INTO compute_offerings (
  id, region_id, code, name, capability_key,
  vcpu_count, memory_mib, storage_gib, image_id, status
)
SELECT
  'a8aa0000-0000-4000-8000-000000000002'::uuid,
  id,
  'example-small',
  'Example Small (disabled)',
  'compute.vm.create.v1',
  1, 1024, 10, 'example-image-not-available', 'unverified'
FROM regions
WHERE code = 'example-unverified'
ON CONFLICT (region_id, code) DO UPDATE
SET name = EXCLUDED.name,
    capability_key = EXCLUDED.capability_key,
    vcpu_count = EXCLUDED.vcpu_count,
    memory_mib = EXCLUDED.memory_mib,
    storage_gib = EXCLUDED.storage_gib,
    image_id = EXCLUDED.image_id,
    status = EXCLUDED.status,
    updated_at = NOW();
