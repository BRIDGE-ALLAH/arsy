-- Deliberately disabled examples only. This is not evidence of owned capacity.
INSERT INTO regions (id, code, name, location, status)
VALUES (
  'a8aa0000-0000-4000-8000-000000000001',
  'example-unverified',
  'Example region (not assigned)',
  'Not assigned - placeholder only',
  'planned'
)
ON CONFLICT DO NOTHING;

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
ON CONFLICT DO NOTHING;
