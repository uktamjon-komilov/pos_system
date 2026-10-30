# Legacy "Eatables" (Meals) section — analysis

Source: `jowi_pos/src/renderer/src/sections/eatables` (React + Gravity UI 5.31 +
Tailwind + the in-house "metallic" kit in `metallicUI/`).

All pixel values below are **CSS px as written in the legacy code**. (The
Electron window also calls `webContents.setZoomFactor(0.75)`; the Slint port
does not copy that: it uses these values as logical px and lets Slint apply
the monitor's display scale, which keeps rendering sharp at every zoom.)

## 1. What the section actually is

The section is **not** a menu editor. Dishes, categories and modifiers are
created in the back-office; the POS only lets staff *operate* them:

| Can do in legacy                                   | Cannot do in legacy                      |
|----------------------------------------------------|------------------------------------------|
| browse nested categories, dishes, dish variants    | create / rename / delete a dish          |
| search dishes / modifiers by name                  | assign a dish to a category              |
| toggle **Go-list** and **Stop-list** of a dish     | attach / detach modifier groups          |
| set a portion **limit** (or "unlimited") on a dish / variant / modifier | edit price, image, description |
| put a modifier on the stop-list (hides it)         | reorder anything                         |
| create / delete **comment templates** of a dish    |                                          |
| read the dish description ("Информация о блюде")   |                                          |

The brief mentions "create/edit/delete meal, assign category, attach modifier
groups". None of these exist in legacy, so per the "do not invent features"
rule they are **not built**; they are listed in the suggestions at the end.
The create/edit/delete actions that *do* exist (comment templates, limits,
go/stop toggles) all work against the mock repository.

## 2. Screens and how they connect

```
Sidebar (104px) ── "Блюда" ──► EatablesPage  (route /eatables, index → /dishes)
                                  │
             ┌────────────────────┴───────────────────────┐
     tab "Блюда" (/dishes)                       tab "Модификаторы" (/modifiers)
     DishesSection                               ModifiersSection
     ├─ TopNavigation                            ├─ TopNavigation
     ├─ DishesListMenu (9/12 width)              ├─ modifier grid (9/12 width)
     │    ├─ "Меню": category grid               └─ ModifiersDetails (3/12)
     │    ├─ "<category>|Все блюда": dish grid        ├─ NotSelected (empty)
     │    ├─ "Варианты блюд": variant grid             ├─ limit card ⇄ ModifierLimitForm
     │    └─ empty text / infinite-scroll loader      └─ stop-list card
     └─ DishesDetails (3/12, bg #DFDFDF)
          ├─ NotSelected (empty)
          └─ horizontal 2-page slider
               page 1: name, DishQuickInfo, DishIngredientsBadge (►),
                       DishStockControl (limit card ⇄ DishLimitForm,
                       Go-list, Stop-list), DishCommentsTemplate
               page 2: DishIngredients ("Информация о блюде" + ✕)
```

There are no modal dialogs or drawers. "Dialogs" are inline swaps (limit card ⇄
limit form) and one anchored popup (numeric keypad under the limit input). The
dish-info "drawer" is a horizontal smooth-scroll inside the details column.

Switching tab, pressing Home, or entering the section resets the shared store
(`setDefaults`: search "", no dish, no variation, empty category stack).

## 3. Layout and real sizes

Shell (`SectionsLayout`):
- sidebar: width 104, bg `#1A1A1A`, padding-top 28, side padding 8, gap 8.
  Logo row 53 high. Items: 64 high, radius 12, icon 20, label caption-1 (9px),
  gap 8, text `#FFFFFFBF`; hover/active bg `#FFFFFF26`, border `#FFFFFF4D`, text white.
- content: padding 20 top/bottom/right, bg `#1A1A1A`; inner panel radius 24,
  bg `#C2C2C2`, border 1 `#6B849926`, clips children.
- section row: `gap 1px` between list column (75%) and details column (25%),
  so a 1px `#C2C2C2` hairline shows between them.

TopNavigation (above the list, height 76 = 16+44+16):
- two `#DFDFDF` blocks with 2px gap; padding 16.
- block 1: SegmentedControl xl "light" — tabs "Блюда" / "Модификаторы".
- block 2 (flex 1, gap 16): [◄|►] joined pair (44×44 each, ► always disabled),
  gap 8, [⌂] 44×44, then search input (xl, 44 high, flex 1, magnifier 16 on the right).

Dish list column: `gap 16` below the nav, scrollable, scrollbar hidden.
- "Меню" block: padding 0 16 16, border-bottom 1 `#7E7E7E`; title header-2
  complementary; grid **4 columns**, gap 4, margin-top 16.
- dishes block: padding 16 (top 0 when no categories), border-top 1 white
  when categories are shown (together = engraved double line); same grid.
- variants block: padding 0 16; title "Варианты блюд"; same grid.
- empty: header-1 secondary, centered: `В категории "<name|Все блюда>" нет блюд`.
- infinite scroll: page size 12; sentinel 80 high with Gravity Loader m.

Modifier list column: padding 0 16; title "Модификаторы" header-2
complementary; grid gap 4, columns **6** (≥1800 css px viewport),
**5** (1536–1799), **4** (<1536).

Details column: bg `#DFDFDF`, padding 16, gap 16 (dish) / 8 (modifier).

Cards:
| element | size / spacing |
|---|---|
| DishCard | h 200, padding 16, radius 30, border 6 `#C2C2C2`, bg `#EEEEEE`, selected: 3px outline `#2D2D2D`, pressed: scale .95 |
| CategoryCard | padding 16, radius 24, border 1 `#C2C2C2`, bg `#EEEEEE`, name 18/600 max 2 lines + 16 bottom margin |
| ModifierCard | as DishCard but radius 24 |
| MetallicCard (details) | padding 16, radius 24 (comment chips: 8×12, radius 12, no shadow) |
| limit form (dish) | Gravity Card filled, padding 16, radius 16; buttons 38 high, radius 16 |
| icon tile (go/stop) | 32×32, radius 12, icon 16 (modifier: 12) |
| Switch "l" (overridden) | 56×32, radius 17, knob 26, checked = green gradient |
| MetallicButton xl | 44 high, radius 16, icon 20, text 15/400 |
| numeric keypad | min width 400, 3 columns, keys 72 high, header-2 digits, "Готово" xl button radius 8 |

## 4. Design tokens actually used

Colors:
| token | value | used for |
|---|---|---|
| display bg | `#1A1A1A` | sidebar and outer frame |
| panel bg | `#C2C2C2` | content panel, card borders |
| surface | `#DFDFDF` | nav blocks, details column, gray button, active tab plate |
| card | `#EEEEEE` | cards, white button, inputs |
| text primary | `rgba(0,0,0,.85)` | default text |
| text complementary | `rgba(0,0,0,.70)` | section titles, comments |
| text secondary | `rgba(0,0,0,.50)` | captions, empty-list text |
| text hint | `rgba(0,0,0,.30)` | NotSelected text, gray button icon, disabled |
| brand / text-brand | `#FFBE5C` / `#BD8E4B` | Go-list bolt icon |
| gold gradient | `#FFBE5C → #B37922`, border `#BD8E4B` | primary MetallicButton |
| green gradient | `#30AA6E → #24A163`, border `#2D8B5D` | switch on |
| switch off | `rgba(0,0,0,.15)` (hover `.25`) | switch track |
| danger | `#E9033A` | stop-list tile & badge |
| purple | `#844DBB` | modifier stop-list tile |
| warning bg / text | `rgba(255,190,92,.3)` / `#BD5C0A` | "unlimited" alert |
| dark tile | `rgba(0,0,0,.85)` | Go-list tile |
| divider | `rgba(0,0,0,.10)`, `#7E7E7E`, `#7F7F7F` | separators, pills, plate border |
| selected outline | `#2D2D2D` | selected card |
| label neutral | `rgba(0,0,0,.05)` | go-list / limit badges |

Typography (Gravity, font **Inter** 400/600 from Google Fonts):
| variant | size/line | weight |
|---|---|---|
| caption-1 | 9/12 | 400 |
| caption-2 | 11/16 | 400 |
| body-1, body-short | 13/18, 13/16 | 400 |
| body-2 | 15/20 | 400 |
| subheader-1 / -2 / -3 | 13/18, 15/20, 17/24 | 600 |
| header-1 / -2 | 20/24, 24/28 | 600 |
| card name override | 16 (dish), 18 (category) | 600 |

Radii: 3 (label xs), 4 (stop badge), 6 (modifier pill), 8 (pill, keypad button),
10 (alert), 12 (tile, chip, tab plate, sidebar item), 16 (buttons, inputs,
segmented control), 24 (cards, panel), 30 (dish card).

Shadows ("metallic"): buttons/inputs/segmented control
`2px 2px 4px #0000000D` + `-2px -2px 4px #FFFFFF80`. Cards declare both but
the CSS overrides itself, so cards only get the white `-2px -2px 4px #FFFFFF80`.

Icons (Gravity Icons, 16×16 viewBox): chevron-left/right, house, magnifier,
xmark, pencil, thunderbolt-fill, circle-exclamation-fill, circle-xmark-fill,
eye, clock, arrow-left, square-list-ul, circle-dollar, gear; in-house SVGs:
DishIconFilled, DIshIcon2 (64px empty-state), DishIconLight, HallIcon,
KitchenIcon, Logo.

## 5. Component states

| component | states |
|---|---|
| DishCard / ModifierCard | normal, selected (outline), pressed (scale .95); badges: go-list, stop-list, limit "N шт.", cooking time; variant shows `size_name`; dish with variants shows "N варианта" |
| CategoryCard | pressed; "N блюда" + chevron, or "Нет блюда" without chevron |
| MetallicButton | normal, hover (flat: text primary), pressed, disabled (back/forward), loading (comment buttons) |
| SegmentedControl | selected plate, unselected text `#000000B2` |
| Switch | off, on (green), hover darker track |
| Text input | empty with placeholder, focused (opens keypad / on-screen keyboard) |
| "Добавить" | disabled while comment is blank, loading while saving |
| Details column | empty (NotSelected), dish, variant, info page |
| Limit card | view (Установлено / Осталось) ⇄ edit form; "unlimited" alert; hidden for `weight_based` dishes |
| Stop-list card | off; on → red tile + divider + info text |
| Modifier stop card | off ("Стоп лист", ✕); on ("Показывать модификатор", eye, purple) |
| List | categories+dishes, dishes only, variants only, empty text, loading next page |
| Loading / error | first load shows nothing; errors are not surfaced in this section |

## 6. Data

Dish (`IDishEntity`): id, name, price, type ("dish" | "variant"), size_name,
variant_count, cooking_time (minutes), description, image (unused),
is_in_go_list, is_in_stop_list, is_limited, limit_count, is_visible,
measurement, production_type ("weight_based" hides limit UI).

Category: id, name, button_color (unused), children_count, dishes_count.

Modifier: id, name, price, measurement, is_in_stop_list, is_visible,
is_limited, limit_count, dishes[] (linked dishes — fetched but never shown).

Comment template: id, comment (belongs to a dish, not a variant).

Validation / rules found in code:
- comment: blank (trimmed) comment cannot be added; field clears after add.
- Go-list and Stop-list are mutually exclusive (turning one on turns the other off).
- `is_limited = true` actually means **unlimited** (alert "Установлено
  бесконечное количество порций", input hidden, `limit_count` saved as null).
- limit input accepts keypad text (digits and "."), no client-side check.
- modifier stop-list sets `is_visible = !stop`.
- "Осталось" shows the same number as "Установлено" (no remaining count in API).
- price: thousands separated by spaces; 0 → "0"; missing → "N/A".
- cooking time label: `"<n> час"` if n ≥ 60 else `"<n> минут"` (no conversion).

## 7. User flows

1. Open section → dishes tab, root: "Меню" (top categories) + "Все блюда".
2. Tap category → push on stack, list shows its sub-categories and dishes; ◄ enabled.
3. Tap dish → selected outline, details column shows it. If it has variants,
   the list switches to "Варианты блюд" (dish + its variants); ◄ leaves variants.
4. Toggle Go-list / Stop-list → saved immediately, badges update on the card.
5. Pencil → limit form: toggle unlimited, or tap input → keypad → Готово → Сохранить.
6. Type comment → Добавить → chip appears; ⓧ on a chip deletes it.
7. "Информация о блюде ►" slides to description page; ✕ slides back.
8. Search (any tab) resets category stack and selection and filters by name.
9. ⌂ resets everything. Tab "Модификаторы" → grid; tap → details; limit / stop-list.

## 8. Not mappable to Slint 1:1, and substitutes

| legacy | Slint substitute |
|---|---|
| on-screen QWERTY keyboard (`react-simple-keyboard`) for search/comment | not ported (shared component outside this section); physical / OS touch keyboard is used |
| `flex-wrap` row of comment chips | Slint 1.16's FlexboxLayout is experimental-only; chips stack one per row |
| two stacked box-shadows | two stacked rectangles, each with one `drop-shadow` |
| CSS line-height | Slint has no line-height; heights are fixed via layout, text uses font metrics |
| `line-clamp-2` | `wrap: word-wrap` + `overflow: elide` with a 2-line max height |
| `outline` (outside border) | extra rectangle drawn 3px outside the card |
| smooth horizontal scroll snap | animated `x` of a two-page row |
| infinite scroll IntersectionObserver | Flickable `viewport-y` near-bottom check calling `load-more` |
| Gravity Popup anchored to input | absolutely positioned keypad panel under the input, closed by "Готово" |
| CSS breakpoints for modifier columns | column count computed from window width in Slint |
| react-query cache + refetch | on a worker thread: comment changes re-query the list; stock and limit updates patch state from the entity the repository returns |
| permission check `edit_dishes` + error toast | not ported (no auth in UI-only scope); listed as next step |
