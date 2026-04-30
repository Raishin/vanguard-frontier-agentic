# Rollback Playbook: Azure Live App Service Slot Swap Guard

## Immediate swap-back (standard rollback path)

The swap operation is symmetric — a second swap returns both slots to their original state.

```bash
# Verify current slot state before swapping back
az webapp show -g <RESOURCE_GROUP> -n <APP_NAME> \
  --query "{hostNames:properties.hostNames}"
az webapp show -g <RESOURCE_GROUP> -n <APP_NAME> --slot staging \
  --query "{hostNames:properties.hostNames}"

# Swap back: production → staging (reverts the original swap)
az webapp deployment slot swap \
  -g <RESOURCE_GROUP> -n <APP_NAME> \
  --slot staging \
  --target-slot production
```

## Verify after rollback

```bash
az webapp show -g <RESOURCE_GROUP> -n <APP_NAME> \
  --query "{state:properties.state, defaultHostName:properties.defaultHostName}"
# Check application health endpoint
curl -s https://<APP_NAME>.azurewebsites.net/health
```

## Traffic shifting (partial rollback via A/B routing)

```bash
# Route 10% of traffic to staging while investigating
az webapp traffic-routing set -g <RESOURCE_GROUP> -n <APP_NAME> \
  --distribution staging=10

# Return all traffic to production
az webapp traffic-routing clear -g <RESOURCE_GROUP> -n <APP_NAME>
```

## Rollback limitations

- Slot swap is symmetric and reversible **only if you swap back before a second swap**.
- App settings with `slotSetting: false` were swapped — they will swap back.
- Any data written by the new code version to a shared database or storage is NOT rolled back by swapping.
- Log stream evidence must be captured before initiating a rollback; logs do not travel with slot state.
