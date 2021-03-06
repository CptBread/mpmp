use std::env;

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

	// We rely on that any number in a "fibonacci like" sequence, starting with X and Y,
	// can be calculated by "f(n) = X * Fib(n - 1) + Y * Fib(n)" except for f(0) which equals X
	let mut high = *fib.last().unwrap();
	for low in fib.iter().rev().skip(1) {
		for n in 1..target {
			for i in n..target {
				// This will produce the lower value of the two so if that is above the limit then break
				let v = high.saturating_mul(n).saturating_add(low.saturating_mul(i));
				if v > target {
					break;
				}
				else if v == target {
					// println!("{} * {} + {} * {}", n, high, i, low);
					return Some((n, i));
				}
				if high.saturating_mul(i).saturating_add(low.saturating_mul(n)) == target {
					// println!("{} * {} + {} * {}", n, low, high, i);
					return Some((i, n));
				}
			}
		}
		high = *low;
	}
	None
}

