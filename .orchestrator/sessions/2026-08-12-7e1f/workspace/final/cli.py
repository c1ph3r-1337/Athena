import argparse
import os
import sys

try:
    from parser import parse_markdown
except ImportError:
    print("Error: Could not import parser.py. Ensure it is in the same directory.", file=sys.stderr)
    sys.exit(1)

def main():
    arg_parser = argparse.ArgumentParser(description="Convert Markdown to HTML.")
    arg_parser.add_argument("input_file", help="Path to the input .md file")
    arg_parser.add_argument("-o", "--output", dest="output_file", help="Optional path to the output .html file")
    
    args = arg_parser.parse_args()
    
    if not os.path.exists(args.input_file):
        print(f"Error: Input file '{args.input_file}' does not exist.", file=sys.stderr)
        sys.exit(1)
        
    try:
        with open(args.input_file, 'r', encoding='utf-8') as f:
            markdown_content = f.read()
    except Exception as e:
        print(f"Error reading input file: {e}", file=sys.stderr)
        sys.exit(1)
        
    try:
        html_content = parse_markdown(markdown_content)
    except Exception as e:
        print(f"Error parsing markdown: {e}", file=sys.stderr)
        sys.exit(1)
        
    if args.output_file:
        try:
            with open(args.output_file, 'w', encoding='utf-8') as f:
                f.write(html_content)
                if html_content and not html_content.endswith('\n'):
                    f.write('\n')
        except Exception as e:
            print(f"Error writing to output file: {e}", file=sys.stderr)
            sys.exit(1)
    else:
        print(html_content)

if __name__ == "__main__":
    main()
