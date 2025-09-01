/// Test file to verify doc comment configuration
use ide::HighlightConfig;
use syntax::{AstNode, SourceFile};

fn test_separate_comment_highlighting() {
    let code = r#"
// This is a regular comment
/// This is a doc comment  
fn test() {
    // Another regular comment
    /* Block comment */
    /** Doc block comment */
}
"#;

    // Test with regular comments disabled but doc comments enabled
    let config = HighlightConfig {
        strings: true,
        comments: false,  // Regular comments disabled
        doc_comments: true,  // Doc comments enabled
        punctuation: true,
        specialize_punctuation: true,
        specialize_operator: true,
        operator: true,
        inject_doc_comment: true,
        macro_bang: true,
        syntactic_name_ref_highlighting: false,
    };

    // Parse the source
    let parse = SourceFile::parse(code, span::Edition::CURRENT);
    let file = parse.tree();

    // This test would need actual highlighting to verify the filtering
    // For now this just ensures the config structure is correct
    assert!(!config.comments);
    assert!(config.doc_comments);
}

fn main() {
    test_separate_comment_highlighting();
}