//sort this array
// [2,1,3,1,2] ->
// iterates twice as 2 the next value
fn countInversions(arr: &mut [i32]) -> i64 {
    println!("starting array: {:?}", arr);
    //whenever you're iterating through an array
    let mut counter = 0;
    for i in 0..arr.len() - 1 {
        let mut next_val = false;
        let mut val = i;
        if val != arr.len() {
            while !next_val {
                /*                 println!("swap: {:?} with: {:?}?", arr[val], arr[val + 1]); */
                if arr[val] > arr[val + 1] {
                    arr.swap(val, val + 1);
                    counter += 1;
                    check_smaller_valu(arr, val, &mut counter);
                    println!("{:?}", arr);
                } else {
                    /*                     println!("{:?} < {:?}", arr[val], arr[val + 1]); */
                    next_val = true;
                }
            }
        }
    }

    counter
}

fn check_smaller_valu(arr: &mut [i32], mut index: usize, counter: &mut i64) {
    let mut next_val = false;

    while !next_val {
        if index != 0 {
            if arr[index] < arr[index - 1] {
                arr.swap(index, index - 1);
                index -= 1;
                *counter += 1;

                println!("{:?}", arr);
            } else {
                /*                     println!("{:?} < {:?}", arr[val], arr[val + 1]); */
                next_val = true;
            }
        } else {
            next_val = true;
        }
    }
}
