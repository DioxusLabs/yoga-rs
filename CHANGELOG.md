# Changelog

## Unreleased

- Upgrade to Yoga commit [08250431e09bc4b864917e51f9e9d52998ee2dce](https://github.com/facebook/yoga/commit/08250431e09bc4b864917e51f9e9d52998ee2dce)
- `Errata` is now a bitflag newtype (so combinations such as `Errata::StretchFlexBasis | Errata::AbsolutePercentAgainstInnerSize` can be represented); added `ExperimentalFeature`
- Added `Config` wrappers: `set_errata`/`get_errata`, `set_experimental_feature_enabled`/`is_experimental_feature_enabled`, `set_point_scale_factor`/`get_point_scale_factor`, `set_use_web_defaults`/`get_use_web_defaults`
- Added `Node` wrappers: `get_layout_had_overflow`, `get_layout_raw_width`/`get_layout_raw_height`, `set_node_type`/`get_node_type`, `set_always_forms_containing_block`/`get_always_forms_containing_block`, `set_is_reference_baseline`/`is_reference_baseline`, `has_measure_func`, `has_baseline_func`, `get_has_new_layout`/`set_has_new_layout`, and the min-content APIs (`set_min_content_measure_func`, `has_min_content_measure_func`, `set_min_content_width`/`get_min_content_width`, `set_min_content_height`/`get_min_content_height`)
- Added `MaxContent`, `FitContent` and `Stretch` variants to `StyleUnit` (new Yoga units)
- `Node::get_gap`, `get_row_gap` and `get_column_gap` now return a `StyleUnit` (Yoga's getter returns a `YGValue`) and take `&self`
- Yoga is compiled with `-ffp-contract=off` so its float results do not depend on fused multiply-add contraction

## 0.5.0
Released: 17th Jan 2025

- Upgrade to Yoga  [v3.2.1](https://github.com/facebook/yoga/commit/042f5013152eb81c1552dec945b88f7b95ca350f) (committed on: 13th Dec 2024)
- Added `Absolute` variant to `PositionType` enum
- Added `Contents` variant to `Display` enum
- Support percentage gaps
- Support `BoxSizing`
- Updated dependencies:
  - `ordered_float` from `3.x` to `4.x`
  - `bindgen` from `0.63.x` to `0.71.x`

## 0.4.0
Released: Dec 20th 2022

- Upgrade to Yoga commit [ba27f9d1ecfa7518019845b84b035d3d4a2a6658](https://github.com/facebook/yoga/commit/ba27f9d1ecfa7518019845b84b035d3d4a2a6658) (committed on: 16th Dec 2022)
- Added `Static` variant to `PositionType` enum
- Added bindings for gap styles: `Gutter` enum and `Node.set_gap()` and `Node.get_gap()` functions.
- Updated dependencies:
  - `ordered_float` from `1.0.1` to `3.4.0`
  - `cc` from `1.0.17` to `1.0.78`
  - `bindgen` from `0.37.0` to `0.63.0`
  - `serde` and `serde_derive` from `1.0.27` to `1.0.151`


## 0.3.1
Released: Jan 16th 2019

- Add `Node.mark_dirty()`
- Update dependencies:
  - `ordered_float` from `0.5.0` to `1.0.1`


## 0.3.0
Released: 16 Oct 2018

- Initial tagged version