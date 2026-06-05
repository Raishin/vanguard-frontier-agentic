# Preflight Commands: Azure Live App Service Slot Swap Guard

Run these before initiating a slot swap. Paste sanitized output as evidence.

## 1. Confirm identity and App Service target

```bash
az account show --query "{subscription:id, name:name, user:user.name}"
az webapp show -g <resource-group-name> -n <app-name> \
  --query "{name:name, state:properties.state, hostNames:properties.hostNames}"
```

## 2. List all slots and their current traffic weights

```bash
az webapp deployment slot list -g <resource-group-name> -n <app-name> \
  --query "[].{name:name, state:properties.state}"
az webapp traffic-routing show -g <resource-group-name> -n <app-name>
```

## 3. Compare app settings between slots

```bash
az webapp config appsettings list -g <resource-group-name> -n <app-name> \
  --slot staging --query "[].{name:name, slotSetting:slotSetting}"
az webapp config appsettings list -g <resource-group-name> -n <app-name> \
  --query "[].{name:name, slotSetting:slotSetting}"
```

Pay special attention to `slotSetting: false` — those settings WILL swap with the slot.
Settings with `slotSetting: true` are slot-sticky and will NOT be swapped.

## 4. Check slot health before swap

```bash
az webapp show -g <resource-group-name> -n <app-name> --slot staging \
  --query "{state:properties.state, availabilityState:properties.availabilityState}"
# State must be "Running" and availabilityState must be "Normal" before swap
```

## 5. Review connection strings

```bash
az webapp config connection-string list -g <resource-group-name> -n <app-name> --slot staging \
  --query "[].{name:name, type:type, slotSetting:slotSetting}"
```


## 6. Start swap with preview, then validate before completion

```bash
az webapp deployment slot swap \
  -g <resource-group-name> -n <app-name> \
  --slot staging --target-slot production --action preview

# Complete only after validation succeeds
az webapp deployment slot swap \
  -g <resource-group-name> -n <app-name> \
  --slot staging --target-slot production --action swap

# Reset if validation fails
az webapp deployment slot swap \
  -g <resource-group-name> -n <app-name> \
  --slot staging --target-slot production --action reset
```
