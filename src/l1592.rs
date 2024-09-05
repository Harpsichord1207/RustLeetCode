pub struct Solution;


impl Solution {
    #[allow(dead_code)]
    pub fn reorder_spaces(text: String) -> String {
        let space = text.chars().filter(|&c| c == ' ').count();
        let words = text.split_ascii_whitespace().into_iter().map(
            |w| w.to_string()
        ).collect::<Vec<String>>();

        if words.len() == 1 {
            return format!("{}{}", words[0], " ".repeat(space));
        }

        let sep = space / (words.len() - 1);
        let remain = space % (words.len() - 1);

        format!("{}{}", words.join(&" ".repeat(sep)), " ".repeat(remain))

    }
}
