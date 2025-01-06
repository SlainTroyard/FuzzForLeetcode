import os
import argparse
import json
from typing import Dict, Any, List, Optional
import sys

class TemplateGenerator:
    def __init__(self, contest_num: int, problem_num: int):
        self.contest_num = contest_num
        self.problem_num = problem_num
        self.base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        self.problem_id = f"weekly_contest_{contest_num}_p{problem_num}"

    def create_directory_structure(self):
        """Create project directory structure"""
        dirs = [
            f"C_CPP/C/src",
            f"C_CPP/C/constraints",
            f"C_CPP/CPP/src",
            f"C_CPP/CPP/constraints",
            f"fuzz_outputs/C/{self.problem_id}",
            f"fuzz_outputs/CPP/{self.problem_id}",
            # f"fuzz_outputs/C/{self.problem_id}/corpus",
            # f"fuzz_outputs/C/{self.problem_id}/output",
            # f"fuzz_outputs/CPP/{self.problem_id}/corpus",
            # f"fuzz_outputs/CPP/{self.problem_id}/output"
        ]
        for dir_path in dirs:
            os.makedirs(os.path.join(self.base_dir, dir_path), exist_ok=True)

    def generate_c_source(self) -> str:
        """Generate C source file template"""
        return f"""// Problem: Weekly Contest {self.contest_num} Problem {self.problem_num}
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

// TODO: Add your function declaration here
// Example:
// bool solutionFunction(type1 param1, type2 param2) {{
//     // Implementation
// }}

int main() {{
    // TODO: Add the base I/O interface here
}}
"""

    def generate_cpp_source(self) -> str:
        """Generate C++ source file template"""
        return f"""// Problem: Weekly Contest {self.contest_num} Problem {self.problem_num}
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {{
public:
    // TODO: Add your function declaration here
    // Example:
    // ReturnType solutionFunction(ParamType1 param1, ParamType2 param2) {{
    //     // Implementation
    // }}
}};
int main() {{
    // TODO: Add the base I/O interface here
}}
"""

    def generate_constraint_template(self, language: str) -> str:
        """Generate constraint file template"""
        if language == "C":
            template_fuzz = f"""import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/C/{self.problem_id}/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "{self.problem_id}.c"  # C source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())
    pass

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    pass

# Compile the C program
def compile_c():
    try:
        compile_command = ["gcc", os.path.join(c_folder, c_file), "-o", os.path.join(c_folder, executable_name)] # sometimes need to add -lm for math library
        subprocess.run(compile_command, check=True)
        print("Compilation successful.")
    except subprocess.CalledProcessError as e:
        print(f"Error during compilation: {{e}}")
        raise

# Simulate output for a test case by running the C program
def simulate_output(test_input):
    # Format input for the C program
    formatted_input = format_test_input(test_input)
    try:
        # Run the C program with the generated input
        run_command = [os.path.join(c_folder, executable_name)]
        process = subprocess.run(run_command, input=formatted_input, text=True, capture_output=True, check=True)
        return process.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error during execution: {{e}}")
        return "Error"
    
# Clean up the compiled executable
def cleanup():
    executable_path = os.path.join(c_folder, executable_name)
    if os.path.exists(executable_path):
        os.remove(executable_path)
        print("Cleaned up compiled executable.")

# Main program
if __name__ == "__main__":
    try:
        compile_c()
        with open(output_file, "w") as f:
            for case_id in range(1, test_cases + 1):
                test_input = generate_test_input()
                formatted_input = format_test_input(test_input)
                simulated_output = simulate_output(test_input)
                # TODO: if the formatted_input ends with a newline, remove the newline
                if formatted_input.endswith("\\n"):
                    formatted_input = formatted_input[:-1]
                f.write(f"input:\\n{{formatted_input}}\\n")
                f.write(f"output:\\n{{simulated_output}}\\n")
                f.write("-------------------------\\n")
    finally:
        cleanup()
"""
        else:
            template_fuzz = f"""import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate


# File Configs
output_file = "../../../fuzz_outputs/CPP/{self.problem_id}/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "{self.problem_id}.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())
    pass

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    pass

# Compile the C++ program
def compile_cpp():
    try:
        compile_command = ["g++", os.path.join(cpp_folder, cpp_file), "-o", os.path.join(cpp_folder, executable_name)]
        subprocess.run(compile_command, check=True)
        print("Compilation successful.")
    except subprocess.CalledProcessError as e:
        print(f"Error during compilation: {{e}}")
        raise

# Simulate output for a test case by running the C++ program
def simulate_output(test_input):
    # Format input for the C++ program
    formatted_input = format_test_input(test_input)
    try:
        # Run the C++ program with the generated input
        run_command = [os.path.join(cpp_folder, executable_name)]
        process = subprocess.run(run_command, input=formatted_input, text=True, capture_output=True, check=True)
        return process.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error during execution: {{e}}")
        return "Error"

# Clean up the compiled executable
def cleanup():
    executable_path = os.path.join(cpp_folder, executable_name)
    if os.path.exists(executable_path):
        os.remove(executable_path)
        print("Cleaned up compiled executable.")

# Main program
if __name__ == "__main__":
    try:
        compile_cpp()
        with open(output_file, "w") as f:
            for case_id in range(1, test_cases + 1):
                test_input = generate_test_input()
                formatted_input = format_test_input(test_input)
                simulated_output = simulate_output(test_input)
                # TODO: if the formatted_input ends with a newline, remove the newline
                if formatted_input.endswith("\\n"):
                    formatted_input = formatted_input[:-1]
                f.write(f"input:\\n{{formatted_input}}\\n")
                f.write(f"output:\\n{{simulated_output}}\\n")
                f.write("-------------------------\\n")
    finally:
        cleanup()
"""

        return f"""{template_fuzz}"""


    def generate_json_config(self) -> Dict[str, Any]:
        """Generate JSON configuration file template"""
        return {
            "fuzzing_params": {
                "Constraints": "max_len ==10 ",  # TODO: Adjust based on problem
                "runs": 100
            },
            "problem_info": {
                "contest": f"Weekly Contest {self.contest_num}",
                "problem_number": self.problem_num,
                "title": "TODO: Add problem title",
                "difficulty": "TODO: Add difficulty",
                "source_url": f"https://leetcode.com/problems/...",
                "tags": ["TODO", "Add", "Tags"],
                "test_examples":" ",
            }
        }


    def generate_template(self):
        """Generate all template files"""
        # Create directory structure
        self.create_directory_structure()
        
        # Generate source files
        with open(os.path.join(self.base_dir, f"C_CPP/C/src/{self.problem_id}.c"), 'w') as f:
            f.write(self.generate_c_source())
        
        with open(os.path.join(self.base_dir, f"C_CPP/CPP/src/{self.problem_id}.cpp"), 'w') as f:
            f.write(self.generate_cpp_source())
        
        # Generate constraint files
        for lang in ["C", "CPP"]:
            with open(os.path.join(self.base_dir, f"C_CPP/{lang}/constraints/{self.problem_id}_fuzz.py"), 'w') as f:
                f.write(self.generate_constraint_template(lang))
            
            with open(os.path.join(self.base_dir, f"C_CPP/{lang}/constraints/{self.problem_id}.json"), 'w') as f:
                json.dump(self.generate_json_config(), f, indent=4)
        
        # Generate output files
        with open(os.path.join(self.base_dir, f"fuzz_outputs/C/{self.problem_id}/outputs"), 'w') as f:
            f.write("")
        with open(os.path.join(self.base_dir, f"fuzz_outputs/CPP/{self.problem_id}/outputs"), 'w') as f:
            f.write("")

def main():
    parser = argparse.ArgumentParser(description='Generate fuzzing test templates for LeetCode problems')
    parser.add_argument('contest_num', type=int, help='LeetCode contest number')
    parser.add_argument('problem_num', type=int, help='Problem number within the contest (1-4)')
    
    args = parser.parse_args()
    
    if not (1 <= args.problem_num <= 4):
        print("Error: Problem number must be between 1 and 4")
        sys.exit(1)
    
    generator = TemplateGenerator(args.contest_num, args.problem_num)
    generator.generate_template()
    print(f"Successfully generated templates for Weekly Contest {args.contest_num} Problem {args.problem_num}")

if __name__ == "__main__":
    main()