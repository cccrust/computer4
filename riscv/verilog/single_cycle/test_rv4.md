```
(.venv) cccuser@cccimacdeiMac single_cycle % ./test_rv4.sh
++ dirname ./test_rv4.sh
+ DIR=.
+ for ex in hello sum fact fib
+ ./link_run.sh ../../rv4/examples/hello.o program.hex
Wrote program.hex (      46 bytes)
+ iverilog -o rv64i_cpu rv64i_cpu.v tb_rv64i.v
+ vvp rv64i_cpu
+ grep -E '(puts|PASS|FAIL)'
puts[14]:i48Hi65ei6cli6cli6foi2c,i20 i57Wi6foi72ri6cli64di21!i0a
  #####  PASS  #####
+ for ex in hello sum fact fib
+ ./link_run.sh ../../rv4/examples/sum.o program.hex
Wrote program.hex (     214 bytes)
+ iverilog -o rv64i_cpu rv64i_cpu.v tb_rv64i.v
+ vvp rv64i_cpu
+ grep -E '(puts|PASS|FAIL)'
puts[14]:i73si75ui6dmi28(i311i2e.i2e.i311i300i300i29)i20 i3d=i20 
puts[4]:d355d300d355d300
  #####  PASS  #####
+ for ex in hello sum fact fib
+ ./link_run.sh ../../rv4/examples/fact.o program.hex
Wrote program.hex (     281 bytes)
+ iverilog -o rv64i_cpu rv64i_cpu.v tb_rv64i.v
+ vvp rv64i_cpu
+ grep -E '(puts|PASS|FAIL)'
puts[5]:i66fi61ai63ci74ti28(
puts[2]:d311d300
puts[4]:i29)i20 i3d=i20 
puts[7]:d333d366d322d388d388d300d300
  #####  PASS  #####
+ for ex in hello sum fact fib
+ ./link_run.sh ../../rv4/examples/fib.o program.hex
Wrote program.hex (     216 bytes)
+ iverilog -o rv64i_cpu rv64i_cpu.v tb_rv64i.v
+ vvp rv64i_cpu
+ grep -E '(puts|PASS|FAIL)'
puts[16]:i66fi69ii62bi6foi6eni61ai63ci63ci69ii28(i322i300i29)i20 i3d=i20 
puts[4]:d366d377d366d355
  #####  PASS  #####
+ echo ==============================
==============================
+ echo '  All rv4 examples PASS!'
  All rv4 examples PASS!
+ echo ==============================
============================== 

這些輸出為何是正確的？
Thought: 6.3s
The user is asking why the output hex dump format is correct. Let me analyze one of the outputs to explain.

Let me take the hello output as an example:

puts[14]:i48Hi65ei6cli6cli6foi2c,i20 i57Wi6foi72ri6cli64di21!i0a

The format from the testbench is:
1. i prefix = reading from imem (address < 0x8000)
2. %02x hex value
3. %c character

So i48H breaks down as:
- i = imem prefix
- 48 = hex value 0x48 = 'H'
- H = character print of 0x48

Let me trace through the whole string:

j=0: byte=0x48 → "i48H" → 'H'
j=1: byte=0x65 → "i65e" → 'e'
j=2: byte=0x6c → "i6cl" → 'l'
j=3: byte=0x6c → "i6cl" → 'l'
j=4: byte=0x6f → "i6fo" → 'o'
j=5: byte=0x2c → "i2c," → ','
j=6: byte=0x20 → "i20 " → ' '
j=7: byte=0x57 → "i57W" → 'W'
j=8: byte=0x6f → "i6fo" → 'o'
j=9: byte=0x72 → "i72r" → 'r'
j=10: byte=0x6c → "i6cl" → 'l'
j=11: byte=0x64 → "i64d" → 'd'
j=12: byte=0x21 → "i21!" → '!'
j=13: byte=0x0a → "i0a\n" → newline

So the string is "Hello, World!\n" — which is correct!

The confusing part is when the hex value and the character happen to be the same symbol. For example:
- "i6cl" — the hex is 0x6c, and the character 0x6c is 'l'. So it looks like "i6cl" where '6' and 'c' and 'l' are part of different things.
- "i6fo" — hex 0x6f = 'o', character = 'o'. So "i6fo".

The output looks cryptic because the hex digit 'c' (from %02x of 0x6c) and the character 'l' (from %c) run together without separators: "i6cl".

Similarly for dmem outputs like:
puts[4]:d366d377d366d355

j=0: hex 0x36, char '6' → "d366" — here 'd' is prefix, '36' is hex, '6' is the printed char
j=1: hex 0x37, char '7' → "d377"
j=2: hex 0x36, char '6' → "d366"  
j=3: hex 0x35, char '5' → "d355"

So the string is "6765" — which is fib(20)=6765. Correct!

The user is asking why this output is "correct" — the answer is that the hex prefix (i or d) + %02x + %c format is just a debug dump mode that shows both the raw hex value and the ASCII character for each byte. The actual content (ignoring the formatting markers) spells out the correct strings.

Let me give a clear explanation.
這是 testbench 的 debug dump 格式。每個位元組輸出三部分：
i48H   →  i  = 讀取來源 (imem)
           48 = 該 byte 的十六進位值 (0x48)
           H  = 以 ASCII 字元輸出同一個 byte
所以 puts[14]:i48Hi65ei6cli6cli6foi2c,i20 i57Wi6foi72ri6cli64di21!i0a 拆解：
片段	hex	ASCII	字串
i48H	0x48	H	H
i65e	0x65	e	He
i6cl	0x6c	l	Hel
i6cl	0x6c	l	Hell
i6fo	0x6f	o	Hello
i2c,	0x2c	,	Hello,
i20␣	0x20	空格	Hello,␣
i57W	0x57	W	Hello,␣W
i6fo	0x6f	o	Hello,␣Wo
i72r	0x72	r	Hello,␣Wor
i6cl	0x6c	l	Hello,␣Worl
i64d	0x64	d	Hello,␣World
i21!	0x21	!	Hello,␣World!
i0a↵	0x0a	\n	Hello,␣World!\n
d prefix 代表從 dmem 讀取（堆疊緩衝區），例如 d355d300d355d300 = "5050"（sum 的答案），d366d377d366d355 = "6765"（fibonacci 答案）。
```
