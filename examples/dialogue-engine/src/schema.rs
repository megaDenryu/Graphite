//! スキーマ宣言 (`graph_schema!`) と、シナリオ本編・破損シナリオの
//! `graph!` リテラル本体。
//!
//! v3 (`docs/graph_literal_v3.md` §4) でハンドシェイクマクロを全廃したため、
//! `graph_schema!` と `graph!` を同一ファイルに置く必要は無くなった (`graph!`
//! が参照するのは通常の型・メソッドだけになったため、別モジュールから `use`
//! すれば足りる。実証は `crates/graphite/tests/graph_cross_module.rs`)。
//! この example では単に型定義とシナリオ本編が近くにあった方が読みやすい
//! という理由で同居させている。

// `-[積み荷式]->` 記法は rustfmt が「知らない構文」として誤整形しうるため、
// `graph!` を呼ぶ関数には個別に `#[rustfmt::skip]` を付ける
// (.claude/skills/proc-macro-dev/SKILL.md の注意通り)。

use graphite::Graph;

// ============================================================
// スキーマ宣言 (`docs/schema_v4.md`)
// ============================================================
//
// node Scene:  1 場面。話者と本文を持つ。
// node Ending: 1 エンディング。タイトルとエピローグ本文を持つ。
// edge Choice = Scene -[ChoiceEdge]-> Scene — 選択肢。制約なし (下記参照)。
// edge Finale = Scene -> Ending where each Scene: 0..1 — エンディングへの
//               到達。各シーンにつき高々1つの結末。

/// ノードキー。`graph_schema!` はこれも生成せず参照するだけ
/// (`docs/node_id_v4_2.md`)。`PartialOrd`/`Ord` は必須ではないが
/// (必須なのは `Debug, Clone, PartialEq, Eq, Hash` だけ)、`report.rs` が
/// 決定的な表示順のためにキーをソートする箇所がこのアプリ側の都合で
/// 要求している。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SceneId(pub String);

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub speaker: String,
    pub text: String,
}

/// ノードキー。`PartialOrd`/`Ord` は `report.rs` のソート表示のために
/// 要求している (`graph_schema!` 自体の要求ではない)。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EndingId(pub String);

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Ending {
    pub title: String,
    pub epilogue: String,
}

/// `Choice` 辺の積み荷 (選択肢のラベル文字列)。
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceEdge {
    pub label: String,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema DialogueGraph {
        node Scene;
        node Ending;

        // Choice に `where unique pair` を付けない理由: 同じ (from, to) の
        // 対に対して、文言 (ラベル) が異なる複数の選択肢が正当にありうる
        // 設計 (例: 別々の経緯で同じシーンへ合流する選択肢が2つあっても
        // おかしくない) ため、平行辺を積極的に許す。
        edge Choice = Scene -[ChoiceEdge]-> Scene;
        edge Finale = Scene -> Ending where each Scene: 0..1;
    }
}

// ============================================================
// 導出クエリ (README.md 「使用例3」節のパターン: 保存エッジ=フィールド,
// 導出エッジ=同一モジュール内の普通のメソッド)
// ============================================================

impl DialogueGraph {
    /// あるシーンから出ている選択肢一覧を `(行き先キー, 選択肢ラベル)` で返す。
    /// `Choice::of` は行き先の `Scene` 値 (キーではない) を返すため使えず、
    /// 生の辺 (キー付き) を走査する `Choice::iter` をフィルタして使う。
    pub fn scene_choices(&self, id: &SceneId) -> Vec<(SceneId, String)> {
        Choice::iter(self)
            .filter(|(_key, edge)| edge.from() == id)
            .map(|(_key, edge)| (edge.to().clone(), edge.payload().label.clone()))
            .collect()
    }

