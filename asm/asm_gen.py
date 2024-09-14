extended_opcode: None | int = None

prelude = """#const(noemit) EXT = {ext}`8
"""

psuedo = """#ruledef psuedo {
    jmp.abs {abs: u16} => EXT @ 0xd5 @ le(abs)
}
"""

with open("instructions.txt") as f:
    raw_lines = f.readlines()

raw_lines = [line.strip() for line in raw_lines if line]

output_lines = ["#ruledef bw8 {"]

opcode = 0
for ln in raw_lines:
    assert opcode < 512, ln

    if ln != "":
        if "_ext" in ln:
            assert opcode < 256, ln
            extended_opcode = opcode
            output_lines.append(f"    {ln.replace("$OP$", "EXT")}")
            opcode += 1
            continue

        if opcode < 256:
            output_lines.append(f"    {ln.replace("$OP$", f"0x{opcode:02x}")}")
            opcode += 1
        else:
            output_lines.append(f"    {ln.replace("$OP$", "EXT @ " + f"0x{opcode-256:02x}")}")
            opcode += 1
    else:
        output_lines.append("")

output_lines.append("}\n")
output = "\n".join(output_lines)

with open("asm/bw8.asm", "w") as f:
    f.write(prelude.format(ext=extended_opcode))
    f.write("\n")
    f.write(output)
    f.write(psuedo)
