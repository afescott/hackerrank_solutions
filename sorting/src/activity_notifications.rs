fn main() {
    let expenditure = &[2, 3, 4, 2, 3, 6, 8, 4, 5].to_vec();

    activityNotifications(expenditure, 5);
}
//2x the median income for a trailing nubmer of days, notification sent
fn activityNotifications(expenditure: &[i32], days: i32) -> i32 {
    let mut copy = expenditure.to_vec();

    copy.sort();
    println!("{:?}", copy);
    let expenditure = expenditure.iter().map(|x| *x as f64);
    let expenditure = expenditure.enumerate();

    let mut notifications = 0;
    for (counter, val) in expenditure {
        if (counter + 1) >= days as usize {
            let median = calc(&copy[..counter]);
            println!("median: {:?}", median);
            if val >= (2.0 * median) {
                println!("val: {:?}, median:{:?}", val, (2.0 * median));
                notifications += 1;
            }
        }
    }

    notifications
}

pub fn calc(expenditure: &[i32]) -> f64 {
    println!("{:?}", expenditure);
    let mid = expenditure.len() / 2;
    if expenditure.len() % 2 == 0 {
        println!("mid: {:?}", mid);
        println!("1: {:?}, 2: {:?}", expenditure[mid - 1], expenditure[mid]);
        ((expenditure[mid - 1] + expenditure[mid]) / 2).into() // Average of two middle values
    } else {
        expenditure[mid].into() // Middle value
    }
}
