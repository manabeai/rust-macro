use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeMap;
use std::hash::Hash;
use std::sync::Arc;

pub trait DagDPRules {
    type State: Clone + Eq + Hash;
    type Value: Clone;
    type Ctx;

    fn rank(ctx: &Self::Ctx, s: &Self::State) -> usize;
    fn neighbors(ctx: &Self::Ctx, s: &Self::State) -> Vec<Self::State>;
    fn combine(ctx: &Self::Ctx, s: &Self::State, child_vals: &[Self::Value]) -> Self::Value;
}

pub struct Engine;
impl Engine {
    pub fn solve<D: DagDPRules>(
        ctx: &D::Ctx,
        roots: impl IntoIterator<Item = D::State>,
    ) -> FxHashMap<D::State, D::Value> {
        let mut seen = FxHashSet::<D::State>::default();
        let mut buckets = BTreeMap::<usize, Vec<D::State>>::new();
        let mut adj = FxHashMap::<D::State, Vec<D::State>>::default();

        let mut stack: Vec<D::State> = roots.into_iter().collect();
        for s in &stack {
            if seen.insert(s.clone()) {
                buckets.entry(D::rank(ctx, s)).or_default().push(s.clone());
            }
        }
        while let Some(s) = stack.pop() {
            let rs = D::rank(ctx, &s);
            let ns = D::neighbors(ctx, &s);
            debug_assert!(ns.iter().all(|t| D::rank(ctx, t) < rs));
            adj.insert(s.clone(), ns.clone());
            for t in ns {
                if seen.insert(t.clone()) {
                    buckets.entry(D::rank(ctx, &t)).or_default().push(t.clone());
                    stack.push(t);
                }
            }
        }

        let mut val = FxHashMap::<D::State, D::Value>::default();
        for (_r, states) in buckets.iter() {
            for s in states {
                let childs = adj.get(s).map(|v| v.as_slice()).unwrap_or(&[]);
                let child_vals: Vec<D::Value> =
                    childs.iter().map(|c| val.get(c).unwrap().clone()).collect();
                let v = D::combine(ctx, s, &child_vals);
                val.insert(s.clone(), v);
            }
        }
        val
    }
}

// ===== ここから所有Ctx版 =====
#[derive(Clone, Eq, PartialEq, Hash)]
struct S {
    i: usize,
    sick: bool,
}

struct PoisonCtx {
    xs: Arc<[i32]>,
    ys: Arc<[i64]>,
}

struct Poison;

impl DagDPRules for Poison {
    type State = S;
    type Value = i64;
    type Ctx = PoisonCtx;

    fn rank(ctx: &Self::Ctx, s: &Self::State) -> usize {
        ctx.xs.len() - s.i
    }

    fn neighbors(ctx: &Self::Ctx, s: &Self::State) -> Vec<Self::State> {
        if s.i == ctx.xs.len() {
            return vec![];
        }
        let x = ctx.xs[s.i];
        let mut res = Vec::with_capacity(2);
        // スキップ
        res.push(S {
            i: s.i + 1,
            sick: s.sick,
        });
        // 食べる（死亡手は生成しない）
        match (s.sick, x) {
            (false, 0) | (true, 0) => res.push(S {
                i: s.i + 1,
                sick: false,
            }),
            (false, 1) => res.push(S {
                i: s.i + 1,
                sick: true,
            }),
            (true, 1) => {}
            _ => unreachable!(),
        }
        res
    }

