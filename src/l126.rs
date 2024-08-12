use std::collections::{VecDeque, HashMap, hash_map::Entry};

pub struct Solution;

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {


        let mut queue = VecDeque::from([(begin_word.as_str(), 0)]);
        let mut depth_found = None;

        let mut word_states = HashMap::<&str, (usize, Vec<&str>)>::new();

        while let Some((w, d)) = queue.pop_front() {
            if depth_found.is_some_and(|found| found < d) {
                break;
            }

            if w == end_word {
                depth_found = Some(d);
            }

            let neighbours = word_list.iter().filter(
                |word| {
                    w.bytes().zip(word.bytes()).filter(|(a, b)| a != b).count() == 1
                }
            );

            for n in neighbours {
                match word_states.entry(n) {
                    Entry::Vacant(v) => {
                        v.insert((d + 1, vec![w]));
                        queue.push_back((n, d + 1));
                    },
                    Entry::Occupied(mut o) => {
                        let o = o.get_mut();
                        if d + 1 == o.0 {
                            o.1.push(w);
                        }
                    }
                }
            }
        }

        if !word_states.contains_key(end_word.as_str()) {
            return Vec::new();
        }

        let mut result = Vec::new();
        fn walk_back(word: &str, start_word: &str, states: &HashMap<&str, (usize, Vec<&str>)>, paths: &mut Vec<Vec<String>>, current: &mut Vec<String>) {
            if word == start_word {
                let mut p = current.clone();
                p.reverse();
                paths.push(p);
                return;
            }

            let state = states.get(word).unwrap();
            for prev in &state.1 {
                current.push(prev.to_string());
                walk_back(prev, start_word, states, paths, current);
                current.pop();
            }
        }
        walk_back(end_word.as_str(), begin_word.as_str(), &word_states, &mut result, &mut vec![end_word.to_string()]);
        result
    }
}