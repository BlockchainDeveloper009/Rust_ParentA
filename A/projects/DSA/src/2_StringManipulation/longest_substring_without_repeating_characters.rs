use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.ln();
    let mut set = HashSet::new(); // To keep track of unique characters in the window
    let mut left = 0;             // Left pointer of the window
    let mut max_len = 0;          // Maximum length found

    for right in 0..n {
        // If duplicate character, move left pointer until substring is unique
        while set.contains(&chars[right]) {
            set.remove(&chars[left]);
            left += 1;
        }
        set.insert(chars[right]);
        // Update maximum length
        max_len = max_len.max(right - left + 1);
    }

    max_len as i32
}
// Example usage
// 00.32000000000000000000000000000000
.
