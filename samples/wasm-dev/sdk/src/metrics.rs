// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

pub use _gen::tinykube_graph::processor::metrics::*;
#[cfg(not(test))]
mod _gen {
    // In normal case, use the metrics interface
    sdk_wit::wit_bindgen!("tinykube-graph:processor/metrics-use");
}
#[cfg(test)]
mod _gen {
    // In testing case, mock the tinykube_graph::processor::metrics interface
    sdk_wit::wit_bindgen!("tinykube-graph:processor/test");
    pub mod tinykube_graph {
        pub mod processor {
            pub mod metrics {
                #[derive(Debug)]
                pub enum CounterValue {
                    U64(u64),
                }
                #[derive(Debug)]
                pub enum HistogramValue {
                    U64(u64),
                    F64(f64),
                }
                #[derive(Debug)]
                pub enum MetricError {
                    IncompatibleType(String),
                    LockError(String),
                }
                #[derive(Debug)]
                pub struct Label {
                    pub key: String,
                    pub value: String,
                }
                #[allow(clippy::needless_pass_by_value)]
                pub fn add_to_counter(
                    name: String,
                    value: CounterValue,
                    labels: Option<&Vec<Label>>,
                ) -> Result<(), MetricError> {
                    println!("add_to_counter: {name} - {value:?} - {labels:?}");
                    match &name[..] {
                        "IncompatibleType" => {
                            Err(MetricError::IncompatibleType("IncompatibleType".to_owned()))
                        }
                        "LockError" => Err(MetricError::LockError("LockError".to_owned())),
                        _ => Ok(()),
                    }
                }
                #[allow(clippy::needless_pass_by_value)]
                pub fn record_to_histogram(
                    name: String,
                    value: HistogramValue,
                    labels: Option<&Vec<Label>>,
                ) -> Result<(), MetricError> {
                    println!("record_to_histogram: {name} - {value:?} - {labels:?}");
                    match &name[..] {
                        "IncompatibleType" => {
                            Err(MetricError::IncompatibleType("IncompatibleType".to_owned()))
                        }
                        "LockError" => Err(MetricError::LockError("LockError".to_owned())),
                        _ => Ok(()),
                    }
                }
            }
        }
    }
}

// Tests
#[cfg(test)]
mod tests {

    // Without mock there is following error:
    // misaligned pointer dereference: address must be a multiple of 0x8 but is 0x7f5d54000d64
    // thread caused non-unwinding panic. aborting.

    #[test]
    fn test_metrics_add_to_counter() {
        use crate::metrics::{add_to_counter, CounterValue, Label};
        let labels = vec![Label {
            key: "module".to_owned(),
            value: "module1/filter".to_owned(),
        }];
        let _ = add_to_counter("name".into(), CounterValue::U64(1), None);
        let result = add_to_counter("name".into(), CounterValue::U64(1), Some(&labels));
        assert!(result.is_ok());
        let result = add_to_counter(
            "IncompatibleType".into(),
            CounterValue::U64(1),
            Some(&labels),
        );
        assert!(result.is_err());
        let result = add_to_counter("LockError".into(), CounterValue::U64(1), Some(&labels));
        assert!(result.is_err());
    }

    #[test]
    fn test_metrics_record_gauge() {
        use crate::metrics::{record_to_histogram, HistogramValue, Label};
        let labels = vec![Label {
            key: "module".to_owned(),
            value: "module1/filter".to_owned(),
        }];
        let _ = record_to_histogram("name".into(), HistogramValue::U64(1), None);
        let _ = record_to_histogram("name".into(), HistogramValue::U64(1), Some(&labels));
        let result = record_to_histogram("name".into(), HistogramValue::F64(1.0), Some(&labels));
        assert!(result.is_ok());
        let result = record_to_histogram(
            "IncompatibleType".into(),
            HistogramValue::F64(1.0),
            Some(&labels),
        );
        assert!(result.is_err());
        let result =
            record_to_histogram("LockError".into(), HistogramValue::F64(1.0), Some(&labels));
        assert!(result.is_err());
    }
}
