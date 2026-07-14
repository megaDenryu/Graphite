//! フェーズ4 項目3: エッジ属性・ノードフィールドに `f64` のような `Eq` を
//! 実装できない型を使えることを確認する。
//!
//! 以前は生成されるノード struct / エッジ属性 struct が
//! `#[derive(Debug, Clone, PartialEq, Eq)]` だったため、`f64` フィールドを
//! 持つスキーマは `Eq` の derive に失敗しコンパイルできなかった (README
//! 「未決事項」節)。`Eq` を外し `PartialEq` のみ残すことで解消した。

#[rustfmt::skip]
graphite::graph_schema! {
    schema Measurement {
        node Sensor { name: String }
        node Reading { value: f64 }

        edge measured: Sensor -> Reading (0..*) { confidence: f64 };
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
            SensorId("s1".to_string()),
            ReadingId("r1".to_string()),
            MeasuredAttrs { confidence: 0.95 },
        );
    })
    .expect("f64 フィールドを含むスキーマも正常に構築できるはず");

    let readings = g.measured(&SensorId("s1".to_string()));
    assert_eq!(readings.len(), 1);
    assert_eq!(readings[0].0.value, 23.5);
    assert_eq!(readings[0].1.confidence, 0.95);
}
