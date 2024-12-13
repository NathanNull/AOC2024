use crate::helpers::first_n;

pub fn main(input: String, pt1: bool) {
    let behaviours = input
        .split("\r\n\r\n")
        .map(|b| {
            // string parsing. you do what you gotta do
            let [ba, bb, prize] = first_n(&mut b.lines());
            let mv_a = first_n::<2, _>(&mut ba.split(": ").nth(1).unwrap().split(", "))
                .map(|s| s.split("+").nth(1).unwrap().parse::<i64>().unwrap());
            let mv_b = first_n::<2, _>(&mut bb.split(": ").nth(1).unwrap().split(", "))
                .map(|s| s.split("+").nth(1).unwrap().parse::<i64>().unwrap());
            let p_loc = first_n::<2, _>(&mut prize.split(": ").nth(1).unwrap().split(", "))
                .map(|s| s.split("=").nth(1).unwrap().parse::<i64>().unwrap());
            (mv_a, mv_b, p_loc)
        })
        .collect::<Vec<_>>();
    let mut tokens = 0;
    for (mv_a, mv_b, mut p_loc) in behaviours.into_iter() {
        if !pt1 {
            p_loc = [p_loc[0] + 10000000000000, p_loc[1] + 10000000000000];
        }
        let opt = optimal_presses(mv_a, mv_b, p_loc);
        tokens += opt.0 * 3;
        tokens += opt.1;
    }
    println!("Optimal token count is {tokens}")
}

fn optimal_presses(mv_a: [i64; 2], mv_b: [i64; 2], p_loc: [i64; 2]) -> (i64, i64) {
    // x = ax*ap+bx*bp, y = ay*ap+by*bp, solve for ap and bp
    // thx wolframalpha
    let ap = (mv_b[1] * p_loc[0] - mv_b[0] * p_loc[1]) / (mv_b[1] * mv_a[0] - mv_b[0] * mv_a[1]);
    let bp = (mv_a[1] * p_loc[0] - mv_a[0] * p_loc[1]) / (mv_a[1] * mv_b[0] - mv_a[0] * mv_b[1]);
    let (ap, bp) = (ap as i64, bp as i64);
    if [mv_a[0] * ap + mv_b[0] * bp, mv_a[1] * ap + mv_b[1] * bp] == p_loc {
        return (ap, bp);
    } else {
        return (0, 0);
    }
}