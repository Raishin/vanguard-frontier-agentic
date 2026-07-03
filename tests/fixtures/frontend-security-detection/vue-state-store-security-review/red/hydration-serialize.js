// entry-server.js
import { renderToString } from '@vue/server-renderer'

export async function render(url, pinia, app) {
  const appHtml = await renderToString(app)

  // WRONG: naive JSON.stringify does not escape </script>, <, or U+2028/2029 —
  // a user-controlled state field can break out of the inline script context.
  const html = `<div id="app">${appHtml}</div>
<script>window.__pinia=${JSON.stringify(pinia.state.value)}</script>`

  return html
}
