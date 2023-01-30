#[cfg(test)]
use std::collections::{HashSet, LinkedList};

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    pub fn longest_substring(s: String) -> i32 {
        let mut max: usize = 0;
        let mut list: LinkedList<char> = LinkedList::new();
        let mut set: HashSet<char> = HashSet::new();

        for c in s.chars() {
            if set.contains(&c) {
                while let Some(front) = list.pop_front() {
                    if front == c {
                        list.push_back(c);
                        max = std::cmp::max(max, list.len());
                        break;
                    } else {
                        set.remove(&front);
                    }
                }
            } else {
                set.insert(c);
                list.push_back(c);
                max = std::cmp::max(max, list.len());
            }
        }

        return max as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::longest_substring(String::from("abcabcbb")), 3)
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::longest_substring(String::from("bbbbb")), 1)
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::longest_substring(String::from("pwwkew")), 3)
    }
}
