import os
import sys
import pytest

# Add the codex workspace directory to sys.path to import parser.py
codex_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), 'codex')
if os.path.isdir(codex_dir):
    sys.path.insert(0, codex_dir)

from parser import parse_markdown

def test_headers():
    assert parse_markdown("# Header 1") == "<h1>Header 1</h1>"
    assert parse_markdown("## Header 2") == "<h2>Header 2</h2>"
    assert parse_markdown("### Header 3") == "<h3>Header 3</h3>"
    assert parse_markdown("###### Header 6") == "<h6>Header 6</h6>"
    # Header with trailing hashes
    assert parse_markdown("## Header 2 ##") == "<h2>Header 2</h2>"
    assert parse_markdown("## Header 2  ###  ") == "<h2>Header 2</h2>"

def test_paragraphs():
    assert parse_markdown("This is a paragraph.") == "<p>This is a paragraph.</p>"
    assert parse_markdown("Line 1\nLine 2") == "<p>Line 1 Line 2</p>"
    assert parse_markdown("Para 1\n\nPara 2") == "<p>Para 1</p>\n<p>Para 2</p>"

def test_bold_italics():
    assert parse_markdown("**bold**") == "<p><strong>bold</strong></p>"
    assert parse_markdown("__bold__") == "<p><strong>bold</strong></p>"
    assert parse_markdown("*italic*") == "<p><em>italic</em></p>"
    assert parse_markdown("_italic_") == "<p><em>italic</em></p>"
    # Escaped
    assert parse_markdown("\\*not italic\\*") == "<p>*not italic*</p>"
    assert parse_markdown("\\*\\*not bold\\*\\*") == "<p>**not bold**</p>"

def test_lists():
    # Unordered
    assert parse_markdown("- Item 1\n- Item 2") == "<ul>\n<li>Item 1</li>\n<li>Item 2</li>\n</ul>"
    assert parse_markdown("* Item 1\n+ Item 2") == "<ul>\n<li>Item 1</li>\n<li>Item 2</li>\n</ul>"
    # Ordered
    assert parse_markdown("1. Item 1\n2) Item 2") == "<ol>\n<li>Item 1</li>\n<li>Item 2</li>\n</ol>"
    
def test_list_and_paragraphs():
    md = "Para 1\n\n- Item 1\n\nPara 2"
    expected = "<p>Para 1</p>\n<ul>\n<li>Item 1</li>\n</ul>\n<p>Para 2</p>"
    assert parse_markdown(md) == expected

def test_edge_cases():
    # Empty string
    assert parse_markdown("") == ""
    # Only blank lines
    assert parse_markdown("\n\n\n") == ""
    # Type error
    with pytest.raises(TypeError):
        parse_markdown(123)
