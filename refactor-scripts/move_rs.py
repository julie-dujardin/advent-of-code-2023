import os
import shutil

main_repo_dir = 'src'

def move_mod_rs_files():
    for day_dir in os.listdir(main_repo_dir):
        day_path = os.path.join(main_repo_dir, day_dir)

        # Skip non-directory files and directories that don't match 'd<n>'
        if not os.path.isdir(day_path) or not day_dir.startswith('d'):
            continue

        mod_rs_file = os.path.join(day_path, 'mod.rs')

        # Check if mod.rs exists
        if os.path.isfile(mod_rs_file):
            # Create new destination filename as d<n>.rs
            new_rs_file = os.path.join(main_repo_dir, f'{day_dir}.rs')

            # Move mod.rs to d<n>.rs
            shutil.move(mod_rs_file, new_rs_file)
            print(f'Moved {mod_rs_file} to {new_rs_file}')

        # Optionally remove the now-empty directory
        if not os.listdir(day_path):
            os.rmdir(day_path)
            print(f'Removed empty directory {day_path}')

if __name__ == "__main__":
    move_mod_rs_files()
    print("mod.rs files moved successfully.")
