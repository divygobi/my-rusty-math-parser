# my-rusty-parser
This is my recursive descent parser for simple arithmatic operations. It supports +,-,*,/, and parentheses 

It's defined by the following CFG.
Credit to https://www.cs.rochester.edu/u/nelson/courses/csc_173/grammars/cfg.html

<expression> --> number
<expression> --> ( <expression> )
<expression> --> <expression> + <expression>
<expression> --> <expression> - <expression>
<expression> --> <expression> * <expression>
<expression> --> <expression> / <expression>

