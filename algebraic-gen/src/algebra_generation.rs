// the functions are not optimal performance wise, the focus is on clarity

// Canonizes a given element and returns the sign.
pub fn canonize(element: &mut Vec<usize>) -> bool {
    // swapping and eliminating duplicates flips sign
    let mut negate = false;

    // sort with bubblesort, keeping track of sign
    for _ in 0..element.len() {
        for i in 1..element.len() {
            if element[i - 1] > element[i] {
                element.swap(i - 1, i);
                negate ^= true;
            }
        }
    }

    // remove pairs of duplicates, again tracking sign
    for _ in 0..element.len() {
        for i in 1..element.len() {
            if element[i - 1] == element[i] {
                element.remove(i);
                element.remove(i - 1);
                negate ^= true;
                break;
            }
        }
    }

    negate
}

pub fn wedge_product(a: &[usize], b: &[usize]) -> Vec<usize> {
    a.iter().chain(b.iter()).cloned().collect()
}

// Generates all the elements of a geometric algebra of a given dimension.
pub fn generate_elements(dimension: usize) -> Vec<Vec<usize>> {
    // starting with scalar
    let mut elements: Vec<Vec<usize>> = vec![vec![]];

    for d in 0..dimension {
        // these are the basis vectors
        elements.push(vec![d]);

        // now generate all the bi- tri- etc. vectors up to pseudoscalar
        loop {
            let mut new = Vec::new();

            // slow. but offline
            for a in &elements {
                for b in &elements {
                    let mut c = wedge_product(a, b);
                    canonize(&mut c); // we don't care about sign here

                    // careful not to add duplicates
                    if !elements.contains(&c) && !new.contains(&c) {
                        new.push(c);
                    }
                }
            }
            if new.is_empty() {
                break;
            }
            elements.extend(new.into_iter());
        }
    }

    elements
}

// Generates the geometric product
// basically, multiply out and see to which element is contributed (and under
// which sign)
pub fn generate_product_sums(elements: &[Vec<usize>]) -> Vec<Vec<(bool, usize, usize)>> {
    let mut sums: Vec<Vec<(bool, usize, usize)>> = vec![vec![]; elements.len()];
    for (a_i, a) in elements.iter().enumerate() {
        for (b_i, b) in elements.iter().enumerate() {
            let mut c = wedge_product(a, b);
            let negate = canonize(&mut c);
            let c_i = elements.iter().position(|s| *s == c).unwrap();
            sums[c_i].push((negate, a_i, b_i));
        }
    }
    sums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonization() {
        let mut e = vec![];
        assert!(!canonize(&mut e));
        assert_eq!(e, vec![]);

        let mut e = vec![0, 0];
        assert!(canonize(&mut e));
        assert_eq!(e, vec![]);

        let mut e = vec![1, 0];
        assert!(canonize(&mut e));
        assert_eq!(e, vec![0, 1]);

        let mut e = vec![2, 1, 0];
        assert!(canonize(&mut e));
        assert_eq!(e, vec![0, 1, 2]);

        let mut e = vec![0, 2, 0];
        assert!(!canonize(&mut e));
        assert_eq!(e, vec![2]);
    }

    #[test]
    fn element_generation() {
        assert_eq!(
            generate_elements(3),
            vec![
                vec![],
                vec![0],
                vec![1],
                vec![0, 1],
                vec![2],
                vec![0, 2],
                vec![1, 2],
                vec![0, 1, 2],
            ]
        )
    }

    #[test]
    fn product_sum_generation() {
        let elements = generate_elements(2);
        assert_eq!(
            generate_product_sums(&elements),
            vec![
                vec![
                    (false, 00, 00),
                    (true, 01, 01),
                    (true, 02, 02),
                    (true, 03, 03)
                ],
                vec![
                    (false, 00, 01),
                    (false, 01, 00),
                    (false, 02, 03),
                    (true, 03, 02)
                ],
                vec![
                    (false, 00, 02),
                    (true, 01, 03),
                    (false, 02, 00),
                    (false, 03, 01)
                ],
                vec![
                    (false, 00, 03),
                    (false, 01, 02),
                    (true, 02, 01),
                    (false, 03, 00)
                ],
            ]
        );
    }
}
