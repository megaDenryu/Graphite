//! §3「検証エラーを受ける」のうち、違反を何件受け取るかの選択。
//!
//! 違反の種類ではなく構築APIの選択の話であり、`create` は最初の1件で打ち切り、
//! `create_collecting` は打ち切らず全件を `Vec` に集める。

use crate::Org;
use crate::Org::PersonId;
use crate::Person;

// やりたいこと: `create` は最初の1件の違反で `Err` になる (複数あっても1件目だけ)。
pub fn createは最初の1件で違反を止める() {
    let result: Result<Org::Graph, Org::Violation> = Org::Graph::create(|b: &mut Org::Builder| {
        b.person(
            PersonId("alice".to_string()),
            Person {
                name: "Alice".to_string(),
            },
        );
        b.person(
            PersonId("bob".to_string()),
            Person {
                name: "Bob".to_string(),
            },
        );
        // alice, bobともどのチームにも所属させない (違反が2件あるはず)
    });
    let violation: Org::Violation = match result {
        Err(violation) => violation,
        Ok(_) => panic!("違反が検出されるはず"),
    };
    println!("(create) 最初の1件だけ: {violation}");
}

// やりたいこと: `create_collecting` は打ち切らず全違反を `Vec` に集める。
pub fn create_collectingで全違反を集める() {
    let result: Result<Org::Graph, Vec<Org::Violation>> =
        Org::Graph::create_collecting(|b: &mut Org::Builder| {
            b.person(
                PersonId("alice".to_string()),
                Person {
                    name: "Alice".to_string(),
                },
            );
            b.person(
                PersonId("bob".to_string()),
                Person {
                    name: "Bob".to_string(),
                },
            );
            // alice, bobともどのチームにも所属させない (2件のeach違反が集まるはず)
        });
    let violations: Vec<Org::Violation> = match result {
        Err(violations) => violations,
        Ok(_) => panic!("2件の違反が集まるはず"),
    };
    for violation in &violations {
        println!("(create_collecting) 違反: {violation}");
    }
}
