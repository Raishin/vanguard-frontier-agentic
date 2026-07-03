// Injects an ad-network snippet whose URL parameter names the campaign.
function renderCampaignBanner(campaignNameFromQueryString) {
  // The value is sanitized and inserted as a text node via a template,
  // never through the legacy document-stream write API.
  const banner = document.createElement('div');
  banner.className = 'banner';
  banner.textContent = campaignNameFromQueryString;
  document.body.appendChild(banner);
}
