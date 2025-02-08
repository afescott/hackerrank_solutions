fn rotLeft(array: &[i32], rotations: i32) -> Vec<i32> {
    let mut new_array = array.to_vec();
    for _ in 0..rotations {
        let val = new_array.first();
        let mut last_val = 0;
        if let Some(val) = val {
            last_val = new_array[new_array.len() - 1];
            new_array[array.len() - 1] = *val;
        }
        for i in 0..new_array.len() - 1 {
            if i == new_array.len() - 2 {
                new_array[i] = last_val;
            } else {
                new_array[i] = new_array[i + 1];
            }
            println!("rotate i: {:?}", new_array);
        }
        println!();
    }

    new_array
}
