use std::fmt::Write;

/// Generate the hash for the input lengths
pub fn generate_hash(lengths: &[usize], buffersize: usize, rounds: usize) -> Vec<usize> {
    let mut nbrs: Vec<usize> = (0..buffersize).collect();
    let mut current_position = 0;
    let mut skipsize = 0;

    for _ in 0..rounds {
        for &length in lengths {
            let n = length + skipsize;
            nbrs[0..length].reverse();
            nbrs.rotate_left(n % buffersize);
            current_position += n;
            skipsize += 1;
        }
    }
    nbrs.rotate_right(current_position % buffersize);
    nbrs
}

/// Hash a string with the knot-hash algorithm: ASCII lengths, 64 rounds, xor-dense hexadecimal output.
pub fn get_knot_hash(s: &str, buffer_len: usize) -> String {
    let mut lengths: Vec<usize> = s.chars().map(|c| c as usize).collect();
    lengths.extend([17, 31, 73, 47, 23]);
    let sparse = generate_hash(&lengths, buffer_len, 64);
    let mut dense = String::new();

    for block in sparse.as_chunks::<16>().0 {
        let n = block.iter().fold(0, |a, i| a ^ i);
        let _ = write!(&mut dense, "{n:02x}");
    }
    dense
}
