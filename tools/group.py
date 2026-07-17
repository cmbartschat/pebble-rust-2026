#!/usr/bin/env python3

from collections import Counter, defaultdict
import math
import sys

# Minimum number of symbols that must share a prefix before it is considered
# a namespace.
MIN_GROUP_SIZE = 3

TOP_LEVEL_GROUPS = [
    "speaker",
    "smartstrap",
    "property_animation",
    "graphics",
    "gdraw_command",
    "app_message",
    "window",
]

OVERRIDES = {
}

BANNED_SEGMENTS = [
    "get",
    "set",
    "create",
    "read",
    "write",
]
    

def build_prefix_counts(symbols):
    """
    Count every underscore-delimited prefix.

    Example:
        action_bar_layer_create

    contributes:
        action
        action_bar
        action_bar_layer
    """
    counts = Counter()

    for symbol in symbols:
        parts = symbol.split("_")
        if "service" in parts:
            counts["_".join(parts[:parts.index("service") + 1])] += MIN_GROUP_SIZE
            continue
        for i in range(1, len(parts)):
            if parts[i - 1] in BANNED_SEGMENTS:
                break
            counts["_".join(parts[:i])] += 1

    return counts


def choose_group(symbol, counts):
    """
    Choose the longest prefix shared by at least MIN_GROUP_SIZE symbols.
    """
    parts = symbol.split("_")
    if parts[0] in TOP_LEVEL_GROUPS: 
        return parts[0]
        
    best = symbol
    for i in range(1, len(parts)):
        prefix = "_".join(parts[:i])
        if counts[prefix] >= MIN_GROUP_SIZE:
            best = prefix

    return OVERRIDES.get(best, best)


def parse(lines):
    entries = []

    for line in lines:
        line = line.strip()
        if not line:
            continue

        status, symbol = line.split(maxsplit=1)
        entries.append((status, symbol))

    return entries


def summarize(entries):
    symbols = [symbol for _, symbol in entries]
    counts = build_prefix_counts(symbols)

    groups = defaultdict(list)

    for status, symbol in entries:
        groups[choose_group(symbol, counts)].append(status)

    for group in sorted(groups):
        statuses = groups[group]
        total = len(statuses)
        complete = sum(s == "🟩" for s in statuses)
        goal_width = 10
        if total > goal_width:
            if complete == total:
                green_count = goal_width
            else:
                green_count = int(complete/ total * goal_width)
                
            spacing_count = 0
            red_count = goal_width - green_count
        else:
            spacing_count = goal_width - total
            green_count = complete
            red_count = total - complete

        spacing = "  " * (spacing_count)
        green = "🟩" * (green_count)
        red = "🟥" * (red_count)

        print(f"{green}{red}{spacing}  {group}")


def main():
    summarize(parse(sys.stdin))


if __name__ == "__main__":
    main()
