#!/usr/bin/env python3
# Simple Python script for testing
"""
This is a docstring
spanning multiple lines
"""

def greet(name):
    # This is a comment
    return f"Hello, {name}!"


if __name__ == "__main__":
    print(greet("World"))

# Expected counts:
# Blank: 3
# Comment: 6 (shebang is code, but # comments and docstring)
# Code: 6
# Total: 15
