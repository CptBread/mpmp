use std::env;
use std::io;

pub fn run() {
	let target = env::args().skip(1).next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(1_000_000);
	let res = find_pair(target);
	if let Some((n, i)) = res {
		let mut last = i;
		let mut curr = n;
		println!("{}", n);
		println!("{}", i);
		loop {
			let n = curr + last;
			println!("{}", n);
			if n >= target {
				break;
			}
			last = curr;
			curr = n;
		}
	}
	else {
		println!("Failed to find a valid pair!");
	}
}

fn find_pair(target:u32) -> Option<(u32, u32)> {
	// We can't allow MAX as target as that would make us belive we reach the target when we saturate
	assert!(target != std::u32::MAX);

	// Generate fibonacci upto target
	let mut fib = vec![1];
	let mut last = 1;
	let mut curr = 1;
	while curr <= target {
		fib.push(curr);
		let n = curr + last;
		last = curr;
		curr = n;
	}

	let mut high = *fib.last().unwrap();
	for low in fib.iter().rev().skip(1) {
		// let curr = *curr;
		for n in 1..target {
			for i in n..target {
				// This will produce the lower value of the two so if that is above the limit then break
				let v = high.saturating_mul(n).saturating_add(low.saturating_mul(i));
				if v > target {
					break;
				}
				else if v == target {
					return Some((n, i));
				}
				if high * i + low * n == target {
					return Some((i, n));
				}
			}
		}
		high = *low;
	}
	None
}