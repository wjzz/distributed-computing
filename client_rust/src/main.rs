mod three_n;

use three_n::three_n;

fn main() {
    let result = three_n(1, 1000);
    let json = serde_json::to_string_pretty(&result).unwrap();
    println!("{}", json);
}
