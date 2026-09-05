//! 生成コードの公開項目を辿り、doc コメントの欠落を集める。
//!
//! 公開到達性は「囲むモジュールが全て `pub` であること」で判定する。生成ファイルは
//! 利用者のクレートへ include されるため、クレート境界を越えた再公開は追えない。

use syn::visit::{self, Visit};
use syn::{Attribute, Field, ImplItem, Item, TraitItem, Variant};

use super::attribute_facts::{has_doc_comment, is_doc_hidden, is_public};
use super::item_facts::{impl_item_facts, item_facts, trait_item_facts};

pub(super) struct PublicItemVisitor {
    public_item_count: usize,
    items_without_doc: Vec<String>,
    scope: Vec<String>,
    inside_public_scope: bool,
    inside_variant: bool,
}

impl PublicItemVisitor {
    pub(super) fn new() -> Self {
        Self {
            public_item_count: 0,
            items_without_doc: Vec::new(),
            scope: Vec::new(),
            inside_public_scope: true,
            inside_variant: false,
        }
    }

    pub(super) fn public_item_count(&self) -> usize {
        self.public_item_count
    }

    pub(super) fn items_without_doc(&self) -> &[String] {
        &self.items_without_doc
    }

    fn record(&mut self, name: &str, attributes: &[Attribute]) {
        self.public_item_count += 1;
        if !has_doc_comment(attributes) {
            let mut path = self.scope.clone();
            path.push(name.to_string());
            self.items_without_doc.push(path.join("::"));
        }
    }
}

impl<'ast> Visit<'ast> for PublicItemVisitor {
    fn visit_item(&mut self, item: &'ast Item) {
        let Some((visibility, attributes, name)) = item_facts(item) else {
            visit::visit_item(self, item);
            return;
        };
        if is_doc_hidden(attributes) {
            return;
        }
        let publicly_reachable = self.inside_public_scope && is_public(visibility);
        if publicly_reachable {
            self.record(&name, attributes);
        }
        let enclosing = self.inside_public_scope;
        self.inside_public_scope = publicly_reachable;
        self.scope.push(name);
        visit::visit_item(self, item);
        self.scope.pop();
        self.inside_public_scope = enclosing;
    }

    fn visit_field(&mut self, field: &'ast Field) {
        let Some(name) = field.ident.as_ref() else {
            return;
        };
        if is_doc_hidden(&field.attrs) || !self.inside_public_scope {
            return;
        }
        if self.inside_variant || is_public(&field.vis) {
            self.record(&name.to_string(), &field.attrs);
        }
    }

    fn visit_variant(&mut self, variant: &'ast Variant) {
        if is_doc_hidden(&variant.attrs) || !self.inside_public_scope {
            return;
        }
        self.record(&variant.ident.to_string(), &variant.attrs);
        self.inside_variant = true;
        visit::visit_variant(self, variant);
        self.inside_variant = false;
    }

    fn visit_trait_item(&mut self, item: &'ast TraitItem) {
        let Some((attributes, name)) = trait_item_facts(item) else {
            return;
        };
        if is_doc_hidden(attributes) || !self.inside_public_scope {
            return;
        }
        self.record(&name, attributes);
    }

    fn visit_impl_item(&mut self, item: &'ast ImplItem) {
        let Some((visibility, attributes, name)) = impl_item_facts(item) else {
            return;
        };
        if is_doc_hidden(attributes) || !self.inside_public_scope || !is_public(visibility) {
            return;
        }
        self.record(&name, attributes);
    }
}
