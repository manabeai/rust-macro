use rustc_hash::FxHashMap;
use std::cmp::min;
use std::fmt::Debug;
use std::hash::Hash;
pub trait PushDPRules {
    type State: Clone + Eq + Hash + Debug;
    type Value: Clone + Debug;
    type Ctx;

    // rank: 低→高のトポ順で処理（push なので succ は rank が上がる想定）
    fn rank(ctx: &Self::Ctx, s: &Self::State) -> usize;

    // succs: s から「配る先」(高ランク) へ
    fn succs(ctx: &Self::Ctx, s: &Self::State) -> Vec<Self::State>;

    // モノイド：単位元と結合
    fn identity(ctx: &Self::Ctx) -> Self::Value;
    fn op(ctx: &Self::Ctx, into: &Self::Value, add: &Self::Value) -> Self::Value;

    // 初期値（ソースのみ Some、通常は None）
    fn init(ctx: &Self::Ctx, s: &Self::State) -> Option<Self::Value>;

    // 遷移：val[s] から to へ配る値
    fn trans(
        ctx: &Self::Ctx,
        from: &Self::State,
        to: &Self::State,
        v_from: &Self::Value,
    ) -> Self::Value;
}
pub struct PushDpEngine;
impl PushDpEngine {
    pub fn propagate<D: PushDPRules>(
        ctx: &D::Ctx,
        sources: impl IntoIterator<Item = D::State>,
    ) -> FxHashMap<D::State, D::Value> {
        use rustc_hash::{FxHashMap, FxHashSet};
        use std::collections::BTreeMap;

        let mut seen = FxHashSet::<D::State>::default();
        let mut buckets = BTreeMap::<usize, Vec<D::State>>::new();
        let mut adj = FxHashMap::<D::State, Vec<D::State>>::default();

        let mut stack: Vec<D::State> = sources.into_iter().collect();
        for s in &stack {
            if seen.insert(s.clone()) {
                buckets.entry(D::rank(ctx, s)).or_default().push(s.clone());
            }
        }
        while let Some(s) = stack.pop() {
            let rs = D::rank(ctx, &s);
            let ns = D::succs(ctx, &s);
            // eprintln!("succs: state={:?}, nexts={:?}, rank_next={:?}", s, ns, rs);
            debug_assert!(ns.iter().all(|t| D::rank(ctx, t) > rs));
            adj.insert(s.clone(), ns.clone());
            for t in ns {
                if seen.insert(t.clone()) {
                    buckets.entry(D::rank(ctx, &t)).or_default().push(t.clone());
                    stack.push(t);
                }
            }
        }

        let mut val = FxHashMap::<D::State, D::Value>::default();
        // ソースの初期化
        for (_r, states) in buckets.iter() {
            for s in states {
                if let Some(v0) = D::init(ctx, s) {
                    // eprintln!("init: state={:?}, v={:?}", s, v0);
                    val.insert(s.clone(), v0);
                }
            }
        }

        // rank 昇順で配る
        for (_r, states) in buckets.iter() {
            for s in states {
                let vs = val.get(s).cloned().unwrap_or_else(|| D::identity(ctx));
                if let Some(succs) = adj.get(s) {
                    for t in succs {
                        let inc = D::trans(ctx, s, t, &vs);
                        // eprintln!("trans: state={:?}, from={:?}, to={:?}, v_from={:?} => {:?}", s, s, t, &vs, inc);
                        let entry = val.entry(t.clone()).or_insert_with(|| D::identity(ctx));
                        *entry = D::op(ctx, entry, &inc);
                        // eprintln!("op: state={:?}, a={:?}, b={:?} => {:?}", s, *entry, &inc, *entry);
                    }
                }
            }
        }
        val
    }
}

// 実装用のトレイト実装の雛形。
// #[derive(Clone, Hash, Eq, PartialEq, Debug)]
// struct Dummy;
// struct Problem;
// impl PushDPRules for Problem {
//     type State = Dummy;
//     type Value = Dummy;
//     type Ctx = Dummy;

//     fn rank(ctx: &Self::Ctx, s: &Self::State) -> usize {
//         todo!()
//     }

//     fn succs(ctx: &Self::Ctx, s: &Self::State) -> Vec<Self::State> {
//         todo!()
//     }

//     fn identity(ctx: &Self::Ctx) -> Self::Value {
//         todo!()
//     }

//     fn op(ctx: &Self::Ctx, a: &Self::Value, b: &Self::Value) -> Self::Value {
//         todo!()
//     }

//     fn init(ctx: &Self::Ctx, s: &Self::State) -> Option<Self::Value> {
//         todo!()
//     }

