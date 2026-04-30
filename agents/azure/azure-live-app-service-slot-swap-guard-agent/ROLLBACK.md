# App Service Slot Swap — Rollback Playbook

## Option 1: Reset a swap-with-preview (safest — no prod change yet)

```bash
az webapp deployment slot swap \
  --name <APP_NAME> \
  --resource-group <TARGET_RG> \
  --slot staging \
  --target-slot production \
  --action reset
```

This cancels the preview staging. No traffic was moved to the new version.

## Option 2: Re-swap back after a completed swap

```bash
# Swap prod back to staging (restores previous production code)
az webapp deployment slot swap \
  --name <APP_NAME> \
  --resource-group <TARGET_RG> \
  --slot production \
  --target-slot staging
```

Speed: swap-back completes in seconds (no warmup required as staging was already warm).

## Option 3: Emergency scale-down if app is crashing post-swap

```bash
az webapp stop --name <APP_NAME> --resource-group <TARGET_RG>
# Fix the issue, then:
az webapp start --name <APP_NAME> --resource-group <TARGET_RG>
```

## Verify production health after rollback

```bash
az webapp show \
  --name <APP_NAME> \
  --resource-group <TARGET_RG> \
  --query "{state:state, usageState:usageState}"

curl -I -s "https://<APP_NAME>.azurewebsites.net/health" --max-time 30
```
