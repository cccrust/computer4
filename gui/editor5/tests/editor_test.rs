#[cfg(test)]
mod tests {
    #[test]
    fn test_html_content_exists() {
        // Test that HTML file exists and contains expected content
        let html = include_str!("../assets/index.html");
        assert!(html.contains("monaco.editor.create"));
        assert!(html.contains("vs-dark"));
        assert!(html.contains("rust"));
    }

    #[test]
    fn test_monaco_config() {
        let html = include_str!("../assets/index.html");
        // Check that Monaco is configured with Rust language
        assert!(html.contains("language: 'rust'"));
        // Check that automatic layout is enabled
        assert!(html.contains("automaticLayout: true"));
    }

    #[test]
    fn test_message_passing_setup() {
        let html = include_str!("../assets/index.html");
        // Check that postMessage is set up for JS -> Rust communication
        assert!(html.contains("window.parent.postMessage"));
        // Check that message listener is set up for Rust -> JS communication
        assert!(html.contains("addEventListener('message'"));
    }

    #[test]
    fn test_html_structure() {
        let html = include_str!("../assets/index.html");
        // Check essential HTML structure
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<html"));
        assert!(html.contains("<div id=\"editor\">"));
        assert!(html.contains("require.config"));
        assert!(html.contains("vs/editor/editor.main"));
    }

    #[test]
    fn test_no_syntax_errors_in_html() {
        let html = include_str!("../assets/index.html");
        // Basic sanity check that HTML looks well-formed
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("</html>"));
        assert!(html.contains("<script>"));
        assert!(html.contains("</script>"));
    }
}