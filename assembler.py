OPCODES = {
    "noop":    0x00,
    "const":    0x10,
    "load":     0x11,
    "gload":    0x12,
    "store":    0x14,
    "gstore":   0x15,
    "call":     0x18,
    "add":      0x40,
    "sub":      0x41,
    "mul":      0x42,
    "div":      0x43,
    "pow":      0x44,
    "mod":      0x45,
    "shl":      0x50,
    "shr":      0x51,
    "and":      0x52,
    "or":       0x53,
    "xor":      0x54,
    "not":      0x55,
    "cmpeq":    0x61,
    "cmppne":   0x62,
    "cmpgt":    0x63,
    "cmplt":    0x64,
    "reljump":  0x80,
    "reljumpeq":0x81,
    "reljumpne":0x82,
    "reljumpgt":0x83,
    "reljumplt":0x84,
    "jmp":      0x88,
    "ret":      0xA0,
    "print":    0xE0,
    "halt":     0xF0,
}

def read_lines(filename):
    with open(filename) as f:
        return f.readlines()


def parse_line(line):
    key = line[0]
    action_value = OPCODES[key]
    try:
        value = int(line[1])
    except IndexError:
        value = None

    return action_value, value

def bytes_for_line(line):
    action, value = parse_line(line)
    line_bytes = [action]
    if value:
        d = line & 0xFF;
        c = (line >> 8) & 0xFF
        b = (line >> 16) & 0xFF
        a = (line >> 24) & 0xFF
        line_bytes.extend([a, b, c, d])
    return line_bytes


def split_lines(lines):
    rv = []
    for line in lines:
        rv.append([l.strip() for l in line.split(" ")])
    return rv


def strip_comments(lines):
    return [l.split(";")[0].strip() for l in lines]


def main():
    lines = read_lines("test.sl")
    nocom = strip_comments(lines)
    parsd = split_lines(nocom)
    print("Parsed: {}".format(parsd))
    for line in parsd:
        j = bytes_for_line(parsd)


if __name__ == "__main__":
    main()
