use crate::helpers::first_n;

pub fn main(input: String, pt1: bool) {
    let eqns = input
        .lines()
        .map(|l| {
            let [total_str, nums_str] = first_n(&mut l.split(": "));
            (
                total_str.parse::<i64>().unwrap(),
                nums_str
                    .split(" ")
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let mut possible = 0;
    for (total, nums) in eqns {
        let mut opts = vec![0; nums.len() - 1];
        loop {
            //println!("Trying {total} <- {nums:?} ({i:b})");
            let mut sum = nums[0];
            for (ni, n) in nums.iter().skip(1).enumerate() {
                //print!("{ni},{n} ");
                // If the bit is 1, multiply, else add
                match opts[ni] {
                    2 => sum = (sum.to_string()+&n.to_string()).parse::<i64>().unwrap(),
                    1 => sum *= n,
                    0 => sum += n,
                    _ => panic!("Invalid")
                }
            }
            if sum == total {
                //println!("{total} <- {nums:?} is possible ({i:b})");
                possible += total;
                break;
            }
            opts[0] += 1;
            let mut done = false;
            for n in 0..opts.len() {
                if opts[n] >= if pt1 {2} else {3} {
                    opts[n] = 0;
                    if let Some(o) = opts.get_mut(n+1) {
                        *o += 1;
                    } else {
                        // We've overflown the array, we're done
                        done = true;
                    }
                }
            }
            if done {
                break;
            }
        }
    }
    println!("Possible: {possible}");
}
