import os
import shutil
import re
import subprocess
import toml

# Define directories
main_repo_dir = '../src'
test_data_dir = '../test-data'

# Function to revert changes
def revert_changes():
    # Revert changes in the main repository
    subprocess.run(['git', 'checkout', '--', '.'], cwd='.', check=True)

    # Clean untracked files in the submodule
    subprocess.run(['git', 'clean', '-fd'], cwd=test_data_dir, check=True)

# Ensure test-data directory exists
os.makedirs(test_data_dir, exist_ok=True)

def move_test_files():
    for day_dir in os.listdir(main_repo_dir):
        day_path = os.path.join(main_repo_dir, day_dir)

        # Skip non-directory files
        if not os.path.isdir(day_path):
            continue

        # Create corresponding directory in test-data
        test_data_day_path = os.path.join(test_data_dir, day_dir)
        os.makedirs(test_data_day_path, exist_ok=True)

        # Move files to test-data directory
        for filename in os.listdir(day_path):
            if filename.startswith('input'):
                src_file = os.path.join(day_path, filename)
                dest_file = os.path.join(test_data_day_path, filename)

                # Move the file
                shutil.move(src_file, dest_file)

def extract_test_function(content, test_name):
    """
    Extracts the body of a test function by its name.
    """
    pattern = rf'fn\s+{test_name}\s*\(\s*\)\s*{{(.*?)}}'
    match = re.search(pattern, content, re.DOTALL)
    return match.group(1) if match else None

def update_mod_rs_files():
    for day_dir in os.listdir(main_repo_dir):
        day_path = os.path.join(main_repo_dir, day_dir)

        # Skip non-directory files
        if not os.path.isdir(day_path):
            continue

        mod_rs_file = os.path.join(day_path, 'mod.rs')
        if not os.path.isfile(mod_rs_file):
            continue

        # Read the contents of mod.rs
        with open(mod_rs_file, 'r') as f:
            mod_rs_content = f.read()

        # Prepare to collect expected results
        toml_content = {'p1': {}, 'p2': {}}

        # Extract the bodies of p1 and p2 test functions
        for part_name in ['p1', 'p2']:
            test_body = extract_test_function(mod_rs_content, part_name)

            if test_body:
                # Extract all `assert_eq!` lines from the test body
                pattern = r'assert_eq!\(\s*(\w+)\s*\(\s*"(.+?)"\s*(?:,\s*\d+)?\s*\)\s*,\s*(\d+)\s*\);'
                matches = re.findall(pattern, test_body)

                # Process the matches and store them in the corresponding part
                for func_name, filename, result in matches:
                    file_key = filename.split('/')[-1].split('.')[0]
                    toml_content[part_name][file_key] = int(result)

        # Write updated results to TOML file
        toml_file = os.path.join(test_data_dir, day_dir, 'results.toml')
        with open(toml_file, 'w') as f:
            toml.dump(toml_content, f)

        # Update file paths in `mod.rs`
        new_content = mod_rs_content
        old_path = os.path.join('../src', day_dir)
        new_path = os.path.join('../test-data', day_dir)
        new_content = new_content.replace(old_path, new_path)

        # Write updated mod.rs
        with open(mod_rs_file, 'w') as f:
            f.write(new_content)

if __name__ == "__main__":
    revert_changes()
    move_test_files()
    update_mod_rs_files()
    print("Files moved and mod.rs updated successfully.")
