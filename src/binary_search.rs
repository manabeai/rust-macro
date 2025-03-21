pub fn binary_search<F>(mut ng: isize, mut ok: isize, f: F) -> isize
where
    F: Fn(isize) -> bool,
{
    while (ng - ok).abs() > 1 {
        let mid = (ng + ok) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
