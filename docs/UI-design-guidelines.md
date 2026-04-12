# ADR-00X: UI Design Guidelines Based on Equans Corporate Style

## Status

Accepted

## Context

The Operational Insights Dashboard is developed as part of the Equans SLS Digital Platforms environment.
As the dashboard will be used internally by stakeholders such as DevOps Forge, Product Owners, Team Leads, and Finance, it must align with the Equans corporate identity to ensure consistency, professionalism, and usability.

During design reviews with stakeholders (including Viktor and Brian), it was emphasized that:

- The dashboard should follow Equans design standards
- UI consistency is as important as functional correctness
- Data visualisation must be clear, accessible, and suitable for the type of data shown

Therefore, official Equans branding guidelines for colors and typography are adopted as a non-functional design requirement.

## Decision

The dashboard UI will strictly follow the Equans Corporate Style Guide for:

- Color usage
- Color hierarchy and opacity
- Typography

This decision applies to:

- Dashboard layouts
- Charts and data visualisations
- Labels, titles, and UI text
- Backgrounds and accent elements

## Design Rules Adopted

### 1. Color Strategy

#### Main Corporate Colors

The following colors must dominate the UI and form the visual foundation:

- **Dark Blue**
  RGB: (0, 36, 57)
  HEX: #002439

- **Dark Green**
  RGB: (0, 129, 99)
  HEX: #008163

- **Turquoise Green**
  RGB: (112, 189, 149)
  HEX: #70BD95

- **White**
  RGB: (255, 255, 255)
  HEX: #FFFFFF

These colors are used for:

- Page backgrounds
- Headers
- Navigation
- Primary UI elements

#### Accompanying (Secondary) Colors

One accompanying color may be selected per screen or dashboard to add visual rhythm, for example:

- **Azure Blue**
  RGB: (0, 89, 206)
  HEX: #0059CE

- **Orange**
  RGB: (255, 150, 0)
  HEX: #FF9600

- **Yellow**
  RGB: (255, 202, 0)
  HEX: #FFCA00

- **Apple Green**
  RGB: (118, 197, 18)
  HEX: #76C512

- **Violet**
  RGB: (200, 101, 255)
  HEX: #C865FF

- **Pink**
  RGB: (255, 0, 128)
  HEX: #FF0080

- **Lime Green**
  RGB: (183, 241, 0)
  HEX: #B7F100

- **Light Blue**
  RGB: (0, 222, 232)
  HEX: #00DEE8

**Rules:**

- Only one accompanying color per screen
- Must always be used together with a main corporate color
- Accompanying colors must never dominate the layout

#### Opacity Usage

To ensure visual hierarchy:

- **100% opacity** → primary data
- **60% opacity** → secondary data
- **20% opacity** → background or contextual elements

This is especially important for:

- Charts
- Legends
- Supporting UI elements

### 2. Data Visualisation Guidelines

- The type of chart must match the type of data
- No line charts for discrete or categorical data
- Bar charts for comparisons
- Line charts for continuous trends over time
- Color usage in charts follows the same main/accompanying color rules
- Accessibility is considered (contrast, readability)

### 3. Language

All user-facing text in the front-end interface **must be written in English**. This applies to:

- Page titles, headings, and subtitles
- Button labels, links, and navigation items
- Form labels, placeholders, and validation messages
- Status messages, progress indicators, and notifications
- Tooltips, help text, and informational copy
- Error messages and empty-state descriptions

**Rationale:** The Operational Insights Dashboard is used by international stakeholders across Equans. English ensures a consistent, unambiguous experience for all users regardless of their native language.

**Exceptions:** None. Localisation to other languages is out of scope.

---

### 4. Typography

The dashboard uses **Roboto** as the single typeface, in line with Equans standards.

- **Font:** Roboto
- **Source:** Google Fonts (free to use)
- **Styles allowed:** Light, Regular, Medium, Bold, Italic

**Usage:**

- Body text: Roboto Regular
- Titles: Roboto Medium or Bold

**Text colors:**

- White
- Dark Blue
- Black

Typography must support:

- Clear hierarchy
- Readability
- Consistent spacing and alignment

### 5. General Development Guidelines

Rules for code quality and layout implementation:

- Only use absolute positioning when necessary — opt for responsive layouts using flexbox and grid by default
- Refactor code as you go to keep code clean
- Keep file sizes small and put helper functions and components in their own files

### 6. Design System Guidelines

Rules for how the UI should align with the Equans design system:

- Use a base font-size of 14px
- Date formats should always be in the format "Jun 10"
- The bottom toolbar should only ever have a maximum of 4 items
- Never use the floating action button with the bottom toolbar
- Chips should always come in sets of 3 or more
- Don't use a dropdown if there are 2 or fewer options

#### Button

The Button component is a fundamental interactive element in our design system, designed to trigger actions or navigate users through the application. It provides visual feedback and clear affordances to enhance user experience.

**Usage**

Buttons should be used for important actions that users need to take, such as form submissions, confirming choices, or initiating processes. They communicate interactivity and should have clear, action-oriented labels.

**Variants**

- **Primary Button**
  - _Purpose:_ Used for the main action in a section or page
  - _Visual Style:_ Bold, filled with the primary brand color (`#002439` Dark Blue or `#008163` Dark Green)
  - _Usage:_ One primary button per section to guide users toward the most important action

- **Secondary Button**
  - _Purpose:_ Used for alternative or supporting actions
  - _Visual Style:_ Outlined with the primary color, transparent background
  - _Usage:_ Can appear alongside a primary button for less important actions

- **Tertiary Button**
  - _Purpose:_ Used for the least important actions
  - _Visual Style:_ Text-only with no border, using primary color
  - _Usage:_ For actions that should be available but not emphasized

---

## Consequences

### Positive

- Strong alignment with Equans brand identity
- Professional and consistent user experience
- Improved trust and acceptance by stakeholders
- Better accessibility and readability

### Trade-offs

- Less freedom in color experimentation
- Design choices must comply with predefined constraints

## Alternatives Considered

**Custom color palette**

- _Rejected:_ inconsistent with Equans branding

**Multiple secondary colors per screen**

- _Rejected:_ reduces clarity and violates style guide
