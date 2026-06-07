# Rollback Playbook: Azure Live App Service Slot Swap Guard

## Evidence-variable convention

Variables such as $AZURE_RESOURCE_GROUP_NAME, $APP_SERVICE_APP_NAME, $DEPLOYMENT_NAME, or $DEPLOYMENT_STACK_NAME are local operator placeholders. Do not commit real values, and redact them from shared evidence unless the change record explicitly allows disclosure.

## Immediate swap-back (standard rollback path)

The swap operation is symmetric — a second swap returns both slots to their original state.

```bash
# Verify current slot state before swapping back
az webapp show -g $AZURE_RESOURCE_GROUP_NAME -n $APP_SERVICE_APP_NAME \
  --query "{hostNames:properties.hostNames}"
az webapp show -g $AZURE_RESOURCE_GROUP_NAME -n $APP_SERVICE_APP_NAME --slot staging \
  --query "{hostNames:properties.hostNames}"

# Swap back: production → staging (reverts the original swap)
az webapp deployment slot swap \
  -g $AZURE_RESOURCE_GROUP_NAME -n $APP_SERVICE_APP_NAME \
  --slot staging \
  --target-slot production
```

## Verify after rollback

```bash
az webapp show -g $AZURE_RESOURCE_GROUP_NAME -n $APP_SERVICE_APP_NAME \
  --query "{state:properties.state, defaultHostName:properties.defaultHostName}"
# Check application health endpoint
curl -s https://${APP_SERVICE_APP_NAME}.azurewebsites.net/health
```

## Traffic shifting (partial rollback via A/B routing)

```bash
# Route 10% of traffic to staging while investigating
az webapp traffic-routing set -g $AZURE_RESOURCE_GROUP_NAME -n $APP_SERVICE_APP_NAME \
  --distribution staging=10

# Return all traffic to production
az webapp traffic-routing clear -g $AZURE_RESOURCE_GROUP_NAME -n $APP_SERVICE_APP_NAME
```

## Rollback limitations

- Slot swap is symmetric and reversible **only if you swap back before a second swap**.
- App settings with `slotSetting: false` were swapped — they will swap back.
- Any data written by the new code version to a shared database or storage is NOT rolled back by swapping.
- Log stream evidence must be captured before initiating a rollback; logs do not travel with slot state.
