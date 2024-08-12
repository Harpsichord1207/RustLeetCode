use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};

pub struct Solution;

struct MyLayer {
    paths: Vec<MyPath>,
    last_word_map: HashMap<String, Vec<usize>>
}

impl MyLayer {
    fn new(queue: &VecDeque<MyPath>) -> Self {
        let mut paths = vec![];
        let mut map = HashMap::new();
        for (i, path) in queue.iter().enumerate() {
            paths.push(path.clone());
            map.entry(path.last_word().unwrap().to_string()).or_insert_with(Vec::new).push(i);
        }
        Self {
            paths,
            last_word_map: map
        }
    }
}

struct MyPath {
    path: Vec<String>,
}

impl MyPath {

    fn new(vec_path: Vec<&str>) -> Self {
        Self {
            path: vec_path.iter().map(|s| s.to_string() ).collect()
        }
    }

    fn len(&self) -> usize {
        self.path.len()
    }

    fn last_word(&self) -> Option<&str> {
        self.path.last().map(|s| s.as_str() )
    }

    fn clone(&self) -> Self {
        Self {
            path: self.path.clone()
        }
    }

    fn push(&mut self, word: &str) {
        self.path.push(word.to_string())
    }

    fn to_vec(&self) -> Vec<String> {
        self.path.clone()
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let length = self.path.len();
        let path = self.path.join(" -> ");
        write!(f, "{length}: {path}")
    }

    fn extend(&mut self, other: &Self) {
        let reversed_other_path: Vec<_> = other.path.iter().map(|w|w.clone()).rev().collect();
        self.path.extend_from_slice(&reversed_other_path[1..]);
    }

}

impl Display for MyPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for MyPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl PartialEq for MyPath {
    fn eq(&self, other: &Self) -> bool {
        let len = self.len();
        if other.len() != len {
            return false;
        }
        for i in 0..len {
            if self.path.get(i).unwrap() != other.path.get(i).unwrap() {
                return false;
            }
        }
        true
    }
}

impl Solution {

    fn trans_word(word: &str, word_set: &HashSet<&String>, cache: &mut HashMap<String, HashSet<String>>) -> HashSet<String> {

        if let Some(words) = cache.get(word) {
            return words.clone();
        }

        let mut words = HashSet::new();
        let mut chars: Vec<char> = word.chars().collect();

        for i in 0..chars.len() {
            let original_char = chars[i];
            for ch in "qwertyuiopasdfghjklzxcvbnm".chars() {
                if original_char == ch {
                    continue;
                }
                chars[i] = ch;
                let new_word: String = chars.iter().collect();

                if word_set.contains(&new_word) {
                    words.insert(new_word);
                }
            }
            chars[i] = original_char;

        }
        cache.insert(word.to_string(), words.clone());
        words
    }

    fn _dig_one_layer(word_set: &HashSet<&String>, queue: &mut VecDeque<MyPath>, visited: &HashSet<String>, sub_visited: &mut HashSet<String>, trans_word_cache: &mut HashMap<String, HashSet<String>>) {
        let queue_size = queue.len();
        for _ in 0..queue_size {
            let last_path = queue.pop_front().unwrap();
            let last_word = last_path.last_word().unwrap();
            let words = Self::trans_word(last_word, word_set, trans_word_cache);
            for neighbor in words.difference(visited) {
                let mut last_path_copy = last_path.clone();
                last_path_copy.push(neighbor);
                sub_visited.insert(neighbor.clone());
                queue.push_back(last_path_copy);
            }
        }
    }

