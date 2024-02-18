use serde::{Deserialize, Serialize};

type BigNum = u64;
type BigNumOutput = String;

struct Output {
    length: u64,
    biggest: BigNum,
}

fn three_n_calc(mut n: u64) -> Output {
    let mut length = 1;
    let mut biggest = n;

    while n > 1 {
        if n % 2 == 0 {
            n >>= 1;
        } else {
            n = n * 3 + 1;

            if n > biggest {
                biggest = n;
            }
        }
        length += 1;
    }
    Output { length, biggest }
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
struct Solution<T> {
    n: u64,
    value: T,
}

#[derive(Deserialize, Serialize)]
pub struct ThreeNResponse {
    from: u64,
    to: u64,
    length_vec: Vec<Solution<u64>>,
    biggest_vec: Vec<Solution<BigNumOutput>>,
}

pub fn three_n(from: u64, to: u64) -> ThreeNResponse {
    let mut length_vec = vec![];
    let mut biggest_vec = vec![];

    let mut longest = 0;
    let mut biggest: BigNum = 0;

    for n in from..=to {
        let output = three_n_calc(n);

        if output.length > longest {
            length_vec.push(Solution {
                n,
                value: output.length,
            });
            longest = output.length;

            eprintln!("new longest: n = {}, length = {}", n, longest);
        }

        if output.biggest > biggest {
            biggest_vec.push(Solution {
                n,
                value: output.biggest.to_string(),
            });
            biggest = output.biggest;

            eprintln!("new biggest: n = {}, biggest = {}", n, biggest);
        }
    }

    ThreeNResponse {
        from,
        to,
        length_vec,
        biggest_vec,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let from = 1;
        let to = 5;

        let result = three_n(from, to);

        let lengths_expected = vec![
            Solution { n: 1, value: 1 },
            Solution { n: 2, value: 2 },
            Solution { n: 3, value: 8 },
        ];

        let biggests_expected = vec![
            Solution {
                n: 1,
                value: "1".to_owned(),
            },
            Solution {
                n: 2,
                value: "2".to_owned(),
            },
            Solution {
                n: 3,
                value: "16".to_owned(),
            },
        ];

        assert_eq!(result.from, from);
        assert_eq!(result.to, to);
        assert_eq!(result.length_vec, lengths_expected);
        assert_eq!(result.biggest_vec, biggests_expected);
    }
}
