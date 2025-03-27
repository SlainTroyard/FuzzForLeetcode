import os
import re

def process_file(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    if not lines:
        return
    
    # 保留首行
    first_line = lines[0]
    rest_content = lines[1:]

    # 合并剩余内容用于处理多行注释
    content = ''.join(rest_content)
    
    # 删除所有多行注释（包括跨行注释）
    content = re.sub(r'/\*.*?\*/', '', content, flags=re.DOTALL)
    
    # 按行分割处理单行注释
    processed_lines = []
    for line in content.splitlines(True):  # 保留换行符
        # 删除整行注释（包含空白前缀）
        if re.match(r'^\s*//', line):
            continue
        # 删除行内注释
        line = re.sub(r'//.*', '', line)
        processed_lines.append(line)

    # 合并首行和处理的剩余内容
    final_content = [first_line] + processed_lines
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.writelines(final_content)

def main():
    current_dir = os.getcwd()
    for root, dirs, _ in os.walk(current_dir):
        if 'src' in dirs:
            src_dir = os.path.join(root, 'src')
            for filename in os.listdir(src_dir):
                if filename.endswith(('.c', '.cpp')):
                    file_path = os.path.join(src_dir, filename)
                    process_file(file_path)

if __name__ == '__main__':
    main()