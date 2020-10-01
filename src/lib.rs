pub fn nth(n: u32) -> u32 {
	const UPPER_BOUND: usize = 1000002;

//make prime list
// ---boolean array collection---

	//set array of upper_bound length to all true.
	let mut numbers: [bool; UPPER_BOUND] = [true; UPPER_BOUND];

		// create index_count which will work as representations of the numbers to be tested.
		let mut index_count: usize = 0;

		// test each index_count (number) for primality
		while index_count < UPPER_BOUND {

			// make factor variable, beginning with 2
			let mut factor: usize = 2;
			// test while factor variable is less than square root of number to be tested, to avoid repition (eg 2*4 = 8 and 4*2 = 8)
			while factor as f32 <= (index_count as f32).sqrt(){

				//this is the crashing macro ---->dbg!(factor);
				//if index is not prime... 
				if index_count % factor == 0 {
					//set array value to false at this index.
					numbers[index_count] = false;
					// then break
					break
				}
				factor = factor + 1
			}
			index_count = index_count + 1
		}

//find nth prime 
	//create empty Vec for pushing primes into.
	let mut accumulated_primes = Vec::new();
	// make prime_query_index variable, starting with 2 (to avoid including 0 and 1, which are not prime)
	let mut prime_query_index: usize = 2;
	// check entire range of tested numbers
	while prime_query_index < UPPER_BOUND {
		// if number previously was not converted to false by test above
		if numbers[prime_query_index] == true {
			//push the index (synchronized with prime_query_index) into accumulated_primes
			accumulated_primes.push(prime_query_index as u32);
		}
		prime_query_index = prime_query_index + 1
	} 

// return requested prime by index provided as parameter
accumulated_primes[n as usize]

}
