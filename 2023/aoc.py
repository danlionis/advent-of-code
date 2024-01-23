#!/usr/bin/env python

import argparse
import os
import requests
import string
import subprocess
import os
import utils

YEAR = 2023


def session_id():
    with open(".session", "r") as f:
        return f.read().strip()


def input_url(day):
    return f"https://adventofcode.com/{YEAR}/day/{day}/input"


def fetch_input(day):
    # http request
    r = requests.get(input_url(day), cookies={"session": session_id()})
    with open(f"./input/{day:02}.txt", "w") as f:
        f.write(r.text)


def init_day(day):
    # if day already exists
    path = f"./{day:02}.py"

    fetch_input(day)

    if os.path.exists(path):
        print(f"Day {day} already exists")
        return

    with open("./template.py", "r") as f:
        code = f.read()
        print(code)
        t = string.Template(code)
        res = t.substitute(day=str(day))

        with open(path, "w") as f:
            f.write(res)


def run_day(day):
    script = f"./solutions/{day:02}.py"
    subprocess.run(["python", script])


print(session_id())


def main():
    parser = argparse.ArgumentParser()

    subparsers = parser.add_subparsers(title="subcommands", dest="subcommand")

    init_parser = subparsers.add_parser("init", help="Initialize a new day")
    init_parser.add_argument("day", type=int)

    run_parser = subparsers.add_parser("run", help="Run a day")
    run_parser.add_argument("day", type=int)

    args = parser.parse_args()

    # mkdir if not exists

    if not os.path.exists("input"):
        os.mkdir("input")

    print(args)
    if args.subcommand == "init":
        init_day(args.day)

    if args.subcommand == "run":
        run_day(args.day)


if __name__ == "__main__":
    main()
