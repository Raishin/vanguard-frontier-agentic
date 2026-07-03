// Injects an ad-network snippet whose URL parameter names the campaign.
function renderCampaignBanner(campaignNameFromQueryString) {
  // campaignNameFromQueryString is read straight from location.search and
  // concatenated into markup written directly into the document stream.
  document.write('<div class="banner">' + campaignNameFromQueryString + '</div>');
}
