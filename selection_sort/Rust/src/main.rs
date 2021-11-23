use std::env;
use std::time::Instant;
use rand::Rng;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let len:usize = args[1].parse::<usize>().unwrap();
        let mut arr = create_arr(len);
        selection_sort(& mut arr[..]);
    }
    else {
        selection_sort(& mut create_arr(100)[..]);
        selection_sort(& mut create_arr(1000)[..]);
        selection_sort(& mut create_arr(10000)[..]);
        selection_sort(& mut create_arr(30000)[..]);
        selection_sort(& mut create_arr(70000)[..]);
        selection_sort(& mut create_arr(100000)[..]);
        selection_sort(& mut create_arr(150000)[..]);

    }
    
}

fn create_arr(len: usize) -> Vec<i32>{//&'static mut [i32]{
    let mut arr = Vec::<i32>::with_capacity(len);
    let mut rng = rand::thread_rng();
    for _i in 0..len {
        arr.push(rng.gen::<i32>());
    }
    arr
}

fn selection_sort(vals: &mut [i32]) {
    let now = Instant::now();
    for i in 0..vals.len(){
        let min = find_min(vals, i);
        vals.swap(i, min);
    }
    let total_time = now.elapsed();
    println!("Sorting with {} elements took {} seconds",
        vals.len(), total_time.as_millis() as f64 / 1000 as f64);
}

fn find_min(vals: &[i32], start:usize) -> usize {
    let mut smallest = start;
    for i in (start + 1)..vals.len(){
        if vals[i] < vals[smallest]{
            smallest = i;
        }
    }
    smallest
}