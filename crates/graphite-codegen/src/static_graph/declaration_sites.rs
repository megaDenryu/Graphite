// instanceファイルの意味カードが参照する2つの宣言元の組 (issue #41 段階3)。
//
// schema と instance は別ファイルに書けるため、instance側の宣言元だけでは
// 「関係する schema 宣言」の段落 (`naming::card_names`・`naming::type_names`
// の一部) が実在しない場所 (instanceのファイル) を指してしまう。cliの2段階
// の解決 (静的schema名簿からinstanceを見つける) が、schemaの宣言元を
// instanceの本文組み立てまで運ぶために使う。schema自身の生成物
// (`file::schema_file`) はこの組を必要とせず、自分の宣言元
// (`宣言元ファイルの綴り`) 1つだけで完結する。

use crate::schema::codegen::宣言元ファイルの綴り;

pub(crate) struct 宣言元の対 {
    instance: 宣言元ファイルの綴り,
    schema: 宣言元ファイルの綴り,
}

impl 宣言元の対 {
    pub(crate) fn new(instance: 宣言元ファイルの綴り, schema: 宣言元ファイルの綴り) -> Self {
        Self { instance, schema }
    }

    // instance自身の宣言 (`node`/`edge`/`graph`) への参照に使う。
    pub(crate) fn instance(&self) -> &宣言元ファイルの綴り {
        &self.instance
    }

    // instanceが由来するschemaの宣言 (`node`/`edge` 種別) への参照に使う。
    pub(crate) fn schema(&self) -> &宣言元ファイルの綴り {
        &self.schema
    }
}
