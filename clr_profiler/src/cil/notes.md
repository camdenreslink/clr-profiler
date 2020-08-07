# Notes

- A method in CIL consists of a method header, the method body, and optionally additional sections (usually exception handlers try/catch/finally).
- There are 2 types of method headers: fat and tiny. The 2 least significant bits of the first byte of the method header indicates if it is fat or tiny.
  - If the value is 2 (e.g. `10` in binary) then it is a tiny header.
  - If the value is 3 (e.g. `11` in binary) then it is a fat header.
- A tiny header consists of only 1 byte. The first 6 bits represent the method size (not counting the header itself). The final 2 bits were used to identify it as tiny.

## Considerations When Rewriting IL

- Expand from a tiny method header to a fat method header if any of the following conditions are not met:
  - No local variables are allowed
  - No exceptions
  - No extra data sections
  - The operand stack shall be no bigger than 8 entries
  - Method body smaller than 64 bytes? (This isn't explicitly stated, but the method size must be able to be encoded in 6 bits)
- The CodeSize will need updated whether we have a fat or tiny header.
- If a fat method header, we may need to update the MaxStack.
- Expand short form opcode to long form opcode if target offset can no longer be stored in 1 byte.
- Exception Handler offsets and length will need updated (length only if we added/removed instructions in a try/catch/finally block).

