import os
import re

def count_unsafe_content(file_path):
    """
    Count unsafe blocks/functions in a .rs file with detailed character statistics.
    Also excludes comments and string literals from the analysis.
    """
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Remove comments and string literals
    # Remove line comments
    content_no_line_comments = re.sub(r'//.*?$', '', content, flags=re.MULTILINE)
    # Remove block comments
    content_no_comments = re.sub(r'/\*[\s\S]*?\*/', '', content_no_line_comments)
    # Remove string literals (both "" and '')
    content_clean = re.sub(r'(".*?"|\'.*?\')', '', content_no_comments, flags=re.DOTALL)
    
    # Split into lines for line counting
    lines = content_clean.splitlines()
    non_empty_lines = [line for line in lines if line.strip()]
    
    total_chars = sum(len(line) for line in non_empty_lines)
    
    # Process the content to find unsafe blocks and functions
    unsafe_lines = 0
    unsafe_chars = 0
    
    in_unsafe_block = False
    in_unsafe_function = False
    waiting_for_brace_function = False
    waiting_for_brace_block = False
    brace_count = 0
    
    for line in non_empty_lines:
        stripped = line.strip()
        
        # Count braces in this line
        open_braces = stripped.count('{')
        close_braces = stripped.count('}')
        
        # Handle waiting for function body's opening brace
        if waiting_for_brace_function:
            if '{' in stripped:
                in_unsafe_function = True
                waiting_for_brace_function = False
                brace_count += open_braces - close_braces
            continue
        
        # Handle waiting for block's opening brace
        elif waiting_for_brace_block:
            if '{' in stripped:
                in_unsafe_block = True
                waiting_for_brace_block = False
                brace_count += open_braces - close_braces
            continue
        
        # Update brace count for tracking scope
        brace_count += open_braces - close_braces
        
        if not in_unsafe_block and not in_unsafe_function:
            # Detect unsafe function declarations (excluding type aliases)
            if 'type' not in stripped and re.search(r'\bunsafe\s+.*\bfn\b', stripped):
                waiting_for_brace_function = True
            # Detect unsafe block declarations
            elif re.search(r'\bunsafe\s*{', stripped):
                in_unsafe_block = True
            elif re.search(r'\bunsafe\s*$', stripped):
                waiting_for_brace_block = True
        
        # Track if we're in an unsafe context
        if in_unsafe_block or in_unsafe_function:
            unsafe_lines += 1
            unsafe_chars += len(stripped)
            
            # Check if we're exiting the unsafe block/function
            if brace_count <= 0:
                in_unsafe_block = False
                in_unsafe_function = False
                brace_count = 0
    
    return {
        'total_lines': len(non_empty_lines),
        'unsafe_lines': unsafe_lines,
        'total_chars': total_chars,
        'unsafe_chars': unsafe_chars
    }

def count_one_folder(folder_path):
    """Count unsafe content in all .rs files in the specified folder with paired .c files."""
    file_pairs = {}
    # Find all .c and .rs files
    for root, _, files in os.walk(folder_path):
        for file in files:
            name, ext = os.path.splitext(file)
            if ext in ['.c', '.rs']:
                file_path = os.path.join(root, file)
                if name not in file_pairs:
                    file_pairs[name] = {}
                file_pairs[name][ext] = file_path

    folder_stats = {
        'total_lines': 0,
        'unsafe_lines': 0,
        'total_chars': 0,
        'unsafe_chars': 0,
        'file_count': 0
    }
    
    results = []

    # Process each file pair
    for name, paths in file_pairs.items():
        if '.c' in paths and '.rs' in paths:
            rs_file = paths['.rs']
            stats = count_unsafe_content(rs_file)
            
            folder_stats['total_lines'] += stats['total_lines']
            folder_stats['unsafe_lines'] += stats['unsafe_lines']
            folder_stats['total_chars'] += stats['total_chars']
            folder_stats['unsafe_chars'] += stats['unsafe_chars']
            folder_stats['file_count'] += 1
            
            if stats['total_lines'] > 0:
                line_rate = stats['unsafe_lines'] / stats['total_lines']
                char_rate = stats['unsafe_chars'] / stats['total_chars'] if stats['total_chars'] > 0 else 0
                
                file_result = {
                    'name': name,
                    'stats': stats,
                    'line_rate': line_rate,
                    'char_rate': char_rate
                }
                results.append(file_result)
    
    return folder_stats, results

def format_results(folder_path, folder_stats, file_results):
    """Format the results into a readable string."""
    output = [f"\n\n\n{'=' * 60}", f"Results for folder: {folder_path}", f"{'=' * 60}\n"]
    
    # Add folder summary
    if folder_stats['total_lines'] > 0:
        line_rate = folder_stats['unsafe_lines'] / folder_stats['total_lines']
        char_rate = folder_stats['unsafe_chars'] / folder_stats['total_chars'] if folder_stats['total_chars'] > 0 else 0
        
        output.append(f"FOLDER SUMMARY ({folder_stats['file_count']} file pairs):")
        output.append(f"  Total lines: {folder_stats['total_lines']}")
        output.append(f"  Unsafe lines: {folder_stats['unsafe_lines']} ({line_rate:.2%})")
        output.append(f"  Total characters: {folder_stats['total_chars']}")
        output.append(f"  Unsafe characters: {folder_stats['unsafe_chars']} ({char_rate:.2%})\n")
    else:
        output.append("No valid Rust files found in this folder.\n")
        return "\n".join(output)
    
    # Add individual file results
    output.append("FILE DETAILS:")
    for result in file_results:
        name = result['name']
        stats = result['stats']
        output.append(f"  File pair: {name}.c and {name}.rs")
        output.append(f"    Total lines: {stats['total_lines']}")
        output.append(f"    Unsafe lines: {stats['unsafe_lines']} ({result['line_rate']:.2%})")
        output.append(f"    Total characters: {stats['total_chars']}")
        output.append(f"    Unsafe characters: {stats['unsafe_chars']} ({result['char_rate']:.2%})")
    
    return "\n".join(output)

def main():
    """Main function to process folders and generate the report."""
    output_file = 'unsafeCountResults.txt'
    
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("UNSAFE CODE ANALYSIS REPORT\n")
        f.write(f"Generated on: {os.path.basename(os.getcwd())}\n")
        f.write("-" * 60)
        
        # Process each first-level directory
        current_dir = os.getcwd()
        processed_folders = 0
        
        for item in os.listdir(current_dir):
            folder_path = os.path.join(current_dir, item)
            if os.path.isdir(folder_path):
                folder_stats, file_results = count_one_folder(folder_path)
                
                if folder_stats['file_count'] > 0:
                    formatted_results = format_results(folder_path, folder_stats, file_results)
                    f.write(formatted_results)
                    processed_folders += 1
        
        # Add summary footer
        f.write(f"\n\n{'=' * 60}")
        f.write(f"\nAnalysis complete. Processed {processed_folders} folders.")
        f.write(f"\n{'=' * 60}")
    
    print(f"Analysis complete! Results written to {output_file}")
    print(f"Processed {processed_folders} folders with Rust/C file pairs.")

if __name__ == '__main__':
    main()
