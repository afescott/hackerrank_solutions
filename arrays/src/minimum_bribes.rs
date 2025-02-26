fn minimumBribes(q: &[i32]) {
    let mut bribe_counter = 0;

    let mut single_bribe_count = 0;
    let mut more_than_3 = false;
    for i in 0..q.len() - 1 {
        let mut j = i;
        if i != q.len() {
            while q[i] > q[j + 1] {
                if q[i] > q[j + 1] {
                    bribe_counter += 1;
                    single_bribe_count += 1;
                }
                j += 1;
                if j == q.len() - 1 {
                    break;
                }
            }
        }
        if single_bribe_count > 2 {
            more_than_3 = true;
        }
        single_bribe_count = 0;
    }

    if more_than_3 {
        println!("Too chaotic");
    } else {
        println!("{:?}", bribe_counter);
    }
}
