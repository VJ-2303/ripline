use std::collections::HashMap;

pub fn build_bad_char_table(pattern: &str) -> HashMap<char, usize> {
    let mut table: HashMap<char, usize> = HashMap::new();
    let len = pattern.len();

    if len == 0 {
        return table;
    }

    for (i, c) in pattern.chars().enumerate().take(len - 1) {
        let shift = len - 1 - i;
        table.insert(c, shift);
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_char_table() {
        let pattern = "TOOTH";
        let table = build_bad_char_table(pattern);

        // 'T' is at index 0 and 3. Last one is 3.
        // Shift = 5 - 1 - 3 = 1.
        assert_eq!(table.get(&'T'), Some(&1));

        // 'O' is at index 1 and 2. Last one is 2.
        // Shift = 5 - 1 - 2 = 2.
        assert_eq!(table.get(&'O'), Some(&2));

        // 'H' is the last char, so it was skipped in the loop.
        // It should NOT be in the table (it gets default shift later).
        assert_eq!(table.get(&'H'), None);

        // 'Z' is not in pattern.
        assert_eq!(table.get(&'Z'), None);
    }
}
