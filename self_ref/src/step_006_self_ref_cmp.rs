/// [[step_005_self_ref_holder]] 의 단점은 BTreeSet의 Key가 포인터 타입이라는 데에 있다.
///
/// 따라서 우리가 어떤 멤버를 키값으로 BTreeSet을 구축하고 싶어 별도의 Ord 트레이트를
/// 구현할지라도 `BTreeSet<*mut Me>` 에 의해 결국은 주소값을 비교하게 될 것이고,
/// 의도하지 않은 결과를 얻을 것이다. 따라서 `*mut Me`를 감싸는 래퍼 구조체를 만들어
/// 래퍼 자체에 `Ord` 트레이트를 구현하게 만들고 자동 형변환을 구현할 수 있는 `From`
/// 트레이트를 구현하여 사용에 편의를 제공할 수 있다.
use std::{cell::RefCell, collections::BTreeSet, marker::PhantomPinned, pin::Pin, rc::Rc};
type RcCell<T> = Rc<RefCell<T>>;

/// 다음 모듈은 `Ord` 트레이트의 속성에 대하여 실험을 진행하는 코드이다.
/// 복합 구조체 `Node`에 대하여 노드의 어떤 속성을 기준으로 비교를 진행하는지
/// 확인한 결과, 모든 멤버들에 대한 비교를 진행하는 것으로 확인됐다.
///
/// [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)의 첫번째 줄에서
/// 이미 다음 글귀를 확인할 수 있었다. (좀 일찍 볼걸..)
///
/// > Trait for types that form a **total order**
/// > 전순서 집합을 형성하는 트레이트
mod tutorial {
    use std::collections::BTreeSet;
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct Node {
        name: String,
        id: u32,
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn run() {
            let mut holder: BTreeSet<&Node> = BTreeSet::new();
            let nodes = [
                Node {
                    name: "n1".to_owned(),
                    id: 1,
                },
                Node {
                    name: "n2".to_owned(),
                    id: 2,
                },
                Node {
                    name: "n3".to_owned(),
                    id: 3,
                },
            ];
            nodes.iter().for_each(|each| {
                holder.insert(each);
            });

            assert_eq!(3, holder.len());

            // what if we insert same `name` node in holder?
            let dup1 = Node {
                name: "n1".to_owned(),
                id: 123124,
            };
            holder.insert(&dup1);

            assert_eq!(4, holder.len()); // Ok

            // what if we insert same `id` node in holder?
            let dup2 = Node {
                name: "dup2".to_owned(),
                id: 2,
            };
            holder.insert(&dup2);

            assert_eq!(5, holder.len());

            // what if we insert same both `name` and `id` in holder?
            let dup3 = Node {
                name: "n3".to_owned(),
                id: 3,
            };
            holder.insert(&dup3);

            assert_eq!(5, holder.len()); // didn't inserted!!
        }
    }
}
