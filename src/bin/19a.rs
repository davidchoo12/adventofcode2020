use std::collections::HashMap;
extern crate regex;
use regex::Regex;

fn main() {
    let input = r#"25: 53 116
122: 116 92 | 53 53
100: 53 73 | 116 125
111: 67 116 | 91 53
71: 116 58 | 53 78
55: 116 54 | 53 21
123: 53 97 | 116 108
104: 56 116 | 125 53
19: 125 53
56: 116 116 | 116 53
60: 116 121 | 53 59
75: 53 20 | 116 124
6: 53 131 | 116 33
62: 53 53 | 116 116
101: 116 127 | 53 26
128: 116 125 | 53 93
34: 53 28 | 116 125
65: 63 116 | 135 53
50: 32 53 | 76 116
2: 53 10 | 116 132
133: 28 53 | 131 116
85: 53 131
125: 116 53
118: 87 116 | 3 53
135: 73 53 | 125 116
28: 53 53
110: 116 122 | 53 56
91: 5 53 | 74 116
73: 92 116 | 53 53
80: 15 116 | 109 53
124: 128 116 | 17 53
87: 92 53 | 53 116
43: 116 27 | 53 37
66: 46 53 | 93 116
106: 116 77 | 53 83
137: 72 116 | 104 53
93: 116 116 | 53 116
129: 92 103
12: 116 56 | 53 131
45: 53 132 | 116 12
102: 65 116 | 2 53
24: 116 101 | 53 95
14: 116 84 | 53 29
57: 93 116 | 56 53
70: 108 116 | 100 53
51: 116 1 | 53 68
26: 73 53
15: 116 93 | 53 125
42: 53 134 | 116 106
79: 116 33 | 53 122
5: 116 125 | 53 3
74: 53 122 | 116 73
18: 93 53 | 87 116
20: 47 53 | 17 116
114: 116 85 | 53 118
37: 53 73 | 116 87
47: 116 73 | 53 125
107: 116 3 | 53 46
69: 53 25 | 116 125
32: 53 28 | 116 93
76: 3 53 | 125 116
38: 92 33
119: 116 39 | 53 80
8: 42
21: 125 53 | 125 116
58: 75 116 | 119 53
64: 116 44 | 53 113
68: 90 53 | 45 116
1: 23 116 | 60 53
53: "a"
81: 73 53 | 33 116
86: 116 129 | 53 35
90: 110 116 | 13 53
82: 53 93 | 116 25
95: 99 116 | 66 53
27: 116 56 | 53 3
120: 116 51 | 53 40
126: 116 96 | 53 52
116: "b"
84: 116 123 | 53 49
134: 116 61 | 53 14
78: 116 102 | 53 86
96: 130 53 | 98 116
10: 87 116 | 33 53
88: 53 116 | 116 92
40: 126 53 | 24 116
127: 56 116
59: 87 53 | 3 116
11: 42 31
41: 116 137 | 53 16
44: 116 22 | 53 79
109: 88 53 | 87 116
49: 136 116 | 94 53
113: 107 116 | 81 53
77: 116 64 | 53 36
3: 53 116 | 53 53
9: 133 53 | 112 116
98: 92 93
117: 3 92
39: 53 19 | 116 21
16: 6 116 | 117 53
132: 131 116 | 125 53
63: 116 122 | 53 33
22: 131 116 | 56 53
89: 116 125 | 53 131
115: 55 53 | 50 116
99: 116 33 | 53 93
92: 116 | 53
121: 53 73 | 116 56
13: 73 92
103: 53 3 | 116 62
83: 111 116 | 41 53
61: 105 116 | 115 53
130: 56 92
136: 33 116 | 125 53
30: 53 76 | 116 7
94: 92 87
52: 97 116 | 18 53
7: 116 28 | 53 46
97: 25 53 | 33 116
35: 89 116 | 82 53
72: 53 131 | 116 87
48: 34 116 | 38 53
54: 116 46 | 53 131
131: 116 116
23: 116 4 | 53 57
31: 116 71 | 53 120
29: 116 30 | 53 114
108: 116 25 | 53 46
33: 92 92
4: 131 116 | 3 53
36: 70 116 | 9 53
67: 69 116 | 117 53
0: 8 11
105: 48 116 | 43 53
46: 116 53 | 53 116
17: 93 116 | 122 53
112: 122 53 | 3 116

babbbbabaabaaabbbbbaaabbbbababba
ababaaaaabbbabbbbbabbbba
aabbaabaabbababaababbaba
bbbbabaaabaaabbbbbbbbaab
babbbabababaabbaaaabbbba
bbaabbababbaaabaaaababbbabaaaaaaaaaaababbaabbbbaaaaaaabaabababbbbabbbaababbbaabbbbababab
babbabbabbaababbbbbbbaab
baababababbaababaaabaaab
aababaababbaaaabbbababbbbbbbbbaabbbbabbbbbabbababbababaabaabaaaabbabaaaaabbaaaaa
baabbbbaaabbabbbababbbba
baabbabaaaaababbaababbbbaababaaabbbbbaaabababaab
babbabbabaabbabbaabbabaa
baaabbbbabbaaabbaaaaaabbbbabaaba
abababbaaabaabbbabbbaaaa
ababababbbbaaaababaabaaa
baaababbbbbabababbbaaabaaaabaababaaabbbbaaabbbbaabbaabbbabbbaaab
babaaabaabbabaababbabaaaabbbaaaa
ababaaabaabaabaabbbbababbbaabababaababaaabaabbbb
abbaaaabaabababaabaabbaa
baaaaaabaaaaababbababaaa
aabaaabbababaabbbaaabbbbaabbbaaa
baaabbbaaaaaaaaaaabbbbba
abbbbbabbbbabaabaabbbbabbbaabaabaabbbbbb
baabbbababababbabbaaaaaaabbbaabbbababbbaababaaaaaabbbbaa
bbbbbbbbabababaabaaabbbabbaaabbbaaaaaabaabbaabbabbabbbba
abaabbabaabaaaaabaabaaaa
aabababbababaaaaabbbbbaa
bbbabbaaabaaaababababbbabbbaabab
aabababbaabbbababaabaaab
aabaaababaabbabaaabaaaab
abbbabaabbaabbabaaabbbba
baaabbbbaababbabbbbabbaabaabbbbbabbbbabbbabbaaba
bbaaababaabaaabbabbbbbabbabaaabbbbaabbbbaabbbaaabbabbabb
baabaabaaababbbbbbaabbaa
bbaaabbbbababbabaaaababa
bbabbbbabaaaaabbbaaaaababbbbbaaabaababbbbabbabaabaabbabaabbababbaaaaaaabbbabaaabbbbbbabb
abaabbbababaaaabaabbbbbbbbabbbab
ababaaaaabababbabbbaababbaaabbaaabbaaaba
bbabbaabababbabbbaaaaaabaaabaabb
baabaaaaababbbbabbabbabbbababbaa
aaaaaaaabaaabaabababaaba
baabbababaaabaaaaabaabaaaababbbbaaabbbbb
abaaaabaabbababaababbbab
bbaaabababbababbababbbbb
baababbaaaabababbaaaaaabbaaaabbb
bbbbbbbaabababbabbabbbba
bbaaaaaaaabaabbbabaabbaa
abbabababbaaaaababbbabbbbbaaaaaaababaaaababaaaba
abaababbaabaaaaaaabaaaab
abababaabbaaabbaabababbaaabbabaa
babbabbbbbbbbbbabbaabbaa
bbabbbaaaabaaababbaaababbaaabaabaabbbaababbaabaaaabbbaaaabbaabbbaabbbbba
abbbaabbababaaabbababbbaabbbbababababaaa
bbaaabbabbbaabaaabbbabaaaaababbbbaababbb
baabaaaabbaaabbaaabbaababbbbbbababbbabbbbbbabbba
abaabbbaabaababaaaabbabb
bbbbabaaaabaabbbbbbbbaab
bbaaaaabababbabbaababbab
babbaabaabbbbaaaaabbbbbb
baaabaabbbbbaaaaabbbaaab
aaabbaabbabbabbbbabbbbba
aabbababaabababaaabbabbbbbbbbbbaaaabbaaaababbabbabaaabaabbababaabbbbbabbbabbbbabaabaabaabaababba
ababaabbaabbbabababbabab
bbbabbbabaaabbbaaaabbbab
aabababbaabaabaaabaaabbbaabbbaaa
aaaaaabaabbabbaabbbbaaaaaabbaabbaaabaaaaabbbbaabaaaabbbaababbbbbababbaaa
ababaabbabababbabababbbaaabababaaabbbbaaaabbbabb
babbabbaabbabbbaababaaba
abbbbaaabbbbbbbabbababbbbaabbaab
babbaabaabbbbabbaaaaaaabaabbabba
bbaaababbaabbabbbbbbabbbbabbabaabbabaaaa
aaabaabaaabbababbababaab
bbaaabbbababaabbaabaaababaabbbbbbbabbaaa
abbabbbabaaabbbbbabbbabaaabbaabababbbbbaabbbaabababaabba
aaaaaaaaabbabbaabbabbbaaaabababbbaaabaababaabbaabbababab
aabbbabababbabaaaaabababbabbbbba
abbababbbbbbabbaaabbbbbb
bbbbbbbaabaaaaababbaabbb
aabababbabaaabbaababbaaabbbbababbbabbabbbaaababbabbbbbabaaabaabaabbabaabaaabbaaa
abbbbbabbbbbabaaabaabaab
abbabbbabbbbabababbbbbba
abababbabaaaaaabbbabaaaa
aaaaababbbbbaabbaaabaabaabaaabababbabbabbbbaabbb
baabbbbabbbaaaaababaaabb
bbaaaababbaaaabaababaaaaaabbbaabbbbabaaa
babbbbbbbabbbaaabababaabbabbbaabbbaaabaa
aabaaabbaabbaabbbababaab
baaaabbbbbabbbababbaababbabaaabaabababbaabaabbabbaaaaabaabbaaabb
abababbabaabbbbaaaaababa
aabbbbabbbbbaabaaaabbaabbbabaabababbbbaabbaabbabbaabbaaa
aaabaabaaabbababbabbbababaabbabb
bbbaaababbbbbbbbbbbababbabbbaaab
bababbaaaaaabbbbbabaaaab
bbaaaabaaaaaaabababbbabaaaaabaaa
aabbabbbbbaaababbbababaa
abbaaaababbbbbabaaaaabbb
bbaaaaaabbbaabababaabaab
ababaabbabaababbbbabbaabbbbbbbbbaaabbababbabbabbabbaabaa
bbaabbbaabababbaaaababbb
bbbbabbaaabaabaababaaaaa
bbaabbababbbbaabbababbbabbbbababbbbaaabbbaaaababaababbbababbaabb
bbaaaababababbabbabbbbaaabbbaaab
babaababaaabbaabbbababaa
ababababababbaabbabababa
abbabbbbaaaaaabbabbbabaababaabbaabbbababbbbaabaaaaaaaaabbbaabbba
bbbaaabaaaaaaaaaababbabbbbaabaab
abbbaabbabbbbaaaaaaabaab
abbbbbbbaaabbbbaaaababbaaaaabbabbababaabbbaabbabaabababa
aababbaaabaababbababaabb
babbaaabaabbabbbbbabbaaaaaaabaab
aabababaabbbbabbaaaaabaaaabaabababbaaaba
baabbabbbbaaabababaabbbb
abbaaabbbbbabaabbbbbaabbabbbbaabbaabaaabaabaaaab
bbaabbbabbbbbabbbababbbb
babaababbbbababaabaaaabbbabbabaabaabababaabababb
bbababbabbaabbbbbbbbabbbabbaabaaabbbaaaabbaabaababaaaaaa
babbbbabbbbbbbaaababbbba
abbbbababbbaaaaaaaabbbab
baaaaaabaabbbaabbabbbabbabbaaabaababbaaaabbaaababbaaaaaabaaaaaabbaababababaaabbbabaabaaa
aabaabbbabbbbaaaaababbaaababaabbabbbabbababaabba
aabababbbbbbabbaabbbbbbb
baabababbbaaaaababbbbaabbabbabbbaaababbaaaabaaab
baaabbbababbaabaabbabaaa
abaababbabbbbaabaababbbbbbbaaaab
aababaababbbabbbababbaaa
baaabbbbbbbbabbabbbbbaab
bbabbaabaaabbbaaaabbbaaa
bbabaabaababbaababbbabbbabaabbabbaabbaab
bbbbbbbaaabbaabaababaaba
aababababaaabbbaababbbaa
babbbbbabbbababbaaababaaaababbaaaaabbbabaaababaaabbbbbbbbbbaaaabbaaabbaabaaaababbbaaaaab
aababbaaabaaaababbbbabaaabbbbabaabbbbabbbabbbabb
bbbaabaaaabbbbabaababbaabbabaababbaabaaaababbbba
abaabbbaabbbaaabbabaaaab
bbbbbbbbaaabbabababaaabbbbaaaabbbabaaabb
baaaaabbbaaaaabbabbaaaaa
ababaaaaaabbbaababbaabba
bbaaaaaaaaabbababaaaaaaaaabaaaab
bbbabababbbbabbaabaaaaaa
bbaaaababaaabbbabbbabaabaabaabab
bbbabbbaaabbbbaaabbaabbaabaabbbaaabbbbba
babbabbabbbbbbbaaababababaaabaabbaaaaaabaaaaabaaabbbbbbaaabbbbbabbaabbaa
ababbabbaabbaabbbaaaaabbaababaaaabbabbab
abbabbbbbaabbbbbbabaabbb
abbbaabbaaaaababbbaabbabaababaabaababaababbbbbbbbbaabaab
baaaaaaabbbabbbaaaabbbba
bbaaabbbababbbabbaaaabaabaabbabbbbbbbbbbabbabbabbababababbaababbbbabbaba
bbbbababbaabbababbabbbab
baaabaabaababbbbbbabbbbb
abbbbababbbababbbbabbababbabaabbbbaaaabb
bbaaaaaaabbbabbbabbbbbaa
bbabaabaabbabbaaababbbaa
aabbbaababaabbabbabaabba
baaabbbaaaabaababbaaaabb
babbbbaabbbbaabaabbbaaaa
abbaabbbaababbabbaaabbababbbabab
aaabaaaaaabbabababbbbabbbbabbbbbaaabaaab
babbbaababbaaaaaaabaaababbaaabbb
bbbbaababbaaaaaabbbbbaab
bbaaaaaaabbbbababaabaaab
babaababbabbabbbaabababaabaabaab
abbbabbbabbabbaaaababaababbabaab
bbbbaaaaaabbaababbabaaabbbabaaaabbabbabb
aaabbaabbaaabbbbbbbbbaaa
abbbbaabaabbbabaabaaabababaabbba
babbabbabaaababbbbaaabbababaabaabaabbaababaabaaa
babbabbaaabaabbaaaaaabaaabbbaabaabbaaaaabbbaaabaaababbaabbbbaabaabbbbaba
aabaaabbabbaaabbbabbbabb
bbaaaaaabaabbbbaabbbaaab
bbbbaaaabbbaaabaabbbbabbbababaaa
abbbabaabbabbaabbabbbbba
aaaaaaabaababbbbababbbab
abbbababbbabbbbabbbbbabbbabaaababbababaabbbaaaab
babaaabaaabbbaaaaabbbbbbabbaaaaabaabaabbabbbababaababaaaaaabbaaababbbabbbaaababb
abbababbbaaaaaabbabaaabb
bbabbababbabbaabbababbbb
babbbbaaaabbaaababbbbaaaaabbababaaabbaabbbbbaaab
aabbaabbabaaaabbaabababaabaaaabaaaabaaaaaaabaabababababa
abababbaaaabaabaaaababbb
bababbabaaabbbaaababbbbb
baaaababbabbbabababaabba
abbbbabaaaaababbbbbaabbabaaaabbaaaababba
aabbababbbaababbabaababa
babbaaabbbaabaabbbaaabbabaaaaabaabbabbaabaabbaabbbbbababbbbabbbb
aaabbbaaabbabbabaabaaaaaaabaaaaa
aababbaaaababbbbabbbaaaa
baaabbbabababbbaaabaabba
bbabbaabbbbabaaabaaabbbbbbbaabababbaabbbbabbabbaaabaaabb
baabaabaababababaaabaabb
abbababbbbbbababbbaabaab
abababbabababbabbaaaaaab
bababbbaaabaaabababababb
aabbabbbbaaababbbbabbaaa
abbaaaababbabababbaabbbabbaabaab
bbbabbbaababaaaabbbaaabababbaaaaababbabaaabbabaaaaaaabba
baabaabaababaaabbbbbababbabbabbabababbbb
bbbbbaaabbaababaabbbbbaaababaaba
bbaaaabaabbaaaabbbababbbbbbababaabbabaababbbaaaaabaabaaa
bbabaaaaababbbbbbababbbbbbbababbbabbbbab
ababababbaaabaabbbbaababaababababaabbbaaaaababbbbabaaabb
babbbabaaabbabbbaababbba
aaaaaaaababbbababbaababbabbbaabb
aababababbaaaaaabaaaaaba
aaaaaaabbaaaabaaaababaabbbabbabb
bbabbabbbbbbabbaabbbaabaababbbaabaaabababaabbbbbbabbbbbb
bbbaabbaabaaaabaabaaabaa
babaabbabbabbaaabaabaaaaabaabaaa
aababababbbbaabaabbbbbaa
baabababbaabaabbaaaaaabbaaaaabababbabbaaabbabbaaaaaaabbb
bbabaabaababbabbbabbaabb
abaaaababbbbbbbabbabaaaa
bababbabbbbbabaababbbbbb
aabbabbbbbbbabababbabaab
abbababababbabaaaabbbaaa
aabababbabbaabbbbbbaaaabbbbbaaaa
abbbabbbaaaaaaabbaabbbbb
bbbbbbbaaabaaababbbbababbabaaabaabababbb
aaaababbaaaaaabaaabaabba
abaaaabbbbbbababbbababbbabaabaaababaabaa
bbabbbaaababbabbbbbaaaab
aababaaabaabbbbbbbbabbbb
aabbbaababbbbbabbababaaa
aabbaaababbaababbbbbababbbbbababbbbbaabbbaabbbbbabaabbaabbbabbbb
baaabbbbbbbaaabaabaabaab
baaabbaaaababaabbabaaaaa
aaabababaababbabaababababbbbbbaabbabbbaabaabaabbaaabaabaababbbbb
aabaaabbbaaabaaabbbabaab
bbbaabbaababbabababbbbbababbbabbbbaabbbbbbabaaaaaaaabbab
aabbaaabbbaabbabaaababbb
bbbbabaabbbabbbaaabaaaab
aabbaabbbabaababbaababaa
aaaaaaaababbbbababaaaabaaabababbabaaababababababaaabbabbababbbbbaaaabbab
babbbbabbababbbabaabbaaa
aabbaaabbbaababbbbbbabaaababaabbbabaaabaaabbbabbababbaba
bbbaaaaabbaaabababbabbbb
bbbbabbababbaaaabbbabbbaabbaabbbbbaabaaa
aaaababbabbaababbabababb
bababbabaaabaaabbaabbbaabaaababa
babaababbaaaaaabababbaabbababbaaaaababbb
aabbabababaaabbababaaaba
bbbaaabbabaababbbababbaa
abbabbbaabbaababbabaabba
baaababbaaaaaabaaabbbaaa
baaaaaabaabababbaaabbaabababbaababbaaabbaababbba
abbbbabbbbabbabaaaaaaabbbabbaabb
abbbbaabaaabaababbbababaaabbbaabbabbabbabbabaaababbabbbbbaababaa
bbbabababaabbaaabbaaababbabaaaaabbaaaaaa
abbbbaababbbabbbaabaaaab
bbababaaaaaabababaaabbbbbbabbaaababbabbabaaaababaaaabababbabaabaabaabaab
bbbabbaabbbbabababaaabaa
abaaabbabaaaabaaaaabbabaababbaba
babbaabaabbabbaabaaaaaba
aababababbabbbaaabbaabba
bbbabaabbaaabbbabbabbaaa
bbbababaaabbabababababbb
aabbbbababaaaabbbaabbabbbaabbaab
bbaaaababababaababbbaaab
abbbaabbaaaaabbaaabaaaabbabaaabaaaaabbabbaababaa
babbabaabababbabbbbbabbbaabbbaba
bbbbbbbaabbbaabbbbbabbab
abaabbabbaabaababaabababbbbaaababababbaaaabbbabbabbaabbb
abababbabbaaaaabaabbabba
abbabaaababbababaabaababbabaaaaabbaaabaabbabbaaa
babbabaabbababbbbbabbaaa
aabaaabaababaaaaaaaabbba
bbaabbbabbbbaaaabaabababaabaaabaaabbabba
abaaaabaaabaaabbabbabbbb
abaaaaabaaaaaaaabaaaaaba
aaabbabababbbbaaaaaabbbb
bbbbaabbabaaabababaaabbababbbabaababbaabbabaabba
bbbabbbaaabbbababbaaabbbbbbabbabbbbbbabbabbbbbbb
abaaaababaaabaabababaaba
baaabaaabbabbbaaaabbaababaaaabba
bbbbabbbbbaabbbabaabbbbb
ababbaababbbbabaababaaaaaabbaaabaaaaabbb
baaaababaaaaababbababbaa
aabaababbabbaabbbaabbababaabbaabaaabaaabbabaabbababaaabbabbbaaab
abbabbaaaaaaabbabaaaabbaababbaaa
bbbbaabaabbbbabbaabbabbbaababbbbbabaaaba
bbbaaabbbaaabbbaaaababbb
baaabbbabbbaaaaabbbaaabaaabababbabaaabbababbabab
aabaaabbabaaaaababbbaaab
bbaaaaabaabbabababbbbaaababbababaabaabba
bbaaababbbbbaaaabababbbaaaabbabb
bbababbbbbabbababbbbabaabbbbaaaaaabbbaabbabbabbbbbbabbbb
bababbbaaaabbaabaaaabbba
ababbaababaabbabaaaaabba
aaaaaabaaaabbbaaababbaba
babbbbaaabbbabbbbbbabbab
bbbbbbbabbbbabbbabbaaaba
baaabaaaaabbaabbabaaababaaababbb
baabaabababbabbbaabbbabb
bbaaaaaaabbbbabbabaabbabbabbaaabaaaababaaaaaabba
baaaaaaaaaaaaaaaaaaababa
abbaababaabbbbabaaababba
bbbbababaabbbaabaababbaaaaabaaab
aababaabaabaababbbbbaaaaababaaaaabaababbbabbbbaaabaaaaabababbbab
babaababbbaabbabbbaabbaa
baabbababaaaabaabababbaa
bbaaabbabbaaabbbbbabbabb
baabbbbabbaaaabaaaabbbba
bbababbbbaabbbbaaaaabaab
aaabababaaaaaabbabaaaabababbbbbb
bbbbbbababaaabbbbaaaaaaabbaabbaabbbbbaabaabaaabbaaabbbbabbaabaabbbbaaabbabbbaabb
aabbbaaababbabbbbaabbbbababaaaaababaaaaaaaababbbaabababb
bbabbaabbabbaaaaaaaaabba
abbabbaabbaabbabaaabaabaabbaaaaaaabaaaabbbaabaabaabbabaa
abaababbaabbaaabaaabaaaabbaaaabaabaaababaababaaabbbabbababaaabaaabbaabbabbaaabaa
bbaaaaabbbbbabaaaababbaababbabbbaaabbabababbababbabbbbba
babbbbaaaababbaaaabaabbbbaabbbaababbbbbb
aaaababbbbbbbbaabbbabbbb
bbaaababbbaaabbaaabbbabb
abbaaabbbabbaabababaabbb
aabababbabbbbbababbbaabbaaabaaaaabbabbaababaabaabaaaaabaaababbbaababbbab
babbaaaaaabbbbabbbbbbbaabaabbbabbbbbaaab
babbabaaabbaaabbbabaaaaa
aaaaaaabababababbaababbb
ababaabbbabbaaaaabbbbbbb
aabbaaabaabbbabaaaababba
baabbabaaabaabaaabbaaaaa
abaaaabaaabbbaabbabaabaa
bbaaabbbbaaabbbaababbbbb
baaababbbbbaababbbabbaaa
ababbbbbabbababbaaabaabbaabbbaaaaabbaaabbbbbababbaaaaabaaabbbbaaaabbabaaaababbbbabaabaaa
bbbbabbbababababbaabbaab
ababbaabbabbbbaababbabbabaaabbaabaaabaaabaaaabbbababaababaababbb
baaaababbabbbbabbaabbaab
bbaabbbaabaaabbbbababbaa
bbabbbbaabbaaabaabbaabbaabaabbbb
bbbaaaaabbaababbbbaabbababaababbabbaaaba
aabababbaaabbbaaababaabbabbbbbba
abaaaabaaaabaaaabaaaababaabbbaababaabbabaaaaabbaaababbbabaabbaaababaaaab
baaabbbbbababababaaaababbbbaabaaaabaaabbbabbaabbabbbabaabbaaaaba
abbbbabbbbbbabbbbabbbbbb
bbbbabbbabbaaabbbbabaabaababaababbaaaabb
ababaaaaaababaabbaabbabaabbbabaaababbaaaabbbaaaa
bbbbbbbbbbbbaabbaaaabaab
aababaabbabbabaaaaaabaab
ababaaababbabbbabbaabaaa
bbbababbabbbbbabaaaaabaa
abbbaabbbabbbbabaaabbaaa
abbaaaabbbaaaaabaabaabba
aaabbabaaabaabbbaaabbabb
ababaaababbbbaaababaababbbbaabbbbaabaaaa
baabababbaababbabaabbaab
bbaaabbbaaabaabaababbaba
baaaaabbbbbaabaabaaabaaabaabbbbb
aabbbababaaaaaabababbaaa
baaaababbaaababbaaaaabaa
aabababaabaabbababababbb
bbaaaaaabaababbabbbbabaabbbbbbaaaabbaaabbababaab
bbbbababaabababaaabbbaaa
baaabaababbabbbaaababbaa
babaaaaaabbaababaaabbbbaabababbababaaaababaabaabbbbaabbbaaaaababaabaabbabbbbbbbbabababbaabaaaaab
bbbaababbaabbbbabaaaabaaabbbbbba
aababaabaababaaabbbaaababbbabaabbbaabaab
bbbbababaaabaaaabbbaaaab
bbbbbbbbabbbabbbaababaabbbabbabbbbaabaab
baaabaaabaaabbbbaabaaabbaaabaabaaabbbbaa
bbbabaabbaaaababaaabaaab
ababababbbbbabbbaaabababbaabbaab
abbababbaababaaaaababbaa
ababababaabbaabbbababaaa
ababbabbaaaaaabbababbbab
aaabbabbbaabababaabbaabbaaabaabbbabbaabb
bbbbababaabaaaaaabaaaaaa
bbaababaababbaabaaaabaaaaabbabaabaaaaaaaaabbaaba
bbababbbbaaaaabbaaababba
bbbbbbaaabbaababbaaabbaaabbbbabababbbbba
aabbbaabbbbababbbaaababa
aabaabaabaaabaababbabbbb
bbbbabaabaabbbbababaabab
abbbaaaaaabbbbbababbababbaaaabba
aaaaaaabaaabaababbbaaaaaaabbbbbbbbaabaaaabbaaaba
baaabbabbaaabaababbaabababbabbbabaabababaaababbbbbabbaabaabbbabb
aaaaababbaaabbaabbaaaabb
bbbabbbabaabbabaabbbabbaababaaaabbbbabbabbaabbaaaabaaabbabbbbaab
baabaabaabbbbbababaaaabbbbbabaabbbbaaaabbbbbbaabbaabbbbb
bbaaaaaabbabbaabbaabaaba
abbababbbabbbbabbaaaaaabaaaaaabaaaaaabbb
abaababbaabbbaabbabaabbb
baabababbaabababbbbaaaaabbababbbbaaaabbb
baabaabaabaabbbbbaabababbabababbbbabbbaabaaaaabaaaaabaaababbbbababbbaaabbbabaaba
abbbbaaaaaabbbaabaaaabaaaaaabbaa
baababababbaaabbbabbaabb
aaaaaabaabababbabbabbbbb
aaabbabaaabbaabbbbabaaab
abbbaabbbbbbabaaaaaabaaa
aabbabbbabaaaabbaabbbaaa
aabbababbaaabbbbabbbbbabaabaabab
bbbaababaabbbbbabbabaaaabaaaabba
bbbbababaabababaaabbabaa
ababaabbbbbbbbbaaaaabbbb
aaabababbabbaaaaababbbbb
bbbbabbabbbbbbaaaabaabbbbabbbbba
bbbababbaaababababbaababbbabbaabbabaaabaabbabaab
baaabbbaabababbaababbbaa
aaaaaababbbabbaaaabaaaaaabababababbaabaababaaaba
bbabbaabbbbbbbaabbbaaaab
babbaabaabbaaabbbbaabbbaabababbaaaabbbaabaababbbabbbaaaa
baabaabbbaaababababaaabbabbbbaaaaabaaaaababbabaaabbbbbbabaabaababbbbbaaabaaaaaaa
abbaababbbbabaabababbaba
bbaababbababababbbbbbaab
bbbbaababbaaaababaaabbaaabaabbbbbbbbbaaa
babbbaaabaababaababbbabbabbaabbabbaaaabaaababaababaabaaaabbaaaba
bbababbbabaaaaabbbbbbabb
babbabbbababaaaababbbabaaaabbababbabbbbbabababbb
abbbaabbbbaaabbbaaabbabaaaaaaababaabaabbaabbbaabababbbabbbbbbbabababbbbbaaababaa
aaaaabbabbbaaaabbaaababbbaaaababaaabbabbbabbabbaabbabaabaabbaaaaabbaabaababaabaababbabaa
baaababaababbbabbaaaabbaababbaaababababababbaabbbbbabbbababbababaaabbbbaababbaba
babbabbaabbbabaaabbaaaaa
bbbaaabaaaabababbaaabbbabababbbb
bababbbaababaaaabaabaabaaabbabbbbbabababbabbbaab
aababaaaabaabbabbabaabaa
aababbaaaaabbaabaaaababa
babbbbaababbbbaaaaabbbaabbaababbbbabaaabbaaabbabababbbaa
abaaaababababbabbababbbabbabbaabbbaaabaa
aabbababaabbaababaaaaabbbabbaaaabababbabbbbabbaaaaabbbbb
aabaaababbaaaabbaabbabaaaaababbabababbbb
abbbbbaabbbabaaaaaaaabbaaabbbaaa
bbbbabaaaabbbababbbabbbb
bbbababaababaaabaabbabaa
abaababbbaaababbbabbbbaaabaabaab
aaabbaababbbbaaaaabbbbba
aabaaaabababbaaaaaabaabbaaaabbaa
baaabbabbbabaaabaaaabaabababbbbb
abaaaaabbbaaaaaabaababaa
bababbabbbbabbaaaaabbbab
abaaababbaabbabaaabaaabbabbbbabaaaaababaaabaaaabbbbabbbbbaabbaaa
aabababbaaabaaaaabbaabba
ababaaabbabbabaabbbaaaaaaaaaaabbaaabaaab
aabaaababbbababaaabaabaabbbbaaaaaabbaaaa
bababaaabbababbabbbabaaa
abaaabbabaaabaabbababbaa
abbbabaabbaabbbababbbbbb
aabbbbabbaaabaaabbaaabaa
bbbbabbaabbbbbbaaaababaa
abbbbabaaabababbaaabbbbb
abaaabbaabaaabbbbbaabbbb
aabaaabaaaaaaabaaaaabbba
abbabbbaababbabbaaaabbbb
babbaabaaaaaaaabbaabbbbabaaaababaabbaaaa
ababbabbabaaaabbbbbababbaabbababbbbababb
abbbabbbbabbbbabbbaabaab
baaababbbbaaabbabbbaaaab
abbbbaabbbbbaabbabaaababbaabaaab
bbbbababbbaaabaaabbababbbabbbbababaababaaaabaaaabaaaabaaabbbaabababbbabbbaaaaaababbaabbb
bbbbabaabbabaabbabbbbbabbabaaabbaabbaaab
baabbabaaabbaababbbbbaab
bbabaaabaabaaaababbbbbaabaaabbaabababbaaaabbaaabaaaababbaaaababaaaaaaaaaababbbbb
abaaaabaabbbabbbaaaaababbaaaaaab
abbbabbbaababbbbabaaaababbaaabbaabbbaabbbbabaaaa
bbaaabbabbaababbbbbabbaabaabbbbabbbaababababbbaa
bbaaaaaababbbbbbbaababaaababbaaabaabaaabbbababbabbabbbbababbaaab
bbaababbbbaaabbbababbaba
bbbaaabbbbbbabaabbabbaaa
bbbababaaababbbbbaababaa
baaaababaababaaaabbbbbbb
aabbbbabaaaababbbabaabba
baaabaabbbbabbbabbaaabbaabbbaabbbbaabaab
aabbbaabbaaaaaaabaaaaabbbbbaabab
baaaabaaaabbaabaabaaababaaaabaab
aaaaaabaaababbaabaaaaaba
aaabaaaaaabbbbabbaabbaaa
abaababbbbbabbbaaaaaaaaaaabaaabbbaabbbabaaabaabb
bbbaabbaabbababaabbabaaa
bbbababbabbabbbabbaaabbabababbaaababbbbb
abaabbbaababbbaaabbabaab
aababaabaaabbababababbbaaaabbabababbbbaaaaaabbbababbabab
bbabbbaaaaaaababbbaaaabababbabaaabbabbbbaaabbbbb
aaaaaabbbbbaaabbbbaabaaaaabbabaa
bbbaabbababbaabaaabbbbbb
abbbabaaaaaaababaababbab
bbabaabaaabbaabbaabbabba
baaaabaaaababaabbbabbbab
bbaabbbaaaabbaaaaaaaabaaaaaabbaa
aabaabaaaababaabbbbaabaabbbaaabbbbabbabb
aaabaabaabbbabbbaabbbbbb
babbaaaaabaaabbbbbaaababaaaaaabaababbbababbabbbbaaabbbba
abbababbbaabbabbabaabbabaaaaabbb
ababaaabbaaaaabbabbbabaabbaabbbaabaaaaabbaabaabbabbbaaaa
babbaaaabaabababbaaaabbb
aaabbbaaaabbbabababaabaa
bbbaaabaababbaabababaaba
aaababbaaabbaababbbbabbabbbabbab
abababbaaababaababbaabba
bbaaabbabbbbababaaaabaab"#;
    let split: Vec<&str> = input.split("\n\n").collect();
    let criteria = split[0];
    let tests = split[1];
    let lines: Vec<&str> = criteria.lines().collect();
    let mut map: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
    let mut regexmap: HashMap<i32, String> = HashMap::new();
    for line in lines {
        let split: Vec<&str> = line.split(": ").collect();
        let index = split[0].parse::<i32>().expect("nan");
        let combos: Vec<&str> = split[1].split(" | ").collect();
        if combos[0].contains("\"") {
            regexmap.insert(index, String::from(&combos[0][1..combos[0].len()-1]));
        } else {
            map.insert(index, combos.iter().map(|pair| pair.split(" ").map(|s| s.parse::<i32>().expect("nan")).collect()).collect());
        }
    }
    println!("map {:?}", map);
    println!("regexmap {:?}", regexmap);

    fn regex(map: &HashMap<i32, Vec<Vec<i32>>>, regexmap: &mut HashMap<i32, String>, num: i32) -> String {
        if regexmap.contains_key(&num) {
            return regexmap.get(&num).unwrap().clone();
        }
        let mut curr = String::from("(");
        for group in map.get(&num).unwrap() {
            // println!("{:?}", group);
            for n in group {
                // println!("  {:?}", n);
                curr += regex(map, regexmap, *n).as_str();
            }
            curr += "|"
        }
        curr.pop();
        curr += ")";
        regexmap.insert(num, curr.clone());
        return curr;
    }
    let mut r = regex(&map, &mut regexmap, 0);
    r = format!("^{}$", r);
    println!("{:?}", r);
    let re = Regex::new(r.as_str()).unwrap();
    let mut valid = 0;
    for s in tests.lines() {
        let m = re.is_match(s);
        if m {
            valid += 1;
        }
        println!("{} {}", s, m);
    }
    println!("valid count {}", valid)
    // failed attempt at using iterative dynamic programming
    // let it = s.chars();
    // let c_stack: Vec<Vec<i32>> = map.get(0).clone();
    // let n_stack: Vec<i32> = vec![];
    // while let Some(combo) = c_stack.pop() {
    //     while let Some(num) = n_stack.pop() {
    //         if regexmap.contains_key(num) {

    //         }
    //     }
    // }
}

