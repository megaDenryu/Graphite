//! ノードフィールド・エッジ属性に `f64` のような `Eq` を実装できない型を
//! 使えることを確認する。
//!
//! v2 以降、ノード型・エッジ属性型はどちらもユーザーが `graph_schema!` の
//! 外で宣言する普通の struct であり、マクロは一切 derive を強制しない
//! (README「エッジ属性型に対する trait 要求」節参照)。そのため「`f64` を
//! 含む型に `Eq` を付けられない」という問題は、単に `Eq` を derive しない
//! という利用者側の選択で最初から解消している (このテストが確認するのは
//! むしろ「マクロが余計な derive を強制していない」こと自体)。

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Sensor {
    pub name: String,
}

/// ノード型。`f64` フィールドを持つため `Eq` は derive しない。
#[derive(Debug, Clone, PartialEq)]
pub struct Reading {
    pub value: f64,
}

/// `measured` エッジの属性。
#[derive(Debug, Clone, PartialEq)]
pub struct MeasuredEdge {
    pub confidence: f64,
}

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod Measurement {
    include!("generated/f64_attrs_measurement.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/f64_attrs_measurement.rs";
    schema Measurement {
        node Sensor;
        node Reading;

        edge Measured = (sensor: Sensor) -[measurement: MeasuredEdge]-> (reading: Reading);
    }
}

use Measurement::{Measured, MeasuredId, ReadingId, SensorId};

#[test]
fn f64をエッジ属性とノードフィールドに持つスキーマがコンパイル_構築できる() {
    let g = Measurement::Graph::create(|b| {
        b.sensor(
            SensorId("s1".to_string()),
            Sensor {
                name: "温度センサ".to_string(),
            },
        );
        b.reading(ReadingId("r1".to_string()), Reading { value: 23.5 });
        b.measured(
            MeasuredId("m1".to_string()),
            Measured::new(
                SensorId("s1".to_string()),
                ReadingId("r1".to_string()),
                MeasuredEdge { confidence: 0.95 },
            ),
        );
    })
    .expect("f64 フィールドを含むスキーマも正常に構築できるはず");

    let readings: Vec<_> = g
        .sensor_by_id(&SensorId("s1".to_string()))
        .unwrap()
        .measured_as_sensor()
        .collect();
    assert_eq!(readings.len(), 1);
    assert_eq!(readings[0].reading().value, 23.5);
    assert_eq!(readings[0].payload().confidence, 0.95);
}
