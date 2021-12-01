pub fn _p1_1_imperative(vec: Vec<usize>) -> usize {
    let mut acc = 0;
    for i in 1..vec.len() {
        if vec[i] > vec[i-1] { acc +=1; }
    };
    acc
}

pub fn p1_1(vec: Vec<usize>) -> usize {
   let (_, sum) = vec.iter()
       .fold((None, 0), |state, item| match state {
            (None, _) =>
                (Some(item), 0),
            (Some(pred), acc) =>
                (Some(item), if item > pred {acc+1} else {acc})
       });
   sum
}
pub fn _p1_1_zip(vec: Vec<usize>) -> usize {
    let prevs = vec[0..vec.len()-1].iter();
    let nexts = vec[1..vec.len()  ].iter();

    prevs.zip(nexts)
        .map(|(prev, next)| if prev < next {1} else {0})
        .sum()
}

pub fn _p1_2_imperative(vec: Vec<usize>) -> usize {
    let mut acc = 0;
    for i in 3..vec.len() {
        let previous_window: usize = vec[i-3..i-1].iter().sum();
        let current_window: usize = vec[i-2..i].iter().sum();
        if previous_window < current_window {
            acc += 1;
        }
    };
    acc
}

pub fn p1_2(vec: Vec<usize>) -> usize {
    let digits_1 = vec[0..vec.len()-2].iter();
    let digits_2 = vec[1..vec.len()-1].iter();
    let digits_3 = vec[2..vec.len()  ].iter();

    let windows = digits_1.zip(digits_2).zip(digits_3)
        .map(|((d_1, d_2), d_3)| d_1 + d_2 + d_3);

    p1_1(windows.collect())
}
