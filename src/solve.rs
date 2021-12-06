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

//--------------------------day4---------------------------
use crate::parse::Grid;

fn find_winning_grids(grids: Vec<Grid>, drawn: Vec<usize>) -> Vec<Grid> {
    let mut winning_grids = Vec::new();
    for grid in grids {
        let mut found_grid = false;
        for line in grid.clone() {
            if line.iter().all(|num| drawn.contains(num)) {
                winning_grids.push(grid.clone());
                found_grid = true;
            }
        }
        if !found_grid {
            for col in 0..grid[0].len() {
                if grid.iter().all(|line| drawn.contains(&line[col])) {
                    winning_grids.push(grid.clone());
                }
            }
        }
    };
    winning_grids
}

pub fn p4_1(input: (Vec<usize>, Vec<Grid>)) -> usize {
    let (drawn, grids) = input;
    let mut used_numbers: Vec<usize> = Vec::new();
    let mut winning_grids: Vec<Grid> = Vec::new();
    for last_draw in 0..drawn.len() {
        if winning_grids.len() == 0 {
            used_numbers.push(drawn[last_draw]);
            winning_grids = find_winning_grids(grids.clone(), used_numbers.clone());
        }
    }
    assert!(winning_grids.len() == 1);
    let sum_of_remaining = winning_grids[0]
        .concat()
        .iter()
        .filter(|num| !used_numbers.contains(num))
        .sum::<usize>();
    sum_of_remaining * used_numbers.pop().unwrap()
}

pub fn p4_2(input: (Vec<usize>, Vec<Grid>)) -> usize {
    let (drawn, grids) = input;
    let mut grids_not_won_yet: Vec<Grid> = grids;
    let mut used_numbers: Vec<usize> = Vec::new();
    let mut winning_grids: Vec<Grid> = Vec::new();
    for last_draw in 0..drawn.len() {
        if grids_not_won_yet.len() != 0 {
            used_numbers.push(drawn[last_draw]);
            let new_winners = find_winning_grids(
                grids_not_won_yet.clone(),
                used_numbers.clone());
            grids_not_won_yet = grids_not_won_yet
                .into_iter()
                .filter(|grid| !new_winners.contains(grid))
                .collect();
            winning_grids.append(&mut new_winners.clone());
        }
    }
    let sum_of_remaining = winning_grids.last()
        .unwrap()
        .concat()
        .iter()
        .filter(|num| !used_numbers.contains(num))
        .sum::<usize>();
    sum_of_remaining * used_numbers.pop().unwrap()
}

//--------------------------day5---------------------------
use crate::parse::{Line, Coord};

fn to_points(line: Line) -> Vec<Coord> {
    use std::cmp::{min, max};
    let (start, end) = line;

    if start.x == end.x {
        let (min_y, max_y) = (min(start.y, end.y), max(start.y, end.y));
        (min_y..max_y+1).map(|y| Coord{x: start.x, y: y}).collect()

    } else if start.y == end.y {
        let (min_x, max_x) = (min(start.x, end.x), max(start.x, end.x));
        (min_x..max_x+1).map(|x| Coord{x: x, y: start.y}).collect()

    } else { //diagonal line
        let xrange = if start.x < end.x {
            (start.x..end.x+1).collect::<Vec<usize>>()
        } else {
           (end.x..start.x+1).rev().collect::<Vec<usize>>()
        };
        let yrange = if start.y < end.y {
            (start.y..end.y+1).collect::<Vec<usize>>()
        } else {
            (end.y..start.y+1).rev().collect::<Vec<usize>>()
        };
        xrange.iter()
            .zip(yrange.iter())
            .map(|(x, y)| Coord {x: x.clone(), y: y.clone()}).collect()
    }
}

pub fn p5_1(input: Vec<Line>) -> usize {
    use std::collections::HashMap;
    input.into_iter()
        .filter(|(start, end)| start.x == end.x || start.y == end.y)
        .map(to_points)
        .collect::<Vec<Vec<Coord>>>()
        .concat()
        .iter()
        .fold(&mut HashMap::new(), |hmap: &mut HashMap<Coord, usize>, coord| {
            let previous_count = hmap.get_mut(coord);
            match previous_count { 
                Some(count) => *count+=1, 
                None => {hmap.insert(coord.clone(), 1); }
            };
            hmap
        })
        .values()
        .map(|n| n.clone())
        .filter(|n| n>&1)
        .count()
}

pub fn p5_2(input: Vec<Line>) -> usize {
    use std::collections::HashMap;
    input.into_iter()
        .map(to_points)
        .collect::<Vec<Vec<Coord>>>()
        .concat()
        .iter()
        .fold(&mut HashMap::new(), |hmap: &mut HashMap<Coord, usize>, coord| {
            let previous_count = hmap.get_mut(coord);
            match previous_count { 
                Some(count) => *count+=1, 
                None => {hmap.insert(coord.clone(), 1); }
            };
            hmap
        })
        .values()
        .map(|n| n.clone())
        .filter(|n| n>&1)
        .count()
}

//--------------------------day6---------------------------

pub fn p6_1(days: usize, input: Vec<usize>) -> usize {
    if days < 1 {
        input.iter().count()
    } else {
        let one_day_older = input.iter()
            .map(|&age| if age == 0 {
                    Vec::from([8, 6])
                } else {Vec::from([age-1])
                })
            .collect::<Vec<Vec<usize>>>()
            .concat();
        p6_1(days-1, one_day_older)
    }
}

pub fn p6_2(days: usize, input: Vec<usize>) -> usize {
    let mut fishbowl: Vec<usize> =
        Vec::from([0,0,0,0,0,0,0,0,0,0]);
    fn cycle (ptr: usize) -> usize { ptr.rem_euclid(9) }

    input.iter()
        .for_each(|&day| { fishbowl[day]+=1; });
    for day in 0..days {
        let mature_fish = fishbowl[cycle(day)];
        fishbowl[cycle(day)] = 0;
        fishbowl[cycle(day+7)] += mature_fish;
        fishbowl[cycle(day+9)] += mature_fish;
    };
    fishbowl.iter().sum()
}
