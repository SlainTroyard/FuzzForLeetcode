#!/bin/bash

# 设置输出文本颜色
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}检查C和C++输出目录中哪些缺少examples文件...${NC}\n"

# 存储结果的数组
missing_c=()
missing_cpp=()
has_examples_c=()
has_examples_cpp=()

# 检查C输出目录
echo -e "${YELLOW}C语言输出目录检查:${NC}"
for dir in fuzz_outputs/C/weekly_contest_*; do
    if [ -d "$dir" ]; then
        if [ ! -f "$dir/examples" ] && [ ! -d "$dir/examples" ]; then
            echo -e "${RED}✕ $dir 缺少examples${NC}"
            missing_c+=("$dir")
        else
            echo -e "${GREEN}✓ $dir 有examples${NC}"
            has_examples_c+=("$dir")
        fi
    fi
done

echo -e "\n${YELLOW}C++输出目录检查:${NC}"
# 检查CPP输出目录
for dir in fuzz_outputs/CPP/weekly_contest_*; do
    if [ -d "$dir" ]; then
        if [ ! -f "$dir/examples" ] && [ ! -d "$dir/examples" ]; then
            echo -e "${RED}✕ $dir 缺少examples${NC}"
            missing_cpp+=("$dir")
        else
            echo -e "${GREEN}✓ $dir 有examples${NC}"
            has_examples_cpp+=("$dir")
        fi
    fi
done

# 输出统计数据
echo -e "\n${YELLOW}统计摘要:${NC}"
echo -e "C语言: ${GREEN}${#has_examples_c[@]} 个目录有examples${NC}, ${RED}${#missing_c[@]} 个目录缺少examples${NC}"
echo -e "C++: ${GREEN}${#has_examples_cpp[@]} 个目录有examples${NC}, ${RED}${#missing_cpp[@]} 个目录缺少examples${NC}"

# 直接保存结果到文件
echo "# 缺少examples的C目录" > missing_examples.txt
for dir in "${missing_c[@]}"; do
    echo "$dir" >> missing_examples.txt
done

echo -e "\n# 缺少examples的C++目录" >> missing_examples.txt
for dir in "${missing_cpp[@]}"; do
    echo "$dir" >> missing_examples.txt
done

echo -e "\n${GREEN}结果已保存到 missing_examples.txt${NC}"

exit 0 