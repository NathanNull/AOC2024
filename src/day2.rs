pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn main(input: String, pt1: bool) {
    let reports = input
        .split("\r\n")
        .map(|a| {
            a.split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let safe = reports
        .into_iter()
        .map(|report| {
            if pt1 {
                test_safety(report)
            } else {
                if test_safety(report.clone()) {
                    return true;
                }
                // awful brute force nonsense
                // works though, and it's 10pm so I can't think of anything better
                for i in 0..report.len() {
                    let mut test_rep = report.clone();
                    test_rep.remove(i);
                    if test_safety(test_rep) {
                        return true;
                    }
                }
                return false;
            }
        })
        .collect::<Vec<_>>();
    println!("{}", safe.iter().filter(|n| **n).count())
}

fn test_safety(report: Vec<i64>) -> bool {
    // println!("Considering {report:?}");
    let inc = report[0] < report[1];
    let mut curr = report[0];
    for i in 1..=report.len() - 1 {
        let next = report[i];
        if ((curr < next) ^ inc) || (curr - next).abs() < 1 || (curr - next).abs() > 3 {
            return false;
        } else {
            curr = next;
        }
    }
    return true;
}
