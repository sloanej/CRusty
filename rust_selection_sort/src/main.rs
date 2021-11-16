use std::env;
use rand::Rng;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let len:usize = args[1].parse::<usize>().unwrap();
        let mut arr = Vec::<i32>::with_capacity(len);
        let mut rng = rand::thread_rng();
        for _i in 0..len {
            arr.push(rng.gen::<i32>());
        }
        selection_sort(& mut arr[..]);
        println!("Sorted :)");
    }
    else {
        println!("Please specify a length");
    }
    
}

fn selection_sort(vals: &mut [i32]) {
    for i in 0..vals.len(){
        let min = find_min(vals, i);
        vals.swap(i, min);
    }
}

fn find_min(vals: &[i32], start:usize) -> usize {
    let mut smallest = start;
    for i in (start + 1)..vals.len(){
        if vals[i] < vals[smallest]{
            smallest = i;
        }
    }
    return smallest;
}