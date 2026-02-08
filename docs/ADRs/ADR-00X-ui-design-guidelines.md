# ADR-00X: UI Design Guidelines Based on Equans Corporate Style

## Status
Accepted

## Date
2026-02-04

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

### 3. Typography

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
- *Rejected:* inconsistent with Equans branding

**Multiple secondary colors per screen**
- *Rejected:* reduces clarity and violates style guide
