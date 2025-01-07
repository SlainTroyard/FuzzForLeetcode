import os

examples = [
    {
        "input": "2080-02-29",
        "output": "100000100000-10-11101"
    },
    {
        "input": "1900-01-01",
        "output": "11101101100-1-1"
    }
]

output_file = "fuzz_outputs/CPP/weekly_contest_414_p1/outputs"

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