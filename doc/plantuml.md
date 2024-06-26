**use vscode markdown-plantuml-preview extension to preview plantuml code**

```plantuml
' !theme materia-outline

clock   "Clock"   as C0 with period 50
binary  "Reset"  as B
concise "PC" as C

@0
B is high
C is 8000

@100
B is low

@150
C is 8004

@200
C is 8008

@250
C is 800f
```

```plantuml
' !theme materia-outline

clock   "Clock"   as C0 with period 50
binary  "Reset"  as B
concise "PC" as C

@0
B is high
C is 0000

@100
B is low
C is 8000

@150
C is 8004

@200
C is 8008

@250
C is 800f
```

```plantuml
' !theme materia-outline

clock   "Clock"   as C0 with period 50
binary  "Reset"  as B
concise "PC" as C

@0
B is high
C is 7ffc

@100
B is low
C is 8000

@150
C is 8004

@200
C is 8008

@250
C is 800f
```

```plantuml
' !theme materia-outline

clock   "Clock"   as C0 with period 50
binary  "Reset"  as B
concise "PC" as C
concise "W_IF" as W_IF
concise "W_ID" as W_ID
concise "W_EXE" as W_EXE
concise "W_MEM" as W_MEM
concise "W_WB" as W_WB
concise "R_IF" as R_IF
concise "R_ID" as R_ID
concise "R_EXE" as R_EXE
concise "R_MEM" as R_MEM
concise "R_WB" as R_WB
concise "ADDR_IF" as ADDR_IF
concise "ADDR_ID" as ADDR_ID
concise "ADDR_EXE" as ADDR_EXE
concise "ADDR_MEM" as ADDR_MEM
concise "ADDR_WB" as ADDR_WB
concise "DATA_IF" as DATA_IF
concise "DATA_ID" as DATA_ID
concise "DATA_EXE" as DATA_EXE
concise "DATA_MEM" as DATA_MEM
concise "DATA_WB" as DATA_WB

@0
B is high
C is 7ffc

@100
B is low
C is 8000

@150
C is 8004

@200
C is 8008

@250
C is 800f
```

```plantuml
clock "Clock" as C with period 50
binary "Reset"  as B
scale 50 as 100 pixels

@0

@50
B is high

```

```plantuml
clock  "Clock" as C with period 50
binary "Reset" as B
concise "PC" as PC
concise "INST" as INST
scale 25 as 50 pixels

@0
PC is 0x00000000
INST is 0x00000000

@50
B is high
PC is 0x7FFFFFFC

@100
PC is 0x80000000
INST is 0x00000413

@150
PC is 0x80000004
INST is 0x00009117

@175

PC@75 -> INST@125 : SRAM Request
PC@50 <-> @75 : Send SRAM Request

```