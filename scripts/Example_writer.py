import os

examples = [
    {
        "input": "4 7\n3 2 5 6\n2\n-6\n4\n-5\n-3\n2\n-7",
        "output": "26"
    },
    {
        "input": "4 5\n-1 4 5 -2\n-5\n-1\n-3\n-2\n-4",
        "output": "-1"
    }
]

output_file = "fuzz_outputs/C/weekly_contest_415_p2/outputs"

def write_examples_to_file():
    with open(output_file, "r") as f:
        existing_content = f.read()

    example_content = ""
    for example in examples:
        example_content += f"input:\n{example['input']}\noutput:\n{example['output']}\n-------------------------\n"

    with open(output_file, "w") as f:
        f.write(example_content + existing_content)

if __name__ == "__main__":
    write_examples_to_file()