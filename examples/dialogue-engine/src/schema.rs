//! スキーマ宣言 (`graph_schema!`) と、シナリオ本編・破損シナリオの
//! `graph!` リテラル本体。
//!
//! v3 (`docs/graph_literal_v3.md` §4) でハンドシェイクマクロを全廃したため、
//! `graph_schema!` と `graph!` を同一ファイルに置く必要は無くなった (`graph!`
//! が参照するのは通常の型・メソッドだけになったため、別モジュールから `use`
//! すれば足りる。実証は `crates/graphite/tests/graph_cross_module.rs`)。
//! この example では単に型定義とシナリオ本編が近くにあった方が読みやすい
//! という理由で同居させている。

// `-[label]->` 記法は rustfmt が「知らない構文」として誤整形しうるため、
// `graph!` を呼ぶ関数には個別に `#[rustfmt::skip]` を付ける
// (.claude/skills/proc-macro-dev/SKILL.md の注意通り)。

use graphite::Graph;

// ============================================================
// スキーマ宣言
// ============================================================
//
// node Scene:  1 場面。話者と本文を持つ。
// node Ending: 1 エンディング。タイトルとエピローグ本文を持つ。
// edge choice: Scene -[ChoiceEdge { label: String }]-> Scene (0..*) —
//              選択肢。多重度 0..* = 1 シーンから何本でも選択肢を出せる。
// edge finale: Scene -> Ending (0..1) — エンディングへの到達。
//              多重度 0..1 = 1 シーンにつき高々 1 つの結末。

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub speaker: String,
    pub text: String,
}

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Ending {
    pub title: String,
    pub epilogue: String,
}

/// `choice` エッジの属性 (選択肢のラベル文字列)。
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceEdge {
    pub label: String,
}

graphite::graph_schema! {
    schema DialogueGraph {
        node Scene;
        node Ending;

        edge choice: Scene -[ChoiceEdge]-> Scene (0..*);
        edge finale: Scene -> Ending (0..1);
    }
}

// ============================================================
// 導出クエリ (README.md 「使用例3」節のパターン: 保存エッジ=フィールド,
// 導出エッジ=同一モジュール内の普通のメソッド)
// ============================================================

impl DialogueGraph {
    /// あるシーンから出ている選択肢一覧を `(行き先キー, 選択肢ラベル)` で返す。
    /// `choice().iter()` は `match` クエリの代替として提供されているビューの
    /// 走査で、ここでは特定の始点だけに絞り込むフィルタとして使う。
    pub fn scene_choices(&self, id: &SceneId) -> Vec<(SceneId, String)> {
        self.choice()
            .iter()
            .filter(|(from, _to, _attrs)| *from == id)
            .map(|(_from, to, attrs)| (to.clone(), attrs.label.clone()))
            .collect()
    }

    /// choice 辺だけを汎用グラフ `Graph<SceneId, String, SceneId>` へ射影する。
    /// `reachable_from`/`has_cycle`/`path`/`filter_nodes` のような、図式グラフ
    /// (`graph_schema!`) には無いグラフアルゴリズムを使うための橋渡し。
    /// ノードの値には (使わないが) キー自身を積んでおく。辺の値には選択肢
    /// ラベルを積み、`route` コマンドでの表示に使う。
    ///
    /// 構築は `Scene` の集合と `choice` 辺だけから機械的に決まるため、
    /// このシナリオが `DialogueGraph::create` を通過している時点で
    /// 重複キー・未知キーは有り得ず、`expect` で握り潰してよい。
    pub fn scene_graph(&self) -> Graph<SceneId, String, SceneId> {
        Graph::create(|b| {
            for id in self.scene_ids() {
                b.node(id.clone(), id.clone());
            }
            for (from, to, attrs) in self.choice().iter() {
                b.edge(from.clone(), to.clone(), attrs.label.clone());
            }
        })
        .expect("scene_graph の射影は DialogueGraph が既に検証済みなので必ず成功する")
    }

    /// このシーンに finale (エンディングへの到達) があるか。
    pub fn is_finale_scene(&self, id: &SceneId) -> bool {
        self.finale().of(id).is_some()
    }

