use std::collections::HashMap;

fn main() {
    let input = "49
89
70
56
34
14
102
148
143
71
15
107
127
165
135
26
119
46
53
69
134
1
40
81
140
160
33
117
82
55
25
11
128
159
61
105
112
99
93
151
20
108
168
2
109
75
139
170
65
114
21
92
106
162
124
158
38
136
95
161
146
129
154
121
86
118
88
50
48
62
155
28
120
78
60
147
87
27
7
54
39
113
5
74
169
6
43
8
29
18
68
32
19
133
22
94
47
132
59
83
12
13
96
35";
    let mut numbers: Vec<i32> = input.lines().map(|x| x.parse::<i32>().expect("nan")).collect();
    numbers.sort();
    let diffs: Vec<i32> = numbers
        .iter()
        .enumerate()
        .map(|(i, &e)| e - match i {
            0 => 0,
            _ => numbers[i - 1]
        })
        .collect();
    let mut diffs_map: HashMap<i32, i32> = HashMap::new();
    for d in diffs {
        *diffs_map.entry(d).or_insert(0) += 1;
    }
    let diff1 = diffs_map.get(&1).unwrap_or(&0);
    let diff3 = diffs_map.get(&3).unwrap_or(&0) + 1;
    println!("{:?} {}", diffs_map, diff1*diff3)
}
// ans 2414