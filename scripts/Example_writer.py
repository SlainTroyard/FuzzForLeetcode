import os

examples = [
    {
        "input": "3\n1 2 3\n4 3 2\n1 1 1",
        "output": "8"
    },
    {
        "input": "2\n8 7 6\n8 3 2",
        "output": "15"
    }
]

output_file = "fuzz_outputs/CPP/weekly_contest_413_p3/outputs"

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