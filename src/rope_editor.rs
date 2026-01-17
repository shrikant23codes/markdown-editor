
use ropey::Rope;


#[derive(Debug, Clone)]
pub struct RopeEditor {
    rope: Rope,
    version: u64,
}

impl RopeEditor {

    pub fn new(initial_text: &str) -> Self {
        Self {
            rope: Rope::from_str(initial_text),
            version: 0,
        }
    }

    pub fn get_text(&self) -> String {
        self.rope.to_string()
    }

    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn apply_edits(&mut self, new_text: &str) {
        let old_text = self.rope.to_string();
        
        // Find common prefix length

        let common_prefix = old_text
            .chars()
            .zip(new_text.chars())
            .take_while(|(a, b)| a == b)
            .count();


        // Take suffix by removing the common prefix
        let old_suffix: Vec<char> = old_text.chars().skip(common_prefix).collect();
        let new_suffix: Vec<char> = new_text.chars().skip(common_prefix).collect();

        let common_suffix = old_suffix
            .iter()
            .rev()
            .zip(new_suffix.iter().rev())
            .take_while(|(a, b)| a == b)
            .count();

        // Calculate the changed part
        let start_char = common_prefix;
        let old_end_char = old_text.len() - common_suffix;
        // println!("Old end char: {}, start char: {} and total len:: {}", old_end_char, start_char, old_text.len());

        let new_middle_text: String = new_text
            .chars()
            .skip(common_prefix)
            .take(new_text.len() - common_prefix - common_suffix)
            .collect();

        if old_end_char > start_char {
            // This means there is some text to remove. Otherwise, there won't be gap between start and end.
            self.rope.remove(start_char..old_end_char); // In rust start is inclusive, end is exclusive
        }

        if !new_middle_text.is_empty() {
            self.rope.insert(start_char, &new_middle_text);
        }

        self.version += 1;

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_editor() {
        let editor = RopeEditor::new("Hello, World!");
        assert_eq!(editor.get_text(), "Hello, World!");
        assert_eq!(editor.len_chars(), 13);
        assert_eq!(editor.version, 0);
    }

    #[test]
    fn test_empty_editor() {
        let editor = RopeEditor::new("");
        assert_eq!(editor.get_text(), "");
        assert_eq!(editor.len_chars(), 0);
        assert_eq!(editor.version, 0);
    }

    #[test]
    fn test_apply_edits() {
        let mut editor = RopeEditor::new("Namaste duniya");
        editor.apply_edits("Namaste saari duniya");
        assert_eq!(editor.get_text(), "Namaste saari duniya");
    }

    #[test]
    fn test_apply_edits_delete() {
        let mut editor = RopeEditor::new("Namaste");
        editor.apply_edits("NateT");
        assert_eq!(editor.get_text(), "NateT");
    }

    #[test]
    fn apply_edits_append_text() {
        let mut editor = RopeEditor::new("namaste");
        editor.apply_edits("namaste duniya");
        assert_eq!(editor.get_text(), "namaste duniya");

        editor.apply_edits("namastee duniya..");
        assert_eq!(editor.get_text(), "namastee duniya..");
        assert_eq!(editor.version, 2);

    }

    #[test]
    fn apply_edits_replace_all() {
        let mut editor = RopeEditor::new("Hello");
        editor.apply_edits("namaste");
        assert_eq!(editor.get_text(), "namaste");
    }

}