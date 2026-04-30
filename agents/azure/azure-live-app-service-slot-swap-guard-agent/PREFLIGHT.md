# App Service Slot Swap — Preflight Commands

## 1. List all slots and current states

```bash
az webapp deployment slot list \
  --name <APP_NAME> \
  --resource-group <TARGET_RG> \
  --query "[].{name:name, state:state, host:defaultHostName}"
```

## 2. Audit sticky app settings (settings that do NOT swap with the slot)

```bash
az webapp config appsettings list \
  --name <APP_NAME> --slot staging \
  --resource-group <TARGET_RG> \
  --query "[?slotSetting==true].{name:name, value:value}"

az webapp config appsettings list \
  --name <APP_NAME> --slot production \
  --resource-group <TARGET_RG> \
  --query "[?slotSetting==true].{name:name, value:value}"
```

Flag any mismatch in sticky connection strings before proceeding.

## 3. Verify warmup and startup health

```bash
az webapp show \
  --name <APP_NAME> --slot staging \
  --resource-group <TARGET_RG> \
  --query "{state:state, usageState:usageState, siteConfig:siteConfig.autoHealEnabled}"

curl -I -s "https://<APP_NAME>-staging.azurewebsites.net/health" --max-time 30
```

## 4. Stage the swap-with-preview (does not complete the swap)

```bash
az webapp deployment slot swap \
  --name <APP_NAME> \
  --resource-group <TARGET_RG> \
  --slot staging \
  --target-slot production \
  --action preview
```

Validate the preview URL before committing.
