# RebrnG Design Contract

Status: frozen baseline for Phase 7

Scope: this file is the front-end and AI implementation contract for the Phase 7 ledger UI. It does not replace gameplay specs. It translates the existing ledger UI, technical architecture, and canon-worldview constraints into visual and interaction rules that implementation work must follow.

Reference method, not skin:

- Use the `DESIGN.md` style of explicit tokens plus rationale as a documentation method.
- Borrow high-density information discipline from TUI design references, but do not make the game a terminal UI.
- Borrow CJK layout discipline from Japanese / CJK UI contract projects, but keep the product voice Chinese-first.
- Do not copy any public brand template, marketing-page design, or generic AI-generated front-end style.

## Core Direction

The interface is a cold ledger for survival under clan order.

It should feel like:

- A cultivation account book.
- A debt and causality register.
- A worn field dossier kept under pressure.
- A tool for choosing what to sacrifice next.

It must not feel like:

- A SaaS dashboard.
- A purple AI product landing page.
- A neon cyber terminal.
- A pure decorative ink-painting skin.
- A cozy life sim journal.

## Visual Tokens

### Color

Use low-saturation paper, soot, aged earth, blue-gray, dark bronze, and cinnabar risk.

- `--rg-ink`: #1d1812 - primary text, button base, hard dividers.
- `--rg-ink-muted`: #5f5444 - secondary text and muted metadata.
- `--rg-paper`: #eee2c6 - main paper surface.
- `--rg-paper-aged`: #d7c49b - worn ledger panels.
- `--rg-bamboo-shadow`: #6c765f - Qingmao mountain pressure, neutral risk.
- `--rg-blue-gray`: #5b6873 - rumor, trace, night, uncertainty.
- `--rg-bronze`: #a17a3d - action affordance, route emphasis.
- `--rg-cinnabar`: #9d2f22 - danger, injury, exposure, debt collection.
- `--rg-black-soil`: #2a241c - deep background and command surfaces.

Forbidden defaults:

- Purple-on-white AI SaaS palettes.
- Rainbow gradients.
- Glassmorphism as the main surface language.
- Pure black high-contrast terminal styling.
- Clean white dashboard cards without ledger texture or world tone.

### Typography

Chinese text is the priority.

- Primary Chinese serif: `Noto Serif SC`, `Source Han Serif SC`, `Songti SC`, serif.
- UI numerals and compact IDs may use `Noto Sans Mono`, `Cascadia Mono`, monospace.
- Do not use default Arial / Inter-only stacks as the main visual voice.
- Body text should remain readable at dense information levels.
- English IDs must be visually secondary to Chinese labels.

### Surfaces

Surfaces should read as ledger layers, not floating app cards.

- Main shell: aged paper over dark earth or fogged mountain backdrop.
- Page panels: thin ink or bronze borders, small corner wear, restrained texture.
- Active pressure panels: use cinnabar accents, not animation spam.
- Disabled actions: faded ink, clear reason nearby if possible.

### Spacing

The UI is dense but not cramped.

- Main desktop shell target width: about 1200-1360px.
- Persistent status strip: compact, always visible.
- Page grid: 2-3 columns on desktop, 1 column on narrow screens.
- Critical values should align in ledger rows, not scatter in prose.

## Layout Contract

### Persistent Status Strip

The top status strip must always expose:

- Current day and visible period.
- Window type and window id.
- AP.
- Current node.
- Exposure.
- Debt pressure.
- Injury and active encounter state.

This strip is not a decorative header. It is the player's survival instrument.

### Fixed Ledger Pages

Phase 7 must organize these page groups as parts of one ledger:

- Scene page.
- Causality ledger.
- Node map.
- Resources and debts.
- Relationship situation placeholder.
- Aperture / cultivation page.
- Build page.
- Rumors and clues page.

Future pages such as aperture management, Treasure Yellow Heaven trade, formations, and Gu Houses must be imaginable as new ledger leaves, not a different product shell.

### Crisis Information

Pressure information must float forward when relevant:

- Exposure rises.
- Debt becomes collectible.
- Injury compresses AP.
- Active encounter blocks ordinary action.
- Black market access changes risk.
- A route closes or becomes costly.

Do not bury these in secondary tabs.

## Component Contract

### Action Buttons

Buttons submit `ActionCommand` only. They never create rule effects locally.

Action button states:

- Available: ink base with bronze or paper contrast.
- Risky: visible cinnabar accent and short cost hint.
- Disabled: faded, with a readable reason when possible.
- Encounter decision: stronger contrast than ordinary actions.

### Ledger Rows

Use ledger rows for repeated state facts:

- Label.
- Visible value.
- Optional risk marker.
- Optional source or last change.

Do not turn every state into a generic card with a title and paragraph.

### Narrative Text

Scene text should be calm and cold.

- Second person is allowed when presenting player-facing events.
- Avoid modern jokes, meta commentary, cozy encouragement, and heroic power fantasy.
- Keep danger concrete: price, debt, injury, exposure, route closure.

## CJK Text Rules

- Source files must be UTF-8.
- Phase 7 should fix existing mojibake in UI source while implementing UI, but this freezing package does not edit UI code.
- Do not use `word-break: break-all` as a blanket rule for Chinese prose.
- Chinese labels should not be replaced by English system terms.
- English IDs such as `academy_gate` may appear only as secondary debug-like metadata.
- Mixed Chinese, number, and ID rows must keep stable alignment.

## Accessibility And Performance

- Text contrast must remain readable on aged paper surfaces.
- Risk colors cannot be the only information channel.
- Motion should be rare and meaningful: page reveal, pressure pulse, or encounter emphasis.
- UI must remain responsive while Rust resolves actions.
- No remote AI call may block a click or page transition.

## Anti-Slop Checklist

Before merging Phase 7 UI work, check:

- Does the screen look like a ledger system rather than a web dashboard?
- Can the player see time, AP, node, exposure, debt, injury, and encounter pressure at a glance?
- Do page groups feel like one account book?
- Are crisis states visually stronger than ordinary metadata?
- Is Chinese text readable and not garbled?
- Does the UI avoid purple SaaS, generic gradients, and decorative-only ink painting?
- Does React only consume projections and submit commands?

## Implementation Boundary

This file freezes style direction and front-end contract. It does not define:

- Final art assets.
- Exact pixel-perfect layout.
- Complete component library.
- Complete Gu list.
- Rules formulas.
- Content writing for every event.

Those belong to later implementation and content passes.
