fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn longest_substring(s: &str) -> &str {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::longest_substring("abcabcbb"), "abc")
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::longest_substring("bbbbb"), "b")
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::longest_substring("pwwkew"), "wke") 
    }
}
