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
    let mut diffs: Vec<i32> = numbers
        .iter()
        .enumerate()
        .map(|(i, &e)| e - match i {
            0 => 0,
            _ => numbers[i - 1]
        })
        .collect();
    diffs.push(3); // for the last diff
    let mut count1s: usize = 0;
    let mut ways: u64 = 1;
    let mut temp = 0;
    let mut res = vec![1];
    let mut i = 1;
    // for each group of consecutive 1 step diff, count the no of ways and multiply altogether
    for &d in &diffs {
        if d == 1 {
            count1s += 1;
        } else {
            // adapted method 3 of https://www.geeksforgeeks.org/count-ways-reach-nth-stair/
            while i <= count1s {
                let s: i32 = i as i32 - 4;
                let e = i - 1;
                if s >= 0 {
                    temp -= res[s as usize];
                }
                temp += res[e as usize];
                res.push(temp);
                i += 1;
            }
            ways *= res[count1s];
            println!("{:?} {}, {}", res, count1s, ways);
            count1s = 0;
        }
    }
    println!("{:?} {}", diffs, ways)
}

// ans 21156911906816