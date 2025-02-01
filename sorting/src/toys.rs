/*
 * Complete the 'maximumToys' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY prices
 *  2. INTEGER k
 */

fn maximum_toys(prices: &[i32], budget: i32) -> i32 {
    let mut prices = prices.to_vec();
    prices.sort();

    let mut total = 0;
    let mut purchases = 0;
    for price in prices {
        if total < budget && (price + total) < budget {
            total += price;
            purchases += 1;
        }
    }

    purchases
}

/* fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let prices: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = maximumToys(&prices, k);

    writeln!(&mut fptr, "{}", result).ok();
} */
