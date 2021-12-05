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

//--------------------------day2---------------------------
use crate::parse::Command;

pub fn p2_1(vec: Vec<Command>) -> usize {
    let (forward, down) = vec.iter()
        .fold(
            (0,0),
            |(forward, down), order| match order {
                Command::Forward(distance) => (forward + distance, down),
                Command::Down(distance)    => (forward, down + distance),
                Command::Up(distance)      => (forward, down - distance)
            });
    forward * down
}

pub fn p2_2(vec: Vec<Command>) -> usize {
    let (forward, down, _) = vec.iter()
        .fold(
            (0,0,0),
            |(forward, down, aim), order| match order {
                Command::Forward(distance) =>
                    (forward + distance, down + (aim * distance), aim),
                Command::Down(angle) =>
                    (forward, down, aim + angle),
                Command::Up(angle) =>
                    (forward, down, aim - angle)
            });
    forward * down
}

//--------------------------day3---------------------------
pub fn p3_1(input: Vec<Vec<usize>>) -> usize {
    let mut sum_vec: Vec<(usize, usize)> = vec![(0, 0); input[0].len()];
    for input_line in input {
        for i in 0..input_line.len() {
            if input_line[i] == 1 {
                let (os, is)= sum_vec[i];
                sum_vec[i] = (os, is+1);
            } else {
                let (os, is) = sum_vec[i];
                sum_vec[i] = (os+1, is);
            }
        }
    };
    let mut gamma = 0;
    let mut epsilon = 0;
    for bit in sum_vec {
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        let (os, is) = bit;
        if os > is {
            epsilon +=1;
        } else {
            gamma +=1;
        }
    };
    gamma * epsilon
}

struct Count {
    ones: usize,
    zeros: usize,
}
fn count(input: Vec<Vec<usize>>, bit: usize) -> Count {
    input.iter()
        .map(|vec| vec[bit])
        .fold(Count {ones:0, zeros:0}, |count, i|
            if i == 1 {
                Count {ones: count.ones+1, .. count}
            } else {
                Count {zeros: count.zeros+1, .. count}
            })
}

fn mk_usize(vec: Vec<usize>) -> usize {
    let mut int: usize=0;
    for bit in vec {
        int = int << 1;
        if bit == 1 { int +=1; }
    }
    int
}

pub fn p3_2(input: Vec<Vec<usize>>) -> usize {
    let mut filtered_input = input.clone();
    for i in 0..input[0].len() {
        if filtered_input.len() > 1 {
            let Count {ones, zeros} = count(filtered_input.clone(), i);
            if ones >= zeros {
                filtered_input = filtered_input.iter().cloned()
                    .filter(|vec| vec[i]==1)
                    .collect();
            } else {
                filtered_input = filtered_input.iter().cloned()
                    .filter(|vec| vec[i]==0)
                    .collect();
            }
        }
    }
    assert!(filtered_input.len() == 1);
    let oxygen = mk_usize(filtered_input[0].clone());

    filtered_input = input.clone();
    for i in 0..input[0].len() {
        if filtered_input.len() > 1 {
            let Count {ones, zeros} = count(filtered_input.clone(), i);
            if ones < zeros {
                filtered_input = filtered_input.iter().cloned()
                    .filter(|vec| vec[i]==1)
                    .collect();
            } else {
                filtered_input = filtered_input.iter().cloned()
                    .filter(|vec| vec[i]==0)
                    .collect();
            }
        }
    }
    assert!(filtered_input.len() == 1);
    let co2 = mk_usize(filtered_input[0].clone());

    oxygen * co2
}
