fn hash(b: &[u8]) -> usize {
    b.iter()
        .fold(0u8, |a, &c| a.wrapping_add(c).wrapping_mul(17)) as usize
}

pub fn part1(input: &str) -> usize {
    input.split(',').map(|s| s.as_bytes()).map(hash).sum()
}

pub fn part2(input: &str) -> usize {
    let mut hashmap = vec![vec![]; 256];
    for op in input.split(',') {
        let op = op.as_bytes();
        match op.last().unwrap() {
            b'-' => {
                let b = hash(&op[..op.len() - 1]);
                if let Some(idx) = hashmap[b]
                    .iter()
                    .position(|(n, _)| *n == &op[..op.len() - 1])
                {
                    hashmap[b].remove(idx);
                }
            }
            _ => {
                let mut it = op.splitn(2, |&b| b == b'=');
                let (lens_name, power) = (it.next().unwrap(), it.next().unwrap()[0] - 48);
                let b = hash(lens_name);
                if let Some(idx) = hashmap[b].iter().position(|&(n, _)| n == lens_name) {
                    hashmap[b][idx].1 = power;
                } else {
                    hashmap[b].push((lens_name, power));
                }
            }
        }
    }
    hashmap
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, l)| (i + 1) * (j + 1) * l.1 as usize)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(1320, part1(input));
    }

    #[test]
    fn test_p2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(145, part2(input));
    }
}