    fn _check_layers(begin_layer: MyLayer, end_layer: MyLayer, result: &mut Vec<MyPath>){

        fn _combine_path(path1: &MyPath, path2: &MyPath, result: &mut Vec<MyPath>) {
            let mut path = path1.clone();
            path.extend(path2);
            if !result.contains(&path) {
                result.push(path)
            }
        }

        let bks: HashSet<_> = begin_layer.last_word_map.keys().collect();
        let eks: HashSet<_> = end_layer.last_word_map.keys().collect();
        for &word in bks.intersection(&eks) {
            for &bi in begin_layer.last_word_map.get(word).unwrap() {
                for &ei in end_layer.last_word_map.get(word).unwrap() {
                    _combine_path(
                        begin_layer.paths.get(bi).unwrap(),
                        end_layer.paths.get(ei).unwrap(),
                        result,
                    )
                }
            }
        }
    }

    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        let mut result: Vec<MyPath> = Vec::new();
        let word_set: HashSet<&String> = word_list.iter().collect();
        if !word_set.contains(&end_word) {
            return vec![];
        }

        let mut begin_queue = VecDeque::new();
        let mut end_queue = VecDeque::new();

        begin_queue.push_back(MyPath::new(vec![&begin_word]));
        end_queue.push_back(MyPath::new(vec![&end_word]));

        let mut b_visited: HashSet<String> = HashSet::new();
        b_visited.insert(begin_word.clone());
        let mut e_visited: HashSet<String> = HashSet::new();
        e_visited.insert(end_word.clone());

        let mut trans_word_cache = HashMap::new();

        loop {

            let mut b_sub_visited = HashSet::new();
            let mut e_sub_visited = HashSet::new();

            Self::_dig_one_layer(&word_set, &mut begin_queue, &b_visited, &mut b_sub_visited, &mut trans_word_cache);
            if begin_queue.is_empty() {
                break;
            }

            println!("B: {}", begin_queue.len());
            println!("L: {}", begin_queue.get(0).unwrap().path.len());

            Self::_check_layers(
                MyLayer::new(&begin_queue),
                MyLayer::new(&end_queue),
                &mut result,
            );

            if !result.is_empty() {
                break;
            }

            Self::_dig_one_layer(&word_set, &mut end_queue, &e_visited, &mut e_sub_visited, &mut trans_word_cache);
            if end_queue.is_empty() {
                break;
            }

            println!("E: {}", end_queue.len());
            println!("L: {}", end_queue.get(0).unwrap().path.len());

            Self::_check_layers(
                MyLayer::new(&begin_queue),
                MyLayer::new(&end_queue),
                &mut result,
            );
            if !result.is_empty() {
                break;
            }
            b_visited.extend(b_sub_visited);
            e_visited.extend(e_sub_visited);
        }
        result.iter().map(|r| r.to_vec() ).collect()
    }

    pub fn find_ladders_with_bfs(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        let word_set: HashSet<&String> = word_list.iter().collect();
        if !word_set.contains(&end_word) {
            return vec![];
        }
        let mut result = vec![];
        Self::bfs(&begin_word, &end_word, &word_set, &mut result);
        result
    }

    fn bfs(begin_word: &str, end_word: &str, word_set: &HashSet<&String>, result: &mut Vec<Vec<String>>) {
        let mut queue = VecDeque::new();
        queue.push_back(vec![begin_word.to_string()]);
        let mut is_found = false;
        let mut visited = HashSet::new();
        visited.insert(begin_word.to_string());

        while !queue.is_empty() {
            let queue_size = queue.len();
            let mut sub_visited = HashSet::new();
            for _ in 0..queue_size {
                let last_path = queue.pop_front().unwrap();
                println!("Trying Path: {last_path:?}");
                println!("Visited: {}, Size: {}", visited.len(), queue_size);
                let last_word = last_path.get(last_path.len()-1).unwrap();
                for neighbor in Self::trans_word(last_word, word_set, &mut HashMap::new()) {
                    if visited.contains(&neighbor) {
                        continue;
                    }
                    sub_visited.insert(neighbor.clone());
                    let mut last_path_copy = last_path.clone();
                    last_path_copy.push(neighbor.clone());
                    queue.push_back(last_path_copy.clone());
                    if neighbor == end_word {
                        is_found = true;
                        result.push(last_path_copy.clone());
                    }
                }
            }
            visited.extend(sub_visited);
            if is_found {
                break;
            }
        }
    }

}