    fn combine(ctx: &Self::Ctx, s: &Self::State, child_vals: &[Self::Value]) -> Self::Value {
        if s.i == ctx.xs.len() {
            return 0;
        }
        let x = ctx.xs[s.i];
        let y = ctx.ys[s.i];

        // neighbors の順： [Skip, (Eatがあれば)Eat]
        let mut best = child_vals[0]; // Skip
        let eat_ok = matches!((s.sick, x), (false, 0) | (true, 0) | (false, 1));
        if eat_ok {
            best = best.max(child_vals[1] + y);
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SimpleDP;

    #[derive(Clone, Eq, PartialEq, Hash, Debug)]
    struct SimpleState(usize);

    impl DagDPRules for SimpleDP {
        type State = SimpleState;
        type Value = i32;
        type Ctx = ();

        fn rank(_ctx: &Self::Ctx, s: &Self::State) -> usize {
            s.0
        }

        fn neighbors(_ctx: &Self::Ctx, s: &Self::State) -> Vec<Self::State> {
            if s.0 == 0 {
                vec![]
            } else {
                vec![SimpleState(s.0 - 1)]
            }
        }

        fn combine(_ctx: &Self::Ctx, s: &Self::State, child_vals: &[Self::Value]) -> Self::Value {
            if child_vals.is_empty() {
                1
            } else {
                child_vals[0] + (s.0 as i32)
            }
        }
    }

    #[test]
    fn test_simple_dp_engine() {
        let ctx = ();
        let roots = vec![SimpleState(3)];
        let result = Engine::solve::<SimpleDP>(&ctx, roots);

        assert_eq!(result.get(&SimpleState(0)), Some(&1));
        assert_eq!(result.get(&SimpleState(1)), Some(&2));
        assert_eq!(result.get(&SimpleState(2)), Some(&4));
        assert_eq!(result.get(&SimpleState(3)), Some(&7));
    }

    #[test]
    fn test_poison_dp_basic() {
        let xs = Arc::new([0, 1, 0]);
        let ys = Arc::new([10, 20, 30]);
        let ctx = PoisonCtx { xs, ys };

        let roots = vec![S { i: 0, sick: false }];
        let result = Engine::solve::<Poison>(&ctx, roots);

        let start_state = S { i: 0, sick: false };
        assert!(result.contains_key(&start_state));
        assert!(result.get(&start_state).unwrap() >= &40);
    }

    #[test]
    fn test_poison_dp_edge_cases() {
        let xs = Arc::new([]);
        let ys = Arc::new([]);
        let ctx = PoisonCtx { xs, ys };

        let roots = vec![S { i: 0, sick: false }];
        let result = Engine::solve::<Poison>(&ctx, roots);

        let start_state = S { i: 0, sick: false };
        assert_eq!(result.get(&start_state), Some(&0));
    }

    #[test]
    fn test_poison_dp_single_item() {
        let xs = Arc::new([0]);
        let ys = Arc::new([100]);
        let ctx = PoisonCtx { xs, ys };

        let roots = vec![S { i: 0, sick: false }];
        let result = Engine::solve::<Poison>(&ctx, roots);

        let start_state = S { i: 0, sick: false };
        assert_eq!(result.get(&start_state), Some(&100));
    }

    #[test]
    fn test_poison_dp_all_poison() {
        let xs = Arc::new([1, 1, 1]);
        let ys = Arc::new([10, 20, 30]);
        let ctx = PoisonCtx { xs, ys };

        let roots = vec![S { i: 0, sick: false }];
        let result = Engine::solve::<Poison>(&ctx, roots);

        let start_state = S { i: 0, sick: false };
        assert_eq!(result.get(&start_state), Some(&30));
    }

    struct TreeDP;

    #[derive(Clone, Eq, PartialEq, Hash, Debug)]
    struct TreeState {
        node: usize,
        depth: usize,
    }

    struct TreeCtx {
        children: Vec<Vec<usize>>,
    }

    impl DagDPRules for TreeDP {
        type State = TreeState;
        type Value = usize;
        type Ctx = TreeCtx;

        fn rank(_ctx: &Self::Ctx, s: &Self::State) -> usize {
            usize::MAX - s.depth
        }

        fn neighbors(ctx: &Self::Ctx, s: &Self::State) -> Vec<Self::State> {
            ctx.children[s.node]
                .iter()
                .map(|&child| TreeState {
                    node: child,
                    depth: s.depth + 1,
                })
                .collect()
        }

        fn combine(_ctx: &Self::Ctx, _s: &Self::State, child_vals: &[Self::Value]) -> Self::Value {
            1 + child_vals.iter().sum::<usize>()
        }
    }

    #[test]
    fn test_tree_dp() {
        let children = vec![vec![1, 2], vec![3, 4], vec![], vec![], vec![]];
        let ctx = TreeCtx { children };

        let roots = vec![TreeState { node: 0, depth: 0 }];
        let result = Engine::solve::<TreeDP>(&ctx, roots);

        assert_eq!(result.get(&TreeState { node: 0, depth: 0 }), Some(&5));
        assert_eq!(result.get(&TreeState { node: 1, depth: 1 }), Some(&3));
        assert_eq!(result.get(&TreeState { node: 2, depth: 1 }), Some(&1));
        assert_eq!(result.get(&TreeState { node: 3, depth: 2 }), Some(&1));
        assert_eq!(result.get(&TreeState { node: 4, depth: 2 }), Some(&1));
    }

    #[test]
    fn test_multiple_roots() {
        let ctx = ();
        let roots = vec![SimpleState(2), SimpleState(1)];
        let result = Engine::solve::<SimpleDP>(&ctx, roots);

        assert_eq!(result.len(), 3);
        assert!(result.contains_key(&SimpleState(0)));
        assert!(result.contains_key(&SimpleState(1)));
        assert!(result.contains_key(&SimpleState(2)));
    }

    #[test]
    fn test_empty_roots() {
        let ctx = ();
        let roots: Vec<SimpleState> = vec![];
        let result = Engine::solve::<SimpleDP>(&ctx, roots);

        assert!(result.is_empty());
    }
}
