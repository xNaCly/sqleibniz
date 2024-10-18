-- literals: https://www.sqlite.org/syntax/literal-value.html

-- numbers see https://www.sqlite.org/syntax/numeric-literal.html
1_000.12_000e+3_5;
.1_000e-1_2;
0xabcdef1;
0XABCDEF1;

-- strings:
'string1';
'string202192ijkandkajdohnakd';
'string3';
'string202192ijkandkajdohnakdstring202192ijkandkajdohnakdstring202192ijkandkajdohnakdstring202192ijkandkajdohnakdstring202192ijkandkajdohnakdstring202192ijkandkajdohnakd';

-- blob:
x'abcdef01234567890';
X'ABCDEF01234567890';

-- null (technically a keyword)
NULL;

-- booleans
true;
false;

-- some more things id consider keywords:
CURRENT_TIME;
CURRENT_DATE;
CURRENT_TIMESTAMP;
