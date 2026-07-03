// entry-server.js
import { renderToString } from '@vue/server-renderer'
import devalue from '@nuxt/devalue'

export async function render(url, pinia, app) {
  const appHtml = await renderToString(app)

  // devalue escapes </script>, <, and line-separator characters for safe
  // embedding inside an inline <script> tag.
  const serialized = devalue(pinia.state.value)
  const html = `<div id="app">${appHtml}</div>
<script>window.__pinia=${serialized}</script>`

  return html
}
