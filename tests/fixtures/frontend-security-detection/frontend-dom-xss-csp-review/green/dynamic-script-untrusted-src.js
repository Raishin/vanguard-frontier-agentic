// Tag-manager style loader: fetch remote config, but constrain the
// injected script's origin via a Trusted Types createScriptURL policy.
const allowedScriptOrigins = ['https://cdn.example.com'];

const scriptPolicy = trustedTypes.createPolicy('vendor-script-loader', {
  createScriptURL(url) {
    const parsed = new URL(url, location.href);
    if (!allowedScriptOrigins.includes(parsed.origin)) {
      throw new Error('Blocked script URL outside allowlist: ' + parsed.origin);
    }
    return url;
  },
});

async function loadVendorScript() {
  const config = await fetch('/api/marketing-config').then((r) => r.json());

  const script = document.createElement('script');
  script.src = scriptPolicy.createScriptURL(config.remoteScriptUrl);
  script.crossOrigin = 'anonymous';
  document.body.appendChild(script);
}

loadVendorScript();