// ans 235
// final regex:
// ^(((a(b((((a(aa)|b(ba))b|((b|a)((b|a)(b|a)))a)b|(b(b(bb|ba)|a(ab|aa))|a(a((b|a)b|aa)|b((b|a)a|ab)))a)b|((b(b(ba|ab)|a(bb))|a((ba)a|(ba)b))a|((a(aa)|b(bb|ab))a|((ab|aa)a|(ba)b)b)b)a)|a(b(b(a((ab)a|((b|a)(b|a))b)|b(b(ab)|a(ba|ab)))|a((((b|a)(b|a))b|(ba)a)b|((b|a)((b|a)a|ab))a))|a(b(a((ab|aa)a|(ba)b)|b(b(aa)|a(ba|ab)))|a(b(a(bb))|a(((b|a)a|ab)b|(ab|aa)a)))))|b(b(b(b(b((bb)b|(bb|ba)a)|a(b((b|a)(b|a))|a(b(b|a)|aa)))|a((b(ab|aa)|a(ba|ab))b|(((b|a)b|aa)a|((b|a)(b|a))b)a))|a(((b(ab)|a(ba|ab))b|(a((b|a)b|aa)|b(ba))a)b|(((aa)a|(bb)b)a|((b(b|a)|aa)a|(ab|aa)b)b)a))|a((((a(ab)|b(ba))b|((ab|aa)(b|a))a)b|((b(ba)|a(ab|aa))a|(a(b(b|a)|aa)|b((b|a)b|aa))b)a)b|(b((a(bb)|b((b|a)a|ab))b|((bb|ba)b|(ba)a)a)|a((a(bb)|b((b|a)(b|a)))b|((ab|aa)(b|a))a))a))))((a(b((((a(aa)|b(ba))b|((b|a)((b|a)(b|a)))a)b|(b(b(bb|ba)|a(ab|aa))|a(a((b|a)b|aa)|b((b|a)a|ab)))a)b|((b(b(ba|ab)|a(bb))|a((ba)a|(ba)b))a|((a(aa)|b(bb|ab))a|((ab|aa)a|(ba)b)b)b)a)|a(b(b(a((ab)a|((b|a)(b|a))b)|b(b(ab)|a(ba|ab)))|a((((b|a)(b|a))b|(ba)a)b|((b|a)((b|a)a|ab))a))|a(b(a((ab|aa)a|(ba)b)|b(b(aa)|a(ba|ab)))|a(b(a(bb))|a(((b|a)a|ab)b|(ab|aa)a)))))|b(b(b(b(b((bb)b|(bb|ba)a)|a(b((b|a)(b|a))|a(b(b|a)|aa)))|a((b(ab|aa)|a(ba|ab))b|(((b|a)b|aa)a|((b|a)(b|a))b)a))|a(((b(ab)|a(ba|ab))b|(a((b|a)b|aa)|b(ba))a)b|(((aa)a|(bb)b)a|((b(b|a)|aa)a|(ab|aa)b)b)a))|a((((a(ab)|b(ba))b|((ab|aa)(b|a))a)b|((b(ba)|a(ab|aa))a|(a(b(b|a)|aa)|b((b|a)b|aa))b)a)b|(b((a(bb)|b((b|a)a|ab))b|((bb|ba)b|(ba)a)a)|a((a(bb)|b((b|a)(b|a)))b|((ab|aa)(b|a))a))a)))(b(b((a((b((b|a)b|aa)|a(ba))a|((bb|ab)b|(b(b|a)|aa)a)b)|b((b(ba)|a(bb|ab))b|((bb|ab)b|(b(b|a)|aa)a)a))b|(b(a((ba)a)|b((ba)a|(ba)b))|a((b(bb|ab)|a(ba))b|((ab|b(b|a))a|((b|a)a|ab)b)a))a)|a(b(((b(b(b|a)|aa)|a((b|a)(b|a)))b|(((b|a)b|aa)a|(ba)b)a)b|(a(((b|a)a|ab)b|((b|a)(b|a))a)|b((bb)b|(ba)a))a)|a(b((b|a)(a(ab|aa)|b(aa|bb)))|a((b(ba)|a(bb))b|(a(bb|ab)|b(ab))a))))|a(b(b((b((bb)b|(ab|aa)a)|a((bb|ab)b|(bb|ba)a))b|(b(a((b|a)b|aa)|b(bb|ba))|a(((b|a)a|ab)a|(ab|aa)b))a)|a(((b(b(b|a)|aa)|a(bb|ba))b|(((b|a)b|aa)(b|a))a)a|(a((bb)b|(ba)a)|b(b(bb|ba)|a(bb)))b))|a((b(((bb|ba)(b|a))a|((b|a)(bb|ab))b)|a(((ab)a|((b|a)(b|a))b)b|((bb|ab)a|((b|a)a|ab)b)a))a|(b(b((bb|ba)b)|a(((b|a)b|aa)a))|a((b((b|a)(b|a))|a(bb|ab))b|((ba|ab)a|(bb|ab)b)a))b)))))$