    /// choice 辺だけを汎用グラフ `Graph<SceneId, String, SceneId>` へ射影する。
    /// `reachable_from`/`has_cycle`/`path`/`filter_nodes` のような、図式グラフ
    /// (`graph_schema!`) には無いグラフアルゴリズムを使うための橋渡し。
    /// ノードの値には (使わないが) キー自身を積んでおく。辺の値には選択肢
    /// ラベルを積み、`route` コマンドでの表示に使う。
    ///
    /// 構築は `Scene` の集合と `Choice` 辺だけから機械的に決まるため、
    /// このシナリオが `DialogueGraph::create` を通過している時点で
    /// 重複キー・未知キーは有り得ず、`expect` で握り潰してよい。
    pub fn scene_graph(&self) -> Graph<SceneId, String, SceneId> {
        Graph::create(|b| {
            for id in Scene::ids(self) {
                b.node(id.clone(), id.clone());
            }
            for (_key, edge) in Choice::iter(self) {
                b.edge(edge.from().clone(), edge.to().clone(), edge.payload().label.clone());
            }
        })
        .expect("scene_graph の射影は DialogueGraph が既に検証済みなので必ず成功する")
    }

    /// このシーンに finale (エンディングへの到達) があるか。
    pub fn is_finale_scene(&self, id: &SceneId) -> bool {
        Finale::of(self, id).is_some()
    }

    /// このシーンに選択肢が 0 本、かつ finale も無いか (= デッドエンド)。
    pub fn is_dead_end(&self, id: &SceneId) -> bool {
        Choice::of(self, id).is_empty() && Finale::of(self, id).is_none()
    }
}

