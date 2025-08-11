use rustc_hash::{FxHashMap, FxHashSet};
use std::hash::Hash;
pub struct ChildRef<'a, S, V> {
    pub state: &'a S,
    pub value: &'a V,
}

pub trait PullDPRules {
    type State: Clone + Eq + Hash;
    type Value: Clone;
    type Ctx;

    /// rank(child) < rank(parent) を満たす整数ランク
    fn rank(ctx: &Self::Ctx, s: &Self::State) -> usize;

    /// s の子（rank が小さい側）を返す
    fn neighbors(ctx: &Self::Ctx, s: &Self::State) -> Vec<Self::State>;

    /// 子 (state, value) の列を参照で受け取り、s の値を一発で決定
    fn combine<'a, I>(ctx: &Self::Ctx, s: &Self::State, childs: I) -> Self::Value
    where
        I: IntoIterator<Item = ChildRef<'a, Self::State, Self::Value>>,
        Self::State: 'a,
        Self::Value: 'a;

    /// 基底（直値を置きたいノード）。無ければ None
    fn base(_ctx: &Self::Ctx, _s: &Self::State) -> Option<Self::Value> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct Plan<S: Eq + Hash + Clone> {
    buckets: Vec<Vec<S>>,      // rank 昇順
    adj: FxHashMap<S, Vec<S>>, // s -> children
}

pub struct PullDpEngine;

impl PullDpEngine {
    pub fn prepare<D: PullDPRules>(
        ctx: &D::Ctx,
        roots: impl IntoIterator<Item = D::State>,
    ) -> Plan<D::State> {
        let mut seen = FxHashSet::<D::State>::default();
        let mut adj = FxHashMap::<D::State, Vec<D::State>>::default();

        let mut buckets: Vec<Vec<D::State>> = Vec::new();
        let push_bucket = |r: usize, s: D::State, b: &mut Vec<Vec<D::State>>| {
            if r >= b.len() {
                b.resize(r + 1, Vec::new());
            }
            b[r].push(s);
        };

        let mut stack: Vec<D::State> = roots.into_iter().collect();
        for s in &stack {
            if seen.insert(s.clone()) {
                push_bucket(D::rank(ctx, s), s.clone(), &mut buckets);
            }
        }
        while let Some(s) = stack.pop() {
            let rs = D::rank(ctx, &s);
            let ns = D::neighbors(ctx, &s);
            debug_assert!(ns.iter().all(|t| D::rank(ctx, t) < rs));
            adj.insert(s.clone(), ns.clone());
            for t in ns {
                if seen.insert(t.clone()) {
                    push_bucket(D::rank(ctx, &t), t.clone(), &mut buckets);
                    stack.push(t);
                }
            }
        }
        Plan { buckets, adj }
    }

    /// 構築済み Plan を使って DP を一発計算
    pub fn solve_with_plan<D: PullDPRules>(
        ctx: &D::Ctx,
        plan: &Plan<D::State>,
    ) -> FxHashMap<D::State, D::Value> {
        let mut val = FxHashMap::<D::State, D::Value>::default();

        for states in plan.buckets.iter() {
            for s in states {
                if let Some(b) = D::base(ctx, s) {
                    val.insert(s.clone(), b);
                    continue;
                }
                let childs = plan.adj.get(s).map(|v| v.as_slice()).unwrap_or(&[]);
                // clone・一時 Vec なしで子 (state, value) を供給
                let v = D::combine(
                    ctx,
                    s,
                    childs.iter().map(|c| ChildRef {
                        state: c,
                        value: val.get(c).expect("child DP value must exist before parent"),
                    }),
                );
                val.insert(s.clone(), v);
            }
        }
        val
    }

    pub fn solve<D: PullDPRules>(
        ctx: &D::Ctx,
        roots: impl IntoIterator<Item = D::State>,
    ) -> FxHashMap<D::State, D::Value> {
        let plan = Self::prepare::<D>(ctx, roots);
        Self::solve_with_plan::<D>(ctx, &plan)
    }
}

// コピペ用のダミー実装
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// struct Dummy;
// struct Problem;

// impl PullDPRules for Problem {
//     type State = Dummy;
//     type Value = Dummy;
//     type Ctx = Dummy;

//     fn rank(ctx: &Self::Ctx, s: &Self::State) -> usize {
//         todo!();
//     }

//     fn neighbors(ctx: &Self::Ctx, s: &Self::State) -> Vec<Self::State> {
//         todo!();
//     }

//     fn base(ctx: &Self::Ctx, s: &Self::State) -> Option<Self::Value> {
//         todo!();
//     }

//     fn combine<'a, I>(ctx: &Self::Ctx, s: &Self::State, childs: I) -> Self::Value
//     where
//         I: IntoIterator<Item = ChildRef<'a, Self::State, Self::Value>>,
//     {
//         todo!();
//     }
// }

mod tests {
    use super::*;
    struct Ctx {
        h: Vec<i64>,
    }
    struct Frog;

    impl PullDPRules for Frog {
        type State = usize;
        type Value = i64;
        type Ctx = Ctx;

        fn rank(_ctx: &Self::Ctx, s: &Self::State) -> usize {
            *s
        }

        fn neighbors(_ctx: &Self::Ctx, s: &Self::State) -> Vec<Self::State> {
            match *s {
                0 => vec![],
                1 => vec![s - 1],
                _ => vec![s - 1, s - 2],
            }
        }

        fn base(_ctx: &Self::Ctx, s: &Self::State) -> Option<Self::Value> {
            match *s {
                0 => Some(0),
                _ => None,
            }
        }

        fn combine<'a, I>(ctx: &Self::Ctx, s: &Self::State, childs: I) -> Self::Value
        where
            I: IntoIterator<Item = ChildRef<'a, Self::State, Self::Value>>,
        {
            let hi = ctx.h[*s];
            childs
                .into_iter()
                .map(|p| *p.value + (hi - ctx.h[*p.state]).abs())
                .min()
                .unwrap()
        }
    }

    // source(https://atcoder.jp/contests/dp/tasks/dp_a)
    #[test]
    fn test_pull_dp() {
        let ctx = Ctx {
            h: vec![10, 30, 40, 20],
        };

        let plan = PullDpEngine::prepare::<Frog>(&ctx, [3]);
        let vals = PullDpEngine::solve_with_plan::<Frog>(&ctx, &plan);
        assert_eq!(vals[&0], 0);
        assert_eq!(vals[&1], 20);
        assert_eq!(vals[&2], 30);
        assert_eq!(vals[&3], 30);
    }
}
