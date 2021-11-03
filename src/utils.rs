#![allow(unused_macros)]
macro_rules! chmin {
  ($base:expr, $($cmps:expr),+ $(,)*) => {{
      let cmp_min = min!($($cmps),+);
      if $base > cmp_min {
          $base = cmp_min;
          true
      } else {
          false
      }
  }};
}

macro_rules! chmax {
  ($base:expr, $($cmps:expr),+ $(,)*) => {{
      let cmp_max = max!($($cmps),+);
      if $base < cmp_max {
          $base = cmp_max;
          true
      } else {
          false
      }
  }};
}

macro_rules! min {
  ($a:expr $(,)*) => {{
      $a
  }};
  ($a:expr, $b:expr $(,)*) => {{
      std::cmp::min($a, $b)
  }};
  ($a:expr, $($rest:expr),+ $(,)*) => {{
      std::cmp::min($a, min!($($rest),+))
  }};
}

macro_rules! max {
  ($a:expr $(,)*) => {{
      $a
  }};
  ($a:expr, $b:expr $(,)*) => {{
      std::cmp::max($a, $b)
  }};
  ($a:expr, $($rest:expr),+ $(,)*) => {{
      std::cmp::max($a, max!($($rest),+))
  }};
}

#[allow(dead_code)]
fn counter_vi(arr: Vec<i64>) -> std::collections::HashMap<i64, usize> {
    let mut cnt = std::collections::HashMap::<i64, usize>::new();
    for i in arr {
        *cnt.entry(i).or_insert(0) += 1;
    }
    return cnt;
}

#[allow(dead_code)]
fn counter_vu(arr: Vec<usize>) -> std::collections::HashMap<usize, usize> {
    let mut cnt = std::collections::HashMap::<usize, usize>::new();
    for i in arr {
        *cnt.entry(i).or_insert(0) += 1;
    }
    return cnt;
}

#[allow(dead_code)]
fn counter_vs(arr: Vec<String>) -> std::collections::HashMap<String, usize> {
    let mut cnt = std::collections::HashMap::<String, usize>::new();
    for i in arr {
        *cnt.entry(i).or_insert(0) += 1;
    }
    return cnt;
}

#[cfg(test)]
mod tests {
    #[test]
    fn chmax_test() {
        let mut a: usize = 0;
        let res = chmax!(a, 10);
        assert_eq!(a, 10);
        assert_eq!(res, true);

        let res = chmax!(a, 0);
        assert_eq!(a, 10);
        assert_eq!(res, false);
    }
}