// ============================================================
// シナリオ本編: 「月面基地アルテミスIII、通信途絶」
// ============================================================
//
// SF ミステリー短編。月面研究基地との通信が途絶し、調査に向かった主人公が
// 基地内で寄生性の未知胞子 (S-7) による事故の痕跡を追う。
//
// 構造上の見どころ:
// - 合流: `central` (中央ホール) や `lower_hall` (地下ホール) は複数の経路
//   から到達する集合点。`seal_sacrifice` と `takashi_rescue` も別々の選択
//   から合流する。
// - ループ: `hangar_log` は自分自身へ戻る「もう一度調べる」選択肢を持つ
//   (自己ループ)。さらに `central <-> hangar/lab/quarters`、
//   `lower_hall <-> reactor/comms/control_room` は行って戻れる往復路であり、
//   グラフ全体は木ではなく循環を含む。
// - 4 種のエンディング (脱出 / 犠牲 / 真実 / 孤立=バッドエンド)。
//
// choice/finale エッジキーの命名規則 (v4 は辺も第一級キー付き要素なので、
// 全56本のchoice + 4本のfinale全てに一意なキーが要る):
// - choice: `c_<from>_<to>` (自己ループ・複数選択肢の場合は用途で接尾辞)
// - finale: `f_<from>` (1シーンにつき finale は高々1本なので from だけで一意)
// シーン名が長い箇所は読みやすさのため一貫した省略形を使う:
//   hangar_log→hlog, lab_samples→lsamples, lab_computer→lcomputer,
//   lab_echo→lecho, quarters_diary→qdiary, quarters_rooms→qrooms,
//   quarters_locked→qlocked, quarters_takashi→qtakashi,
//   lower_hatch→lhatch, lower_hall→lhall, control_room→croom,
//   control_analysis→canalysis, takashi_seal→tseal, takashi_rescue→trescue,
//   crisis_evacuate→cevac, crisis_seal→cseal, crisis_truth→ctruth,
//   crisis_freeze→cfreeze, shuttle_bay→sbay, seal_sacrifice→ssac,
//   truth_sent→tsent
#[rustfmt::skip]
pub fn build_story() -> Result<DialogueGraph, DialogueGraphViolation> {
    graphite::graph!(DialogueGraph {
        // --- 導入 ---
        start = Scene {
            speaker: "管制官ナオ".to_string(),
            text: "アルテミスIII基地との定期通信が3時間途絶えている。至急向かってくれ。".to_string()
        },
        arrival = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "エアロックのハッチが半開きのままだ。非常灯だけが点滅している。".to_string()
        },
        airlock = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "内部は静まり返っている。どこから調べる?".to_string()
        },

        // --- 中央ホール (合流点1) ---
        central = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "中央ホール。3方向への通路と、地下へのハッチがここに集まっている。".to_string()
        },

        // --- 格納庫ルート ---
        hangar = Scene {
            speaker: "整備士ケンタの記録".to_string(),
            text: "格納庫にローバーが一台足りない。勝手に持ち出した者がいるのか。".to_string()
        },
        hangar_log = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "運行記録には見知らぬ座標。基地の地図に無い、未登録クレーター付近だ。".to_string()
        },

        // --- 研究室ルート ---
        lab = Scene {
            speaker: "研究員レイのメモ".to_string(),
            text: "培養器が全て開けっ放しだ。何かが持ち出されている。".to_string()
        },
        lab_samples = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "保管庫の中身は空。ラベルには『S-7 未分類胞子』とある。".to_string()
        },
        lab_computer = Scene {
            speaker: "AI管理システム エコー".to_string(),
            text: "警告: S-7検体が漏出。隔離プロトコルは実行されませんでした。".to_string()
        },
        lab_echo = Scene {
            speaker: "AI管理システム エコー".to_string(),
            text: "漏出は3時間前、通信途絶と同時刻です。何者かが意図的に隔離を解除しました。".to_string()
        },

        // --- 居住区ルート ---
        quarters = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "居住区のドアが軒並み開いている。誰もいない。".to_string()
        },
        quarters_diary = Scene {
            speaker: "隊員ミサキの日誌".to_string(),
            text: "『クレーターで奇妙な発光体を見つけた。ケンタに報告する』…最後のページはここで途切れている。".to_string()
        },
        quarters_rooms = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "一部屋だけ、内側から鍵がかけられている。".to_string()
        },
        quarters_locked = Scene {
            speaker: "隊員タカシ(衰弱した声)".to_string(),
            text: "来るな…近づくな…もう手遅れなんだ…".to_string()
        },
        quarters_takashi = Scene {
            speaker: "隊員タカシ".to_string(),
            text: "S-7に感染した仲間が胞子を撒き散らす前に…頼む、隔壁を閉めてくれ…".to_string()
        },

        // --- 地下区画 (合流点2) ---
        lower_hatch = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "地下へのハッチは重く軋む。奥から機械音が響いている。".to_string()
        },
        lower_hall = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "地下区画。原子炉室と通信室、そして管制室への扉がある。".to_string()
        },
        reactor = Scene {
            speaker: "保守ログ".to_string(),
            text: "原子炉は自動停止モードに移行している。誰かが手動で停止させた形跡がある。".to_string()
        },
        comms = Scene {
            speaker: "通信士の最終記録".to_string(),
            text: "『応答不能、感染拡大中、封鎖を要請する』…その後、通信は途絶えている。".to_string()
        },
        control_room = Scene {
            speaker: "AI管理システム エコー".to_string(),
            text: "管制室へようこそ。ここから基地の全システムを制御できます。ご指示を。".to_string()
        },
        control_analysis = Scene {
            speaker: "AI管理システム エコー".to_string(),
            text: "分析完了。S-7胞子は寄生し正気を奪います。基地内の複数名が感染している可能性が高いです。".to_string()
        },

        // --- タカシを巡る分岐 (合流点3・4の起点) ---
        takashi_seal = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "タカシの願い通り、隔壁を閉める。彼の声が、静かになった。".to_string()
        },
        takashi_rescue = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "タカシを担ぎ上げ、共に脱出路を目指す。まだ間に合うかもしれない。".to_string()
        },

        // --- クライマックス分岐 ---
        crisis_evacuate = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "残った全員をシャトルへ誘導する。一刻の猶予もない。".to_string()
        },
        shuttle_bay = Scene {
            speaker: "生存者たち".to_string(),
            text: "全員……間に合った、のか。最後の警報が鳴り響く中、ハッチが閉まる。".to_string()
        },
        crisis_seal = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "隔壁の起動キーを握る。これを回せば感染区画ごと基地を封じ込められる…だがタカシがまだ中にいる。".to_string()
        },
        seal_sacrifice = Scene {
            speaker: "隊員タカシ(通信越し)".to_string(),
            text: "……ありがとう。基地は、任せた。".to_string()
        },
        crisis_truth = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "エコーに全記録を外部へ送信させる。真実だけは、消させない。".to_string()
        },
        truth_sent = Scene {
            speaker: "AI管理システム エコー".to_string(),
            text: "送信完了。これで地球はすべてを知ることになるでしょう。".to_string()
        },
        crisis_freeze = Scene {
            speaker: "主人公(独白)".to_string(),
            text: "決断できないまま、時間だけが過ぎていく。警報音が遠くなっていく…".to_string()
        },

        // --- エンディング ---
        ending_evacuate = Ending {
            title: "生存者、脱出".to_string(),
            epilogue: "基地は失われたが、命は繋がった。地球への帰路、誰も口を開かなかった。".to_string()
        },
        ending_sacrifice = Ending {
            title: "犠牲による静寂".to_string(),
            epilogue: "タカシの名は、基地の最終ログにだけ残った。静かな終わりだった。".to_string()
        },
        ending_truth = Ending {
            title: "真実の伝播".to_string(),
            epilogue: "地球は全てを知った。次の探査隊は、同じ過ちを繰り返さないだろう。".to_string()
        },
        ending_isolation = Ending {
            title: "沈黙する基地".to_string(),
            epilogue: "誰も決断しないまま、通信は完全に途絶えた。基地は今も月面で沈黙している。".to_string()
        },

        // ============================================================
        // 選択肢 (Choice) — 導入
        // ============================================================
        c_start_arrival = Choice(start -[ChoiceEdge { label: "基地へ急行する".to_string() }]-> arrival),
        c_arrival_airlock = Choice(arrival -[ChoiceEdge { label: "中へ入る".to_string() }]-> airlock),
        c_airlock_hangar = Choice(airlock -[ChoiceEdge { label: "格納庫を調べる".to_string() }]-> hangar),
        c_airlock_lab = Choice(airlock -[ChoiceEdge { label: "研究室を調べる".to_string() }]-> lab),
        c_airlock_quarters = Choice(airlock -[ChoiceEdge { label: "居住区を調べる".to_string() }]-> quarters),

        // --- 中央ホール: 3エリア + 地下への行き来 (合流点) ---
        c_central_hangar = Choice(central -[ChoiceEdge { label: "格納庫へ".to_string() }]-> hangar),
        c_central_lab = Choice(central -[ChoiceEdge { label: "研究室へ".to_string() }]-> lab),
        c_central_quarters = Choice(central -[ChoiceEdge { label: "居住区へ".to_string() }]-> quarters),
        c_central_lhatch = Choice(central -[ChoiceEdge { label: "地下区画へ続くハッチを開ける".to_string() }]-> lower_hatch),

        // --- 格納庫ルート (central との往復 + 自己ループ) ---
        c_hangar_hlog = Choice(hangar -[ChoiceEdge { label: "ローバーの運行記録を調べる".to_string() }]-> hangar_log),
        c_hangar_central = Choice(hangar -[ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central),
        c_hlog_retry = Choice(hangar_log -[ChoiceEdge { label: "もう一度記録を洗い直す".to_string() }]-> hangar_log),
        c_hlog_central = Choice(hangar_log -[ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central),

        // --- 研究室ルート ---
        c_lab_lsamples = Choice(lab -[ChoiceEdge { label: "サンプル保管庫を調べる".to_string() }]-> lab_samples),
        c_lab_lcomputer = Choice(lab -[ChoiceEdge { label: "研究用端末を調べる".to_string() }]-> lab_computer),
        c_lab_central = Choice(lab -[ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central),
        c_lsamples_lcomputer = Choice(lab_samples -[ChoiceEdge { label: "端末を調べる".to_string() }]-> lab_computer),
        c_lsamples_central = Choice(lab_samples -[ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central),
        c_lcomputer_lecho = Choice(lab_computer -[ChoiceEdge { label: "エコーに詳細を尋ねる".to_string() }]-> lab_echo),
        c_lcomputer_central = Choice(lab_computer -[ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central),
        c_lecho_lab = Choice(lab_echo -[ChoiceEdge { label: "研究室に戻る".to_string() }]-> lab),
        c_lecho_lhatch = Choice(lab_echo -[ChoiceEdge { label: "地下区画へ向かう".to_string() }]-> lower_hatch),

        // --- 居住区ルート ---
        c_quarters_qdiary = Choice(quarters -[ChoiceEdge { label: "日誌を調べる".to_string() }]-> quarters_diary),
        c_quarters_qrooms = Choice(quarters -[ChoiceEdge { label: "個室を順に見て回る".to_string() }]-> quarters_rooms),
        c_quarters_central = Choice(quarters -[ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central),
        c_qdiary_qrooms = Choice(quarters_diary -[ChoiceEdge { label: "個室を見て回る".to_string() }]-> quarters_rooms),
        c_qdiary_central = Choice(quarters_diary -[ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central),
        c_qrooms_qlocked = Choice(quarters_rooms -[ChoiceEdge { label: "鍵のかかった部屋をこじ開ける".to_string() }]-> quarters_locked),
        c_qrooms_central = Choice(quarters_rooms -[ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central),
        c_qlocked_qtakashi = Choice(quarters_locked -[ChoiceEdge { label: "話を聞く".to_string() }]-> quarters_takashi),
        c_qlocked_central = Choice(quarters_locked -[ChoiceEdge { label: "その場を離れる".to_string() }]-> central),
        c_qtakashi_tseal = Choice(quarters_takashi -[ChoiceEdge { label: "隔壁を閉める".to_string() }]-> takashi_seal),
        c_qtakashi_trescue = Choice(quarters_takashi -[ChoiceEdge { label: "彼を連れて避難する".to_string() }]-> takashi_rescue),
        c_qtakashi_lhatch = Choice(quarters_takashi -[ChoiceEdge { label: "地下区画へ急ぐ".to_string() }]-> lower_hatch),

        // --- タカシを巡る分岐の合流 ---
        c_tseal_lhall = Choice(takashi_seal -[ChoiceEdge { label: "地下ホールへ戻る".to_string() }]-> lower_hall),
        c_tseal_ssac = Choice(takashi_seal -[ChoiceEdge { label: "封鎖を完了させる".to_string() }]-> seal_sacrifice),
        c_trescue_lhall = Choice(takashi_rescue -[ChoiceEdge { label: "地下ホールへ急ぐ".to_string() }]-> lower_hall),
        c_trescue_croom = Choice(takashi_rescue -[ChoiceEdge { label: "管制室で状況を確認する".to_string() }]-> control_room),

        // --- 地下区画 (合流点 + 往復) ---
        c_lhatch_lhall = Choice(lower_hatch -[ChoiceEdge { label: "下りる".to_string() }]-> lower_hall),
        c_lhall_reactor = Choice(lower_hall -[ChoiceEdge { label: "原子炉室へ".to_string() }]-> reactor),
        c_lhall_comms = Choice(lower_hall -[ChoiceEdge { label: "通信室へ".to_string() }]-> comms),
        c_lhall_croom = Choice(lower_hall -[ChoiceEdge { label: "管制室へ".to_string() }]-> control_room),
        c_lhall_central = Choice(lower_hall -[ChoiceEdge { label: "中央ホールへ戻る".to_string() }]-> central),
        c_reactor_lhall = Choice(reactor -[ChoiceEdge { label: "地下ホールに戻る".to_string() }]-> lower_hall),
        c_comms_lhall = Choice(comms -[ChoiceEdge { label: "地下ホールに戻る".to_string() }]-> lower_hall),
        c_comms_croom = Choice(comms -[ChoiceEdge { label: "管制室へ".to_string() }]-> control_room),
        c_croom_canalysis = Choice(control_room -[ChoiceEdge { label: "感染源を分析する".to_string() }]-> control_analysis),
        c_croom_lhall = Choice(control_room -[ChoiceEdge { label: "地下ホールへ戻る".to_string() }]-> lower_hall),

        // --- クライマックス分岐 ---
        c_canalysis_cevac = Choice(control_analysis -[ChoiceEdge { label: "全員を退避させる".to_string() }]-> crisis_evacuate),
        c_canalysis_cseal = Choice(control_analysis -[ChoiceEdge { label: "感染区画を封鎖する".to_string() }]-> crisis_seal),
        c_canalysis_ctruth = Choice(control_analysis -[ChoiceEdge { label: "真相を記録し外部に送信する".to_string() }]-> crisis_truth),
        c_canalysis_cfreeze = Choice(control_analysis -[ChoiceEdge { label: "何も決められず立ち尽くす".to_string() }]-> crisis_freeze),

        c_cevac_sbay = Choice(crisis_evacuate -[ChoiceEdge { label: "シャトルへ急ぐ".to_string() }]-> shuttle_bay),
        c_cseal_ssac = Choice(crisis_seal -[ChoiceEdge { label: "隔壁を封鎖する".to_string() }]-> seal_sacrifice),
        c_cseal_trescue = Choice(crisis_seal -[ChoiceEdge { label: "タカシを助けに戻る".to_string() }]-> takashi_rescue),
        c_ctruth_tsent = Choice(crisis_truth -[ChoiceEdge { label: "送信を実行する".to_string() }]-> truth_sent),

        // ============================================================
        // finale (エンディングへの到達)
        // ============================================================
        f_sbay = Finale(shuttle_bay -> ending_evacuate),
        f_ssac = Finale(seal_sacrifice -> ending_sacrifice),
        f_tsent = Finale(truth_sent -> ending_truth),
        f_cfreeze = Finale(crisis_freeze -> ending_isolation),
    })
}

/// シナリオの開始シーン。
pub fn start_scene_id() -> SceneId {
    SceneId("start".to_string())
}

// ============================================================
// 意図的に壊れたシナリオ (validate のテスト用)
// ============================================================
//
// - `br_unreachable` は誰からも choice/finale で参照されない (到達不能)。
// - `br_dead` は選択肢もfinaleも持たない (デッドエンド)。
#[rustfmt::skip]
pub fn build_broken_story() -> Result<DialogueGraph, DialogueGraphViolation> {
    graphite::graph!(DialogueGraph {
        br_start = Scene {
            speaker: "テスト".to_string(),
            text: "壊れたシナリオの開始".to_string()
        },
        br_ok = Scene {
            speaker: "テスト".to_string(),
            text: "普通に続いてエンディングへ到達する".to_string()
        },
        br_dead = Scene {
            speaker: "テスト".to_string(),
            text: "選択肢もfinaleも無い、行き止まり".to_string()
        },
        br_unreachable = Scene {
            speaker: "テスト".to_string(),
            text: "誰からも参照されない孤立シーン".to_string()
        },
        br_end = Ending {
            title: "テスト終了".to_string(),
            epilogue: "壊れたシナリオのエンディング".to_string()
        },

        c_brstart_brok = Choice(br_start -[ChoiceEdge { label: "進む".to_string() }]-> br_ok),
        c_brstart_brdead = Choice(br_start -[ChoiceEdge { label: "行き止まりへ向かう".to_string() }]-> br_dead),
        f_brok = Finale(br_ok -> br_end),
        // br_dead は意図的に何の辺も出さない (デッドエンド)。
        c_brunreachable_brok = Choice(br_unreachable -[ChoiceEdge { label: "戻る".to_string() }]-> br_ok),
        // br_unreachable は意図的に誰からも参照しない (到達不能)。
    })
}

pub fn broken_start_scene_id() -> SceneId {
    SceneId("br_start".to_string())
}

// ============================================================
// 「エンディングに到達できない閉じたループ」だけを持つテスト用シナリオ
// ============================================================
//
// `validate` の trapped_scenes (循環 + どのエンディングにも到達できない)
// 検出だけをピンポイントで確認するための最小フィクスチャ。v3 では
// `graph!` を別モジュールから呼ぶこと自体は制約ではなくなったが、この
// フィクスチャは schema.rs 内の型・スキーマにだけ依存する小さな関数なので、
// `validate.rs` のテストからはこの関数を経由して使う (単なる整理上の判断)。
#[rustfmt::skip]
pub fn build_pure_loop_story() -> Result<DialogueGraph, DialogueGraphViolation> {
    graphite::graph!(DialogueGraph {
        t_start = Scene { speaker: "テスト".to_string(), text: "開始".to_string() },
        t_loop_a = Scene { speaker: "テスト".to_string(), text: "ループA".to_string() },
        t_loop_b = Scene { speaker: "テスト".to_string(), text: "ループB".to_string() },

        c_tstart_tloopa = Choice(t_start -[ChoiceEdge { label: "ループへ".to_string() }]-> t_loop_a),
        c_tloopa_tloopb = Choice(t_loop_a -[ChoiceEdge { label: "Bへ".to_string() }]-> t_loop_b),
        c_tloopb_tloopa = Choice(t_loop_b -[ChoiceEdge { label: "Aへ".to_string() }]-> t_loop_a),
        // どのシーンにも finale が無い = t_loop_a/t_loop_b は循環しつつ
        // どのエンディングにも到達できない「罠」になる。
    })
}

pub fn pure_loop_start_scene_id() -> SceneId {
    SceneId("t_start".to_string())
}
