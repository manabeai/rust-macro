use std::collections::HashSet;
use std::hash::Hash;

pub struct MemoizedDFS;
impl MemoizedDFS {
    pub fn search<T, A, FTrans, FGoal, FCollect>(
        start: T,
        trans: FTrans,
        is_goal: FGoal,
        collect: FCollect,
        return_on_first: bool,
    ) -> Vec<A>
    where
        T: Clone + Hash + Eq,
        A: Clone,
        FTrans: Fn(&T) -> Vec<T>,
        FGoal: Fn(&T) -> bool,
        FCollect: Fn(&T) -> A,
    {
        let mut visited = HashSet::new();
        let mut result = vec![];

        fn dfs<T, A, FTrans, FGoal, FCollect>(
            current: T,
            visited: &mut HashSet<T>,
            result: &mut Vec<A>,
            trans: &FTrans,
            is_goal: &FGoal,
            collect: &FCollect,
            return_on_first: bool,
        ) -> bool
        where
            T: Clone + Hash + Eq,
            A: Clone,
            FTrans: Fn(&T) -> Vec<T>,
            FGoal: Fn(&T) -> bool,
            FCollect: Fn(&T) -> A,
        {
            if !visited.insert(current.clone()) {
                return false;
            }

            if is_goal(&current) {
                result.push(collect(&current));
                if return_on_first {
                    return true;
                }
            }

            for next in trans(&current) {
                if dfs(next, visited, result, trans, is_goal, collect, return_on_first) {
                    return true;
                }
            }

            false
        }

        dfs(
            start,
            &mut visited,
            &mut result,
            &trans,
            &is_goal,
            &collect,
            return_on_first,
        );

        result
    }

    pub fn search_with_best<T, A, FTrans, FGoal, FCollect, FCompare>(
        start: T,
        trans: FTrans,
        is_goal: FGoal,
        collect: FCollect,
        is_better: FCompare,
    ) -> Option<A>
    where
        T: Clone + Hash + Eq,
        A: Clone,
        FTrans: Fn(&T) -> Vec<T>,
        FGoal: Fn(&T) -> bool,
        FCollect: Fn(&T) -> A,
        FCompare: Fn(&A, &A) -> bool,
    {
        let mut visited = HashSet::new();
        let mut best: Option<A> = None;

        fn dfs<T, A, FTrans, FGoal, FCollect, FCompare>(
            current: T,
            visited: &mut HashSet<T>,
            best: &mut Option<A>,
            trans: &FTrans,
            is_goal: &FGoal,
            collect: &FCollect,
            is_better: &FCompare,
        ) where
            T: Clone + Hash + Eq,
            A: Clone,
            FTrans: Fn(&T) -> Vec<T>,
            FGoal: Fn(&T) -> bool,
            FCollect: Fn(&T) -> A,
            FCompare: Fn(&A, &A) -> bool,
        {
            if !visited.insert(current.clone()) {
                return;
            }

            if is_goal(&current) {
                let val = collect(&current);
                if best
                    .as_ref()
                    .map_or(true, |b| is_better(&val, b))
                {
                    *best = Some(val);
                }
            }

            for next in trans(&current) {
                dfs(next, visited, best, trans, is_goal, collect, is_better);
            }
        }

        dfs(
            start,
            &mut visited,
            &mut best,
            &trans,
            &is_goal,
            &collect,
            &is_better,
        );

        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_simple_graph() {
        let result = MemoizedDFS::search(
            0,
            |&x| if x < 3 { vec![x + 1] } else { vec![] },
            |&x| x == 3,
            |&x| x,
            false,
        );
        assert_eq!(result, vec![3]);
    }

    #[test]
    fn test_search_multiple_goals() {
        let result = MemoizedDFS::search(
            0,
            |&x| if x < 5 { vec![x + 1, x + 2] } else { vec![] },
            |&x| x >= 3,
            |&x| x,
            false,
        );
        let mut sorted_result = result;
        sorted_result.sort();
        assert_eq!(sorted_result, vec![3, 4, 5]);
    }

    #[test]
    fn test_search_return_on_first() {
        let result = MemoizedDFS::search(
            0,
            |&x| if x < 5 { vec![x + 1, x + 2] } else { vec![] },
            |&x| x >= 3,
            |&x| x,
            true,
        );
        assert_eq!(result.len(), 1);
        assert!(result[0] >= 3);
    }

    #[test]
    fn test_search_no_goals() {
        let result = MemoizedDFS::search(
            0,
            |&x| if x < 3 { vec![x + 1] } else { vec![] },
            |&x| x > 10,
            |&x| x,
            false,
        );
        assert!(result.is_empty());
    }

    #[test]
    fn test_search_cycle_detection() {
        let result = MemoizedDFS::search(
            0,
            |&x| match x {
                0 => vec![1],
                1 => vec![2],
                2 => vec![0], // Creates cycle
                _ => vec![],
            },
            |&x| x == 1,
            |&x| x,
            false,
        );
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_search_string_nodes() {
        let result = MemoizedDFS::search(
            "start".to_string(),
            |s: &String| match s.as_str() {
                "start" => vec!["a".to_string(), "b".to_string()],
                "a" => vec!["goal".to_string()],
                "b" => vec!["goal".to_string()],
                _ => vec![],
            },
            |s: &String| s == "goal",
            |s: &String| s.len(),
            false,
        );
        assert_eq!(result, vec![4, 4]);
    }
}
