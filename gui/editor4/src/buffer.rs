pub struct Buffer {
    pub lines: Vec<String>,
    filepath: Option<String>,
    modified: bool,
    is_terminal: bool,
}

impl Buffer {



    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            filepath: None,
            modified: false,
            is_terminal: false,
        }
    }

    pub fn new_terminal() -> Self {
        Self {
            lines: vec![String::from(">>> ")],
            filepath: None,
            modified: false,
            is_terminal: true,
        }
    }
    pub fn from_text(text: &str, filepath: Option<String>) -> Self {
        // Normal editor buffer
    // Note: `is_terminal` is false for buffers created from text
    // Used for loading files etc.
    
        let lines: Vec<String> = if text.is_empty() {
            vec![String::new()]
        } else {
            text.lines().map(|l| l.to_string()).collect()
        };
        Self {
            lines,
            filepath,
            modified: false,
            is_terminal: false,
        }
    }

    pub fn load(filepath: &str) -> Result<Self, std::io::Error> {
        let text = std::fs::read_to_string(filepath)?;
        Ok(Self::from_text(&text, Some(filepath.to_string())))
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        let path = self.filepath.clone();
        if let Some(p) = path {
            self.save_as(&p)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "no filepath"))
        }
    }

    pub fn save_as(&mut self, path: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, self.text())?;
        self.filepath = Some(path.to_string());
        self.modified = false;
        Ok(())
    }

    pub fn text(&self) -> String {
        self.lines.join("\n")
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn line(&self, row: usize) -> &str {
        self.lines.get(row).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn line_len(&self, row: usize) -> usize {
        self.lines.get(row).map(|s| s.chars().count()).unwrap_or(0)
    }

    pub fn insert_char(&mut self, row: usize, col: usize, ch: char) {
        if row >= self.lines.len() {
            return;
        }
        let line = &mut self.lines[row];
        let pos = line.char_indices().nth(col).map(|(i, _)| i).unwrap_or(line.len());
        line.insert(pos, ch);
        self.modified = true;
    }

    pub fn delete_char(&mut self, row: usize, col: usize) {
        if row >= self.lines.len() {
            return;
        }
        let line = &mut self.lines[row];
        if let Some((pos, _)) = line.char_indices().nth(col) {
            line.remove(pos);
            self.modified = true;
        }
    }

    pub fn insert_newline(&mut self, row: usize, col: usize) {
        if row >= self.lines.len() {
            return;
        }
        let line = &mut self.lines[row];
        let byte_pos = line.char_indices().nth(col).map(|(i, _)| i).unwrap_or(line.len());
        let right: String = line.drain(byte_pos..).collect();
        self.lines.insert(row + 1, right);
        self.modified = true;
    }

    pub fn delete_newline(&mut self, row: usize) {
        if row == 0 || row >= self.lines.len() {
            return;
        }
        let moved = self.lines.remove(row);
        let prev = &mut self.lines[row - 1];
        prev.push_str(&moved);
        self.modified = true;
    }

    pub fn backspace(&mut self, row: usize, col: usize) -> Option<(usize, usize)> {
        if col > 0 {
            self.delete_char(row, col - 1);
            Some((row, col - 1))
        } else if row > 0 {
            let prev_len = self.lines.get(row - 1).map(|l| l.chars().count()).unwrap_or(0);
            self.delete_newline(row);
            Some((row - 1, prev_len))
        } else {
            None
        }
    }

    pub fn delete(&mut self, row: usize, col: usize) -> Option<(usize, usize)> {
        if col < self.line_len(row) {
            self.delete_char(row, col);
            Some((row, col))
        } else if row + 1 < self.lines.len() {
            self.delete_newline(row + 1);
            Some((row, col))
        } else {
            None
        }
    }

    pub fn insert_text(&mut self, text: &str, row: usize, col: usize) -> (usize, usize) {
        let mut r = row;
        let mut c = col;
        for ch in text.chars() {
            if ch == '\n' {
                self.insert_newline(r, c);
                r += 1;
                c = 0;
            } else {
                self.insert_char(r, c, ch);
                c += 1;
            }
        }
        (r, c)
    }

    pub fn filepath(&self) -> Option<&str> {
        self.filepath.as_deref()
    }

    pub fn filename(&self) -> &str {
        self.filepath
            .as_deref()
            .and_then(|p| p.rsplit('/').next())
            .unwrap_or("untitled")
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn basic_operations() {
        let mut buf = Buffer::new();
        assert_eq!(buf.text(), "");
        // Insert characters
        buf.insert_char(0, 0, 'a');
        buf.insert_char(0, 1, 'b');
        buf.insert_char(0, 2, 'c');
        assert_eq!(buf.text(), "abc");
        // Insert newline
        buf.insert_newline(0, 3);
        assert_eq!(buf.line_count(), 2);
        assert_eq!(buf.line(0), "abc");
        assert_eq!(buf.line(1), "");
        // Insert more text after newline
        buf.insert_text("def", 1, 0);
        assert_eq!(buf.line(1), "def");
        // Backspace at start of line 1 merges lines
        let pos = buf.backspace(1, 0).expect("should merge lines");
        assert_eq!(pos, (0, 3));
        assert_eq!(buf.text(), "abcdef");
        // Delete a character
        let del_pos = buf.delete(0, 2).expect("should delete");
        assert_eq!(del_pos, (0, 2));
        assert_eq!(buf.text(), "abdef");
    }

    #[test]
    fn save_and_load() {
        let mut buf = Buffer::from_text("Hello\nWorld", Some("temp.txt".to_string()));
        let tmp_path = std::env::temp_dir().join("editor4_test.txt");
        // Ensure clean state
        let _ = fs::remove_file(&tmp_path);
        buf.save_as(tmp_path.to_str().unwrap()).unwrap();
        let loaded = Buffer::load(tmp_path.to_str().unwrap()).unwrap();
        assert_eq!(loaded.text(), "Hello\nWorld");
        // Clean up
        let _ = fs::remove_file(&tmp_path);
    }
}
