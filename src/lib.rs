pub fn nth(n: u32) -> u32 {
    const UPPER_BOUND: usize = 1000002;

    //make prime list
    // ---vector collection attempt---
    /*
        let mut prime_distiller = Vec::new();
        dbg!(&prime_distiller);
        let mut i: u32 = 2;
        while i <= UPPER_BOUND {
            prime_distiller.push(i);
            i = i + 1;
        }

        for num in prime_distiller {
            if let Some(pos) = prime_distiller.iter().position(|x| *x == *num) {
                    prime_distiller.remove(pos);
            }

            let mut count: u32 = 1
            while (count * num <= UPPER_BOUND) {
                num + (num * count)
            }
        }
    */
    // ---boolean array collection---

    //set array of upper_bound length to all true.
    let mut numbers: [bool; UPPER_BOUND] = [true; UPPER_BOUND];

    // create index_count which will work as representations of the numbers to be tested.
    let mut index_count: usize = 0;

    // test each index_count (number) for primality
    while index_count < UPPER_BOUND {
        // make factor variable, beginning with 2
        let mut factor: usize = 2;
        while factor as f32 <= (index_count as f32).sqrt() {
            //this is the crashing macro ---->dbg!(factor);
            //if index is not prime...
            if index_count % factor == 0 {
                //set array value to false at this index.
                numbers[index_count] = false;
                // then break
                break;
            }
            factor = factor + 1
        }
        index_count = index_count + 1
    }

    //find nth prime / create array of primes.
    //create empty Vec for pushing primes into.
    let mut accumulated_primes = Vec::new();
    let mut prime_query_index: usize = 2;
    while prime_query_index < UPPER_BOUND {
        if numbers[prime_query_index] == true {
            accumulated_primes.push(prime_query_index as u32);
        }
        prime_query_index = prime_query_index + 1
    }

    accumulated_primes[n as usize]

    /* --------- thoughts----------
    // hardcode or input upper bound?
    // to check for factors could go on forever 2*2*2*2*2 etc never ends
    // if *deleting* numbers and making set smaller (attrition), all you would know is it's not there so don't delete...
    // Vec is an array that can be changed in length
    // Hashmap (set?) might be interesting to play with too
    // the way I was thinking of this is more of an attritional *composite test*
    // in the case of an array (or a set?) you could just instantiate booleans, and the search could end there were no indicies; you could just iterate over the array or vec or whatever
    // this seems faster probably? the indicies will represent the numbers themselves
    // possible gotcha with indicies starting at [0]
    // this could run a test for prime against every number (primality, probably/maybe slower??) or to do a compositing iterative marking of the numbers ...
    // there is also trial division ... test prime integers from 2 to sqrt of n
    */
}
