# 对指定文件夹中所有.c或者.cpp文件中所有的"// TODO: Add the base I/O interface here"这段内容进行删除,不删除文件
import os
import re

def delete_todo(directory):
    count = 0
    modified_files = []  # 添加列表来存储修改的文件名
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.c') or file.endswith('.cpp'):
                file_path = os.path.join(root, file)
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                if '// TODO: Add the base I/O interface here' in content:
                    # 删除文件中的这段内容
                    content = content.replace('// TODO: Add the base I/O interface here', '')
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(content)
                    # 打印调试信息
                    print(f'已删除文件{file_path}中的"// TODO: Add the base I/O interface here"这段内容')
                    # 统计数量
                    count += 1
                    modified_files.append(file)  # 记录修改的文件名
    print(f'已删除{count}个文件中的"// TODO: Add the base I/O interface here"这段内容')
    return count, modified_files

if __name__ == '__main__':
    count1, files1 = delete_todo('/home/xiaofan/dev_25/transproj/FuzzForLeetcode/C_CPP/C/src')    
    count2, files2 = delete_todo('/home/xiaofan/dev_25/transproj/FuzzForLeetcode/C_CPP/CPP/src')
    total_count = count1 + count2
    
    # 保存结果到文件
    with open('delete_todo.txt', 'w', encoding='utf-8') as f:
        f.write(f'已删除{total_count}个文件中的"// TODO: Add the base I/O interface here"这段内容\n\n')
        f.write('修改的文件列表：\n')
        for file in files1 + files2:
            f.write(file + '\n')
