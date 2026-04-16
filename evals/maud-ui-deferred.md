# Deferred Components — maud-ui

Components from shadcn/ui that are too complex for the initial build wave.
Each requires significant JS, server cooperation, or external dependencies.

## Deferred (10 components)

| Component | Complexity | Reason | Approach when ready |
|-----------|-----------|--------|-------------------|
| Calendar | High | Full date grid, month/year nav, locale formatting | Port a minimal date picker; consider chrono for date math |
| Carousel | High | Touch/swipe, infinite scroll, autoplay, indicators | Start with CSS scroll-snap, add JS for controls |
| Chart | High | Needs a charting library or canvas/SVG rendering | Wrapper around lightweight JS chart lib |
| Combobox | Medium | Autocomplete + filtering on top of Select | Extend existing Select with text input + filter |
| Command | High | Cmd+K palette with fuzzy search, sections, shortcuts | Complex JS; consider it a compound component |
| Data Table | High | Sorting, filtering, pagination, column resize | Build on top of Table + Pagination components |
| Date Picker | High | Calendar + input + popover composition | Compose Calendar + Popover + Input |
| Menubar | Medium | Horizontal menu with keyboard nav between top-level items | Extend Menu with horizontal layout + roving tabindex |
| Navigation Menu | High | Mega-menu with multi-level dropdowns, viewport-aware | Complex positioning + accessibility |
| Resizable | High | Drag-to-resize panels with min/max constraints | Needs pointer events, requestAnimationFrame |

## Also not included (shadcn-specific)

| Component | Reason |
|-----------|--------|
| Direction | React-specific context provider for RTL/LTR |
| Item | Internal React primitive, not a standalone component |
| Sonner | Third-party toast library; we have our own Toast |
| Sidebar | Application-level layout, not a primitive component |
| Sheet | Same as our Drawer component |

## Current coverage

- **Implemented:** 48 components
- **Shadcn total:** ~57 components
- **Coverage:** ~84% (excluding React-specific ones like Direction/Item)
- **Deferred:** 10 complex components for future waves
