use std::collections::{HashMap, HashSet};

use crate::helpers::first_n;

pub fn main(input: String, pt1: bool) {
    let [inits_str, gates_str] = first_n(&mut input.split("\r\n\r\n"));
    let inits = inits_str
        .lines()
        .map(|l| {
            let [a, b] = first_n::<2, _>(&mut l.split(" "));
            (a.split(':').nth(0).unwrap(), b.parse::<i64>().unwrap() == 1)
        })
        .collect::<HashMap<_, _>>();
    let mut gates = gates_str
        .lines()
        .map(|l| {
            let [a, b] = first_n(&mut l.split(" -> "));
            let [w1, gate, w2] = first_n(&mut a.split(' '));
            (HashSet::from_iter([w1, w2]), gate, b)
        })
        .collect::<Vec<_>>();
    let mut wires = gates
        .iter()
        .map(|g| g.0.iter().map(|s|*s).chain([g.2]))
        .flatten()
        .chain(inits.iter().map(|(w, _)| *w))
        .map(|w| (w, None::<bool>))
        .collect::<HashMap<_, _>>();
    if pt1 {
        for (init_w, val) in inits {
            *wires.get_mut(&init_w).unwrap() = Some(val);
            propagate_change(&mut wires, &gates, init_w);
        }
        let mut res = 0i64;
        for (w, val) in wires {
            if w.starts_with('z') && val.is_some_and(|n| n) {
                let n = w.strip_prefix('z').unwrap().parse::<usize>().unwrap();
                res |= 1 << n;
            }
        }

        println!("result is {res}");
    } else {
        // Full adder should look like
        // A XOR B -> t1
        // t1 XOR carry -> ones
        // carry AND t1 -> t2
        // A AND B -> t3
        // t2 OR t3 -> car_out
        // Half adder is just an AND and an XOR

        // Basic algorithm is "go through the adder circuitry until you find something that needs a swap, then do that and restart the search"
        // Restarting because I was having bugs with duplicate swaps and can't be hecked to figure out why

        let mut swaps = vec![];
        let mut done = false;
        let zn = (0..=44)
            .map(|i| "z".to_string() + &format!("{:0>2}", i))
            .collect::<Vec<_>>();
        while !done {
            // Temporarily we'll just assume those are alright
            let mut carry = g_find(Some("x00"), Some("y00"), Some("AND"), None, &gates).unwrap();
            let mut to_swap = None;
            for i in 1..=44 {
                let xg = "x".to_string() + &format!("{:0>2}", i);
                let yg = "y".to_string() + &format!("{:0>2}", i);

                // This should always exist
                let t1 = g_find(Some(&xg), Some(&yg), Some("XOR"), None, &gates).unwrap();

                let t2 = if let Some(t2_test) =
                    g_find(Some(t1), Some(carry), Some("AND"), None, &gates)
                {
                    t2_test
                // vvv Either t1 or carry must be invalid
                } else if let Some(t2_test_2) = g_find(Some(t1), None, Some("AND"), None, &gates) {
                    // Carry is invalid
                    let corr_carry = gates
                        .iter()
                        .find(|c| c.1 == "AND" && c.2 == t2_test_2)
                        .unwrap()
                        .0
                        .iter()
                        .find(|w| **w != t1)
                        .unwrap();
                    swaps.extend([carry, corr_carry]);
                    to_swap = Some((carry.to_string(), corr_carry.to_string()));
                    break;
                } else {
                    let t2_test_2 = g_find(Some(carry), None, Some("AND"), None, &gates).unwrap();
                    // t1 is invalid
                    let corr_t1 = gates
                        .iter()
                        .find(|c| c.1 == "AND" && c.2 == t2_test_2)
                        .unwrap()
                        .0
                        .iter()
                        .find(|w| **w != carry)
                        .unwrap();
                    swaps.extend([t1, corr_t1]);
                    to_swap = Some((t1.to_string(), corr_t1.to_string()));
                    break;
                };

                // This is obviously ok as well
                let t3 = g_find(Some(&xg), Some(&yg), Some("AND"), None, &gates).unwrap();

                let car_out = if let Some(car_out_test) =
                    g_find(Some(t2), Some(t3), Some("OR"), None, &gates)
                {
                    car_out_test
                } else if let Some(carr_test_2) = g_find(Some(t2), None, Some("OR"), None, &gates) {
                    // t3 is invalid
                    let corr_t3 = gates
                        .iter()
                        .find(|c| c.1 == "OR" && c.2 == carr_test_2)
                        .unwrap()
                        .0
                        .iter()
                        .find(|w| **w != t2)
                        .unwrap();
                    swaps.extend([t3, corr_t3]);
                    to_swap = Some((t3.to_string(), corr_t3.to_string()));
                    break;
                } else {
                    let carr_test_2 = g_find(Some(t3), None, Some("OR"), None, &gates).unwrap();
                    // t2 is invalid
                    let corr_t2 = gates
                        .iter()
                        .find(|c| c.1 == "OR" && c.2 == carr_test_2)
                        .unwrap()
                        .0
                        .iter()
                        .find(|w| **w != t3)
                        .unwrap();
                    swaps.extend([t2, corr_t2]);
                    to_swap = Some((t2.to_string(), corr_t2.to_string()));
                    break;
                };

                // At this point we can be alright that carry and t1 both are ok, so this is valid
                let out = g_find(Some(carry), Some(t1), Some("XOR"), None, &gates).unwrap();

                carry = car_out;
                if out != "z".to_string() + &format!("{:0>2}", i) {
                    swaps.extend([&zn[i], out]);
                    swap(&mut gates, out, &zn[i]);
                    break;
                }
                if i == 44 {
                    done = true;
                }
            }
            if let Some((s1, s2)) = to_swap {
                // lifetime hackery
                let s1_lasting = wires.iter().find(|w|*w.0==s1).unwrap().0;
                let s2_lasting = wires.iter().find(|w|*w.0==s2).unwrap().0;
                swap(&mut gates, s1_lasting, s2_lasting);
            }
        }
        swaps.sort();
        println!("Swaps are {}", swaps.join(","))
    }
}
fn g_find<'a>(
    i1: Option<&str>,
    i2: Option<&str>,
    gate: Option<&str>,
    o: Option<&str>,
    gates: &Vec<(HashSet<&str>, &str, &'a str)>,
) -> Option<&'a str> {
    gates
        .iter()
        .find(|c| {
            i1.is_none_or(|iv| c.0.contains(&iv))
                && i2.is_none_or(|iv| c.0.contains(&iv))
                && gate.is_none_or(|g| c.1 == g)
                && o.is_none_or(|out| c.2 == out)
        })
        .map(|c| c.2)
}
fn swap<'a>(gates: &mut Vec<(HashSet<&str>, &str, &'a str)>, s1: &'a str, s2: &'a str) {
    for g in gates {
        if g.2 == s1 {
            g.2 = s2;
        } else if g.2 == s2 {
            g.2 = s1;
        }
    }
}

fn propagate_change<'a>(
    wires: &mut HashMap<&'a str, Option<bool>>,
    gates: &Vec<(HashSet<&'a str>, &str, &'a str)>,
    change_to: &'a str,
) {
    let mut to_propagate = vec![];
    for gate in gates {
        if gate.0.contains(&change_to) && gate.0.iter().all(|w| wires.get(w).unwrap().is_some()) {
            let [v1, v2] = first_n(&mut gate.0.iter().map(|w| wires.get(w).unwrap().unwrap()));
            *wires.get_mut(&gate.2).unwrap() = Some(match gate.1 {
                "XOR" => v1 ^ v2,
                "AND" => v1 && v2,
                "OR" => v1 || v2,
                g => panic!("Invalid gate {g}"),
            });
            to_propagate.push(gate.2);
        }
    }
    for p in to_propagate {
        propagate_change(wires, gates, p);
    }
}
