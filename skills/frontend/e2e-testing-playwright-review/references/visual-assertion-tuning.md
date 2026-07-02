# Visual Assertion Tuning (toHaveScreenshot)

Use this reference when reviewing or tuning `toHaveScreenshot`/`toMatchSnapshot` options (`animations`, `mask`, `maskColor`, `stylePath`, `maxDiffPixelRatio`, `maxDiffPixels`, `threshold`) or diagnosing visual-diff flakiness. For flakiness caused by locators/waits rather than visual noise, that is a fixtures/timing question, not a visual-assertion-tuning one.

> Version note: `toHaveScreenshot` option names below are drawn from the current `PageAssertions`/`LocatorAssertions` API reference. Verify exact defaults (e.g. whether `animations` defaults to `'disabled'` in the installed version) against Context7 (`/microsoft/playwright`) or the official API reference before citing a default value as fact.

## Officially grounded option set

Per the official `PageAssertions.toHaveScreenshot` / `LocatorAssertions.toHaveScreenshot` API reference, the documented tuning options include:

- **`animations`** -- whether to allow or disable animations in the screenshot; the framework's internal screenshot implementation passes `animations: helper.options.animations ?? 'disabled'`, i.e. animations are suppressed by default unless explicitly overridden.
- **`mask`** -- an array of `Locator`/`ElementHandle` values to mask out of the comparison (paired with `maskColor` to control the mask's fill color).
- **`stylePath`** -- a path to a CSS file injected into the page before the screenshot is taken, for style-level suppression of non-deterministic elements (e.g. hiding a live clock or a carousel via CSS) that isn't practical to mask element-by-element.
- **`clip`** / **`fullPage`** -- region-of-capture controls, independent of the diff-tolerance controls below.
- **Diff-tolerance controls**: `maxDiffPixels` (absolute pixel count allowed to differ), `maxDiffPixelRatio` (ratio of differing pixels, 0-1), and `threshold` (a per-pixel color-difference threshold for the comparator). These can be set per-assertion call or globally in `playwright.config.ts` under `expect.toHaveScreenshot` / `expect.toMatchSnapshot`.
- Both `toHaveScreenshot` (web-first, waits for two consecutive identical screenshots before comparing -- i.e. it has its own visual-stability wait built in) and `toMatchSnapshot` (compares an already-captured `page.screenshot()` buffer) support the diff-tolerance options; they differ in whether the wait-for-stability behavior is built in.

## Non-negotiable design rules

1. **Fix the non-determinism before widening tolerance.** If a diff is caused by a specific known-dynamic region (a timestamp, a live counter, an animated element, ad/embed content), the documented, targeted fix is `mask` (or `stylePath` for CSS-level suppression) on that region -- not a global increase to `maxDiffPixelRatio`/`maxDiffPixels`/`threshold`. Widening global tolerance first hides real regressions in the rest of the page along with the intended noise.
2. **Do not disable `animations` masking as a blanket policy without checking whether the default is already `'disabled'`.** Since the built-in implementation defaults `animations` to `'disabled'` unless overridden, an explicit `animations: 'allow'` (or equivalent override) in a config/test is itself worth flagging and asking why -- it re-introduces a documented source of visual flakiness.
3. **A widened `maxDiffPixelRatio`/`maxDiffPixels` needs a stated reason tied to a specific region or rendering variance (e.g. font anti-aliasing differences across OS/CI runners), not a round number picked to make CI green.** An unexplained tolerance value is a signal the underlying flake was never diagnosed.
4. **Global `expect.toHaveScreenshot`/`expect.toMatchSnapshot` config in `playwright.config.ts` sets the default for the whole suite; a per-call override should be justified by why that specific assertion needs different tolerance than the suite default**, not applied broadly as a workaround for one flaky test that then silently loosens every other screenshot assertion using the same call site pattern.
5. **`omitBackground` and `scale` affect what the baseline image actually contains** (transparency handling, DPI scaling); changing either after a baseline was captured invalidates the existing baseline and requires a deliberate baseline-update pass, not an incidental config tweak.

## Response discipline

When reviewing visual-assertion config, cite the specific option and value found (with file reference), state whether a wider tolerance is targeted (paired with `mask`/`stylePath` for a named dynamic region) or global/unexplained, and label the API-shape claim as `documentation-based` (grounded via Context7/official API reference this session) versus `inference`.
