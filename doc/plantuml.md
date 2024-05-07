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