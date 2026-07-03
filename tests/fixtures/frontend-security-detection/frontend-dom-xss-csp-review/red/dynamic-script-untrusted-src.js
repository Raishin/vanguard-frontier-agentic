// Tag-manager style loader: fetch remote config, then inject a script
// whose src comes straight from that config with no allowlist check.
async function loadVendorScript() {
  const config = await fetch('/api/marketing-config').then((r) => r.json());

  const script = document.createElement('script');
  script.src = config.remoteScriptUrl;
  document.body.appendChild(script);
}

loadVendorScript();
