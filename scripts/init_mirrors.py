import os
import subprocess
import sys
from os import path

base_dir = sys.argv[1]

print("Executing python script in " + base_dir)

for line in open(sys.argv[2]):
    if line and not line.isspace() and line[0] != '#': # last check is a simple way to comment out lines
        config = line.strip().split("|")
        if len(config) != 3:
            print("Invalid config.Use this format --> name|source|destination ")
            exit(1)
        [name, source, destination] = config
        subprocess.Popen(["git", "clone", "--mirror", source, name], cwd=base_dir).wait()
        subprocess.Popen(["git", "remote", "set-url", "--push", "origin", destination], cwd=path.join(base_dir, name)).wait()
    