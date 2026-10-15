extern crate yoga;

use yoga::{Config, Errata, ExperimentalFeature, Node, NodeType, StyleUnit};

#[test]
fn config_errata_round_trips_including_combinations() {
    let mut config = Config::new();
    assert_eq!(
        config.get_errata(),
        Errata::MinSizeUndefinedInsteadOfAuto | Errata::FlexFirstPassUsesRunningTotals
    );

    let errata = Errata::StretchFlexBasis | Errata::AbsolutePercentAgainstInnerSize;
    config.set_errata(errata);
    assert_eq!(config.get_errata(), errata);

    config.set_errata(Errata::All);
    assert_eq!(config.get_errata(), Errata::All);
    config.set_errata(Errata::None);
    assert_eq!(config.get_errata(), Errata::None);
}

#[test]
fn config_flags_round_trip() {
    let mut config = Config::new();
    assert!(!config.is_experimental_feature_enabled(ExperimentalFeature::FixFlexBasisFitContent));
    config.set_experimental_feature_enabled(ExperimentalFeature::FixFlexBasisFitContent, true);
    assert!(config.is_experimental_feature_enabled(ExperimentalFeature::FixFlexBasisFitContent));
    assert!(!config.is_experimental_feature_enabled(ExperimentalFeature::WebFlexBasis));

    config.set_point_scale_factor(2.625);
    assert_eq!(config.get_point_scale_factor(), 2.625);

    assert!(!config.get_use_web_defaults());
    config.set_use_web_defaults(true);
    assert!(config.get_use_web_defaults());
}

#[test]
fn node_flags_round_trip() {
    let mut node = Node::new();
    assert!(!node.get_always_forms_containing_block());
    node.set_always_forms_containing_block(true);
    assert!(node.get_always_forms_containing_block());

    assert_eq!(node.get_node_type(), NodeType::Default);
    node.set_node_type(NodeType::Text);
    assert_eq!(node.get_node_type(), NodeType::Text);

    assert!(!node.is_reference_baseline());
    node.set_is_reference_baseline(true);
    assert!(node.is_reference_baseline());

    assert!(!node.has_measure_func());
    assert!(!node.has_baseline_func());
    assert!(!node.has_min_content_measure_func());

    node.set_min_content_width(12.0);
    node.set_min_content_height(34.0);
    assert_eq!(node.get_min_content_width(), 12.0);
    assert_eq!(node.get_min_content_height(), 34.0);
}

#[test]
fn raw_layout_is_unrounded() {
    let mut config = Config::new();
    config.set_point_scale_factor(1.0);
    let mut root = Node::new_with_config(&mut config);
    root.set_width(StyleUnit::Point(10.4.into()));
    root.set_height(StyleUnit::Point(10.6.into()));
    root.calculate_layout(yoga::Undefined, yoga::Undefined, yoga::Direction::LTR);
    assert_eq!(root.get_layout_width(), 10.0);
    assert_eq!(root.get_layout_height(), 11.0);
    assert_eq!(root.get_layout_raw_width(), 10.4);
    assert_eq!(root.get_layout_raw_height(), 10.6);
    assert!(!root.get_layout_had_overflow());
}

#[test]
fn gap_getters_return_style_units() {
    let mut node = Node::new();
    node.set_gap(yoga::Gutter::Column, StyleUnit::Point(5.0.into()));
    node.set_row_gap(StyleUnit::Percent(10.0.into()));
    assert_eq!(node.get_column_gap(), StyleUnit::Point(5.0.into()));
    assert_eq!(node.get_row_gap(), StyleUnit::Percent(10.0.into()));
}
