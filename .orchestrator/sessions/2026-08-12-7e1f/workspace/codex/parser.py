"""A small, safe parser for a useful subset of Markdown.

Supported block elements are ATX headings and ordered/unordered lists.  Text
outside those blocks is emitted as paragraphs.  Within text, ``**bold**`` (or
``__bold__``) and ``*italic*`` (or ``_italic_``) are supported.
"""

from __future__ import annotations

import html
import re

__all__ = ["parse_markdown"]


_HEADING = re.compile(r"^(#{1,6})[ \t]+(.+?)[ \t]*#*[ \t]*$")
_UNORDERED_ITEM = re.compile(r"^[ \t]*[-+*][ \t]+(.+)$")
_ORDERED_ITEM = re.compile(r"^[ \t]*\d+[.)][ \t]+(.+)$")


def _parse_inline(text: str) -> str:
    """Escape *text* and render the supported inline Markdown markers."""
    result = html.escape(text, quote=False)

    # Render strong first so asterisks/underscores inside it may subsequently
    # participate in italic markup, as they do in common Markdown usage.
    result = re.sub(r"(?<!\\)\*\*(.+?)(?<!\\)\*\*", r"<strong>\1</strong>", result)
    result = re.sub(r"(?<!\\)__(.+?)(?<!\\)__", r"<strong>\1</strong>", result)
    result = re.sub(r"(?<!\\)\*(?!\*)(.+?)(?<!\\)\*", r"<em>\1</em>", result)
    result = re.sub(r"(?<!\\)_(?!_)(.+?)(?<!\\)_", r"<em>\1</em>", result)

    # A backslash is only meaningful here as an escape for a Markdown marker.
    return re.sub(r"\\([*_])", r"\1", result)


def parse_markdown(markdown: str) -> str:
    """Convert a basic Markdown document into an HTML fragment.

    Args:
        markdown: The Markdown source.  It must be a string.

    Returns:
        An HTML fragment.  Consecutive ordinary lines become one paragraph;
        blank lines separate paragraphs and lists.

    Raises:
        TypeError: If ``markdown`` is not a string.
    """
    if not isinstance(markdown, str):
        raise TypeError("markdown must be a string")

    output: list[str] = []
    paragraph: list[str] = []
    list_kind: str | None = None

    def close_paragraph() -> None:
        if paragraph:
            output.append(f"<p>{_parse_inline(' '.join(paragraph))}</p>")
            paragraph.clear()

    def close_list() -> None:
        nonlocal list_kind
        if list_kind is not None:
            output.append(f"</{list_kind}>")
            list_kind = None

    for raw_line in markdown.splitlines():
        line = raw_line.rstrip()
        if not line.strip():
            close_paragraph()
            close_list()
            continue

        heading = _HEADING.match(line)
        if heading:
            close_paragraph()
            close_list()
            level = len(heading.group(1))
            output.append(f"<h{level}>{_parse_inline(heading.group(2))}</h{level}>")
            continue

        unordered = _UNORDERED_ITEM.match(line)
        ordered = _ORDERED_ITEM.match(line)
        if unordered or ordered:
            close_paragraph()
            kind = "ul" if unordered else "ol"
            item = (unordered or ordered).group(1)
            if list_kind != kind:
                close_list()
                output.append(f"<{kind}>")
                list_kind = kind
            output.append(f"<li>{_parse_inline(item)}</li>")
            continue

        close_list()
        paragraph.append(line.strip())

    close_paragraph()
    close_list()
    return "\n".join(output)
