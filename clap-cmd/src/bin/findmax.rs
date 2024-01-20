use std::time::Instant;

fn main() {
    let mut arr = vec![1, 25, -4, 10, 18, 67, 43, 27, -78];
    for i in 1..10000 {
        arr.push(i);
    }
    let now = Instant::now();

    let max = find_max(&arr);
    println!(
        "max value = {} use = {:?}",
        max.unwrap_or_default(),
        now.elapsed()
    );
    let now = Instant::now();
    println!(
        "max value orignal = {:?} use = {:?}",
        arr.iter().max(),
        now.elapsed()
    );
}

fn find_max(arr: &[i32]) -> Option<i32> {
    // println!("map = {:?}", arr);
    // match arr.iter().max() {
    //     Some(a) => Some(*a),
    //     None => None
    // }
    const THRESHOLD: usize = 2;
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);
    // find_max(left).max(find_max(right))

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));
        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;
        Some(max_l.max(max_r))
    })
    .unwrap()
}
