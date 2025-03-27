import os

def remove_comments(content):
    """
    清除C/C++代码中的注释，包括单行注释和多行注释。
    保留字符串和字符中的内容。
    """
    # 状态机：NORMAL, STRING, CHAR_LITERAL, SINGLE_LINE_COMMENT, MULTI_LINE_COMMENT
    result = ""
    state = "NORMAL"
    i = 0
    line_start = 0
    is_full_line_comment = False
    while i < len(content):
        c = content[i]
        # NORMAL状态下，处理字符串、字符和注释
        if state == "NORMAL":
            if c == '"':
                state = "STRING"
                result += c
            elif c == "'":
                state = "CHAR_LITERAL"
                result += c
            elif c == '/':
                if i + 1 < len(content) and content[i + 1] == '/':
                    is_full_line_comment = (i == line_start)
                    state = "SINGLE_LINE_COMMENT"
                    i += 1  # 跳过第二个'/'
                elif i + 1 < len(content) and content[i + 1] == '*':
                    state = "MULTI_LINE_COMMENT"
                    i += 1  # 跳过'*'
                else:
                    result += c
            elif c == '\n':
                result += c
                line_start = i + 1
            else:
                result += c
        # STRING状态下，处理字符串内容
        elif state == "STRING":
            result += c
            if c == '\\':
                if i + 1 < len(content):
                    i += 1
                    result += content[i]  # 包括转义字符
            elif c == '"':
                state = "NORMAL"
        # CHAR_LITERAL状态下，处理字符内容
        elif state == "CHAR_LITERAL":
            result += c
            if c == '\\':
                if i + 1 < len(content):
                    i += 1
                    result += content[i]
            elif c == "'":
                state = "NORMAL"
        # SINGLE_LINE_COMMENT状态下，跳过注释内容
        elif state == "SINGLE_LINE_COMMENT":
            if c == '\n':
                # 如果是不整行注释，保留换行符
                if not is_full_line_comment:
                    result += c
                line_start = i + 1
                state = "NORMAL"
            # 跳过注释内容
        elif state == "MULTI_LINE_COMMENT":
            if c == '*' and i + 1 < len(content) and content[i + 1] == '/':
                i += 1  # 跳过'/'
                state = "NORMAL"
            # 跳过注释内容
        i += 1
    return result

def process_file(file_path):
    """
    Processes a C/C++ file to remove comments, keeping the first line intact.
    """
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    if not lines:
        return
    
    # 保留第一行
    first_line = lines[0]
    rest_content = ''.join(lines[1:])
    
    # 处理剩余内容，去除注释
    processed_rest = remove_comments(rest_content)
    
    # 将第一行和处理后的内容合并
    processed_content = [first_line] + processed_rest.splitlines(True)
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.writelines(processed_content)

def main():
    """
    Traverses the current directory to find and process .c and .cpp files in src subdirectories.
    """
    current_dir = os.getcwd()
    for root, dirs, files in os.walk(current_dir):
        if 'src' in dirs:
            src_dir = os.path.join(root, 'src')
            for filename in os.listdir(src_dir):
                if filename.endswith(('.c', '.cpp')):
                    file_path = os.path.join(src_dir, filename)
                    process_file(file_path)

if __name__ == '__main__':
    main()