//     fn trans(
//         ctx: &Self::Ctx,
//         from: &Self::State,
//         to: &Self::State,
//         v_from: &Self::Value,
//     ) -> Self::Value {
//         todo!()
//     }
// }

mod tests {
    use super::*;
    struct Ctx {
        h: Vec<i64>,
    }
    struct FrogPush;
    impl PushDPRules for FrogPush {
        type State = usize;
        type Value = i64;
        type Ctx = Ctx;

        fn rank(_ctx: &Self::Ctx, s: &Self::State) -> usize {
            *s
        }

        fn succs(ctx: &Self::Ctx, s: &Self::State) -> Vec<Self::State> {
            match ctx.h.len() - s {
                0 => unreachable!(),
                1 => vec![],
                2 => vec![s + 1],
                _ => vec![s + 1, s + 2],
            }
        }

        fn identity(_ctx: &Self::Ctx) -> Self::Value {
            i64::MAX
        }

        fn op(_ctx: &Self::Ctx, a: &Self::Value, b: &Self::Value) -> Self::Value {
            min(*a, *b)
        }

        fn init(_ctx: &Self::Ctx, s: &Self::State) -> Option<Self::Value> {
            match s {
                0 => Some(0),
                _ => None,
            }
        }

        fn trans(
            ctx: &Self::Ctx,
            from: &Self::State,
            to: &Self::State,
            v_from: &Self::Value,
        ) -> Self::Value {
            v_from + (ctx.h[*from] - ctx.h[*to]).abs()
        }
    }

    // source(https://atcoder.jp/contests/dp/tasks/dp_a)
    #[test]
    fn test_push_dp() {
        let ctx = Ctx {
            h: vec![10, 30, 40, 20],
        };
        let sources = vec![0];
        let result = PushDpEngine::propagate::<FrogPush>(&ctx, sources);
        assert_eq!(result.len(), 4);
        assert_eq!(result.get(&0), Some(&0));
        assert_eq!(result.get(&1), Some(&20));
        assert_eq!(result.get(&2), Some(&30));
        assert_eq!(result.get(&3), Some(&30));
    }

    #[test]
    fn test_enhanced_push_dp() {
        let ctx = Ctx {
            h: vec![10, 30, 40, 20],
        };
        let sources = vec![0];
        let result = PushDpEngineEnhanced::propagate::<FrogPush>(&ctx, sources);
        assert_eq!(result.len(), 4);
        assert_eq!(result.get(&0), Some(&0));
        assert_eq!(result.get(&1), Some(&20));
        assert_eq!(result.get(&2), Some(&30));
        assert_eq!(result.get(&3), Some(&30));
    }
}

// 高速化したエンジン。　若干可読性が悪いので前のバージョンも残す
pub struct PushDpEngineEnhanced;
impl PushDpEngineEnhanced {
    pub fn propagate<D: PushDPRules>(
        ctx: &D::Ctx,
        sources: impl IntoIterator<Item = D::State>,
    ) -> FxHashMap<D::State, D::Value> {
        use rustc_hash::{FxHashMap, FxHashSet};
        use std::collections::BTreeMap;

        let mut seen = FxHashSet::<D::State>::default();
        let mut buckets = BTreeMap::<usize, Vec<D::State>>::new();
        let mut adj = FxHashMap::<D::State, Vec<D::State>>::default();

        let mut stack: Vec<D::State> = sources.into_iter().collect();
        for s in &stack {
            if seen.insert(s.clone()) {
                buckets.entry(D::rank(ctx, s)).or_default().push(s.clone());
            }
        }
        while let Some(s) = stack.pop() {
            let rs = D::rank(ctx, &s);
            let ns = D::succs(ctx, &s);
            debug_assert!(ns.iter().all(|t| D::rank(ctx, t) > rs));
            adj.insert(s.clone(), ns.clone());
            for t in ns {
                if seen.insert(t.clone()) {
                    buckets.entry(D::rank(ctx, &t)).or_default().push(t.clone());
                    stack.push(t);
                }
            }
        }

        let mut val = FxHashMap::<D::State, D::Value>::default();
        // ソースの初期化
        for (_r, states) in buckets.iter() {
            for s in states {
                if let Some(v0) = D::init(ctx, s) {
                    val.insert(s.clone(), v0);
                }
            }
        }

        // rank 昇順で配る
        for (_r, states) in buckets.iter() {
            for s in states {
                let vs = val.get(s).cloned().unwrap_or_else(|| D::identity(ctx));
                if let Some(succs) = adj.get(s) {
                    for t in succs {
                        let inc = D::trans(ctx, s, t, &vs);
                        let entry = val.entry(t.clone()).or_insert_with(|| D::identity(ctx));
                        *entry = D::op(ctx, entry, &inc);
                    }
                }
            }
        }
        val
    }
}
