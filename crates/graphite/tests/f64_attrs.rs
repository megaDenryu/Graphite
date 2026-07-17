//! ノードフィールド・エッジ属性に `f64` のような `Eq` を実装できない型を
//! 使えることを確認する。
//!
//! v2 以降、ノード型・エッジ属性型はどちらもユーザーが `graph_schema!` の
//! 外で宣言する普通の struct であり、マクロは一切 derive を強制しない
//! (README「エッジ属性型に対する trait 要求」節参照)。そのため「`f64` を
//! 含む型に `Eq` を付けられない」という問題は、単に `Eq` を derive しない
//! という利用者側の選択で最初から解消している (このテストが確認するのは
//! むしろ「マクロが余計な derive を強制していない」こと自体)。

/// ノードキー。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SensorId(pub String);

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Sensor {
    pub name: String,
}

/// ノードキー。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadingId(pub String);

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
graphite::graph_schema! {
    schema Measurement {
        node Sensor;
        node Reading;

        edge Measured = Sensor -[MeasuredEdge]-> Reading;
    }
}

#[test]
fn f64をエッジ属性とノードフィールドに持つスキーマがコンパイル_構築できる() {
    let g = Measurement::create(|b| {
        b.sensor(
            SensorId("s1".to_string()),
            Sensor {
                name: "温度センサ".to_string(),
            },
        );
        b.reading(ReadingId("r1".to_string()), Reading { value: 23.5 });
        b.measured(
            MeasuredId("m1".to_string()),
            Measured(
                SensorId("s1".to_string()),
                ReadingId("r1".to_string()),
                MeasuredEdge { confidence: 0.95 },
            ),
        );
    })
    .expect("f64 フィールドを含むスキーマも正常に構築できるはず");

    let readings = Measured::of(&g, &SensorId("s1".to_string()));
    assert_eq!(readings.len(), 1);
    assert_eq!(readings[0].0.value, 23.5);
    assert_eq!(readings[0].1.confidence, 0.95);
}
