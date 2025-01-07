import os

examples = [
    {
        "input": "2\n0\n1\n1\n0",
        "output": "0 1"
    },
    {
        "input": "4\n0\n3\n2\n1\n3\n2",
        "output": "2 3"
    }
]

output_file = "fuzz_outputs/CPP/weekly_contest_415_p1/outputs"

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