    /// このシーンに選択肢が 0 本、かつ finale も無いか (= デッドエンド)。
    pub fn is_dead_end(&self, id: &SceneId) -> bool {
        self.choice().of(id).is_empty() && self.finale().of(id).is_none()
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
        // 選択肢 (choice) — 導入
        // ============================================================
        start -[choice = ChoiceEdge { label: "基地へ急行する".to_string() }]-> arrival,
        arrival -[choice = ChoiceEdge { label: "中へ入る".to_string() }]-> airlock,
        airlock -[choice = ChoiceEdge { label: "格納庫を調べる".to_string() }]-> hangar,
        airlock -[choice = ChoiceEdge { label: "研究室を調べる".to_string() }]-> lab,
        airlock -[choice = ChoiceEdge { label: "居住区を調べる".to_string() }]-> quarters,

        // --- 中央ホール: 3エリア + 地下への行き来 (合流点) ---
        central -[choice = ChoiceEdge { label: "格納庫へ".to_string() }]-> hangar,
        central -[choice = ChoiceEdge { label: "研究室へ".to_string() }]-> lab,
        central -[choice = ChoiceEdge { label: "居住区へ".to_string() }]-> quarters,
        central -[choice = ChoiceEdge { label: "地下区画へ続くハッチを開ける".to_string() }]-> lower_hatch,

        // --- 格納庫ルート (central との往復 + 自己ループ) ---
        hangar -[choice = ChoiceEdge { label: "ローバーの運行記録を調べる".to_string() }]-> hangar_log,
        hangar -[choice = ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central,
        hangar_log -[choice = ChoiceEdge { label: "もう一度記録を洗い直す".to_string() }]-> hangar_log,
        hangar_log -[choice = ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central,

        // --- 研究室ルート ---
        lab -[choice = ChoiceEdge { label: "サンプル保管庫を調べる".to_string() }]-> lab_samples,
        lab -[choice = ChoiceEdge { label: "研究用端末を調べる".to_string() }]-> lab_computer,
        lab -[choice = ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central,
        lab_samples -[choice = ChoiceEdge { label: "端末を調べる".to_string() }]-> lab_computer,
        lab_samples -[choice = ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central,
        lab_computer -[choice = ChoiceEdge { label: "エコーに詳細を尋ねる".to_string() }]-> lab_echo,
        lab_computer -[choice = ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central,
        lab_echo -[choice = ChoiceEdge { label: "研究室に戻る".to_string() }]-> lab,
        lab_echo -[choice = ChoiceEdge { label: "地下区画へ向かう".to_string() }]-> lower_hatch,

        // --- 居住区ルート ---
        quarters -[choice = ChoiceEdge { label: "日誌を調べる".to_string() }]-> quarters_diary,
        quarters -[choice = ChoiceEdge { label: "個室を順に見て回る".to_string() }]-> quarters_rooms,
        quarters -[choice = ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central,
        quarters_diary -[choice = ChoiceEdge { label: "個室を見て回る".to_string() }]-> quarters_rooms,
        quarters_diary -[choice = ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central,
        quarters_rooms -[choice = ChoiceEdge { label: "鍵のかかった部屋をこじ開ける".to_string() }]-> quarters_locked,
        quarters_rooms -[choice = ChoiceEdge { label: "中央ホールに戻る".to_string() }]-> central,
        quarters_locked -[choice = ChoiceEdge { label: "話を聞く".to_string() }]-> quarters_takashi,
        quarters_locked -[choice = ChoiceEdge { label: "その場を離れる".to_string() }]-> central,
        quarters_takashi -[choice = ChoiceEdge { label: "隔壁を閉める".to_string() }]-> takashi_seal,
        quarters_takashi -[choice = ChoiceEdge { label: "彼を連れて避難する".to_string() }]-> takashi_rescue,
        quarters_takashi -[choice = ChoiceEdge { label: "地下区画へ急ぐ".to_string() }]-> lower_hatch,

        // --- タカシを巡る分岐の合流 ---
        takashi_seal -[choice = ChoiceEdge { label: "地下ホールへ戻る".to_string() }]-> lower_hall,
        takashi_seal -[choice = ChoiceEdge { label: "封鎖を完了させる".to_string() }]-> seal_sacrifice,
        takashi_rescue -[choice = ChoiceEdge { label: "地下ホールへ急ぐ".to_string() }]-> lower_hall,
        takashi_rescue -[choice = ChoiceEdge { label: "管制室で状況を確認する".to_string() }]-> control_room,

        // --- 地下区画 (合流点 + 往復) ---
        lower_hatch -[choice = ChoiceEdge { label: "下りる".to_string() }]-> lower_hall,
        lower_hall -[choice = ChoiceEdge { label: "原子炉室へ".to_string() }]-> reactor,
        lower_hall -[choice = ChoiceEdge { label: "通信室へ".to_string() }]-> comms,
        lower_hall -[choice = ChoiceEdge { label: "管制室へ".to_string() }]-> control_room,
        lower_hall -[choice = ChoiceEdge { label: "中央ホールへ戻る".to_string() }]-> central,
        reactor -[choice = ChoiceEdge { label: "地下ホールに戻る".to_string() }]-> lower_hall,
        comms -[choice = ChoiceEdge { label: "地下ホールに戻る".to_string() }]-> lower_hall,
        comms -[choice = ChoiceEdge { label: "管制室へ".to_string() }]-> control_room,
        control_room -[choice = ChoiceEdge { label: "感染源を分析する".to_string() }]-> control_analysis,
        control_room -[choice = ChoiceEdge { label: "地下ホールへ戻る".to_string() }]-> lower_hall,

        // --- クライマックス分岐 ---
        control_analysis -[choice = ChoiceEdge { label: "全員を退避させる".to_string() }]-> crisis_evacuate,
        control_analysis -[choice = ChoiceEdge { label: "感染区画を封鎖する".to_string() }]-> crisis_seal,
        control_analysis -[choice = ChoiceEdge { label: "真相を記録し外部に送信する".to_string() }]-> crisis_truth,
        control_analysis -[choice = ChoiceEdge { label: "何も決められず立ち尽くす".to_string() }]-> crisis_freeze,

        crisis_evacuate -[choice = ChoiceEdge { label: "シャトルへ急ぐ".to_string() }]-> shuttle_bay,
        crisis_seal -[choice = ChoiceEdge { label: "隔壁を封鎖する".to_string() }]-> seal_sacrifice,
        crisis_seal -[choice = ChoiceEdge { label: "タカシを助けに戻る".to_string() }]-> takashi_rescue,
        crisis_truth -[choice = ChoiceEdge { label: "送信を実行する".to_string() }]-> truth_sent,

        // ============================================================
        // finale (エンディングへの到達)
        // ============================================================
        shuttle_bay -[finale]-> ending_evacuate,
        seal_sacrifice -[finale]-> ending_sacrifice,
        truth_sent -[finale]-> ending_truth,
        crisis_freeze -[finale]-> ending_isolation,
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

        br_start -[choice = ChoiceEdge { label: "進む".to_string() }]-> br_ok,
        br_start -[choice = ChoiceEdge { label: "行き止まりへ向かう".to_string() }]-> br_dead,
        br_ok -[finale]-> br_end,
        // br_dead は意図的に何の辺も出さない (デッドエンド)。
        br_unreachable -[choice = ChoiceEdge { label: "戻る".to_string() }]-> br_ok,
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

        t_start -[choice = ChoiceEdge { label: "ループへ".to_string() }]-> t_loop_a,
        t_loop_a -[choice = ChoiceEdge { label: "Bへ".to_string() }]-> t_loop_b,
        t_loop_b -[choice = ChoiceEdge { label: "Aへ".to_string() }]-> t_loop_a,
        // どのシーンにも finale が無い = t_loop_a/t_loop_b は循環しつつ
        // どのエンディングにも到達できない「罠」になる。
    })
}

pub fn pure_loop_start_scene_id() -> SceneId {
    SceneId("t_start".to_string())
}
