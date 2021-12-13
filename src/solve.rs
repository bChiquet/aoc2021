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

pub fn _p7_1_brute(input: Vec<usize>) -> usize {
    let max_pos = *input.iter().max().unwrap();
    let min_pos = *input.iter().min().unwrap();
    let mut min_cost: usize = input.iter().sum();
    for pos in min_pos..max_pos+1 {
        let fuel_cost = input.iter()
            .map(|&sub_pos| (pos as isize - sub_pos as isize).abs() as usize)
            .sum();
        if fuel_cost < min_cost {min_cost = fuel_cost};
    }
    min_cost
}

pub fn p7_1(input: Vec<usize>)-> usize {
    let mut sorted = input;
    sorted.sort();
    let median = sorted[sorted.len()/2];
    sorted.iter()
        .map(|&pos| (pos as isize - median as isize).abs() as usize)
        .sum()
}

pub fn _p7_2_brute(input: Vec<usize>) -> usize {
    let max_pos = *input.iter().max().unwrap();
    let min_pos = *input.iter().min().unwrap();
    let mut min_cost: usize = 0;
    for pos in min_pos..max_pos+1 {
        let fuel_cost = input.iter()
            .map(|&sub_pos| (pos as isize - sub_pos as isize).abs() as usize)
            .map(|distance| (distance *(distance+1))/2)
            .sum();
        if fuel_cost < min_cost || min_cost == 0 {min_cost = fuel_cost};
    }
    min_cost
}

pub fn p7_2(input: Vec<usize>)-> usize {
    use std::cmp::min;
    let mean = input.iter()
        .sum::<usize>() as f64 /input.len() as f64;

    fn fuel_with_rounding(rounded: &dyn Fn(f64) -> f64
                         , mean: f64
                         , input: Vec<usize>)
                        -> usize {
        input.iter()
            .map(|&pos| (pos as isize - rounded(mean) as isize).abs() as usize)
            .map(|distance| (distance *(distance+1))/2)
            .sum()
    }

    //Interestingly, floor, ceil or nearest integer rounding don't work in
    //every situation (as shown by the test set). Therefore, we try both floor
    //and ceil to find the correct solution.
    min( fuel_with_rounding(&f64::ceil,  mean, input.clone())
       , fuel_with_rounding(&f64::floor, mean, input))
}

pub fn p8_1(input: Vec<Vec<String>>)-> usize {
    input.concat().iter()
        .map(|str| str.len())
        .filter(|str_len| [2,3,4,7].contains(str_len))
        .count()
}

pub fn solve_one(input: &(Vec <String>, Vec<String>)) -> usize {
    let (defs, number) = input;
    let one = defs.iter()
        .filter(|str| str.len() == 2)
        .collect::<Vec<&String>>()[0];
    let seven = defs.iter()
        .filter(|str| str.len() == 3)
        .collect::<Vec<&String>>()[0];
    let four = defs.iter()
        .filter(|str| str.len() == 4)
        .collect::<Vec<&String>>()[0];
    let eight = defs.iter()
        .filter(|str| str.len() == 7)
        .collect::<Vec<&String>>()[0];
    let nine = defs.iter()
        .filter(|str| str.len() == 6)
        .filter (|str| four.chars().all(|chr| str.contains(chr)))
        .collect::<Vec<&String>>()[0];
    let zero = defs.iter()
        .filter(|str| str.len() == 6)
        .filter (|&str| str != nine)
        .filter (|str| one.chars().all(|chr| str.contains(chr)))
        .collect::<Vec<&String>>()[0];
    let six = defs.iter()
        .filter(|str| str.len() == 6)
        .filter (|&str| str != nine && str != zero)
        .collect::<Vec<&String>>()[0];
    let three = defs.iter()
        .filter(|str| str.len() == 5)
        .filter (|&str| one.chars().all(|chr| str.contains(chr)))
        .collect::<Vec<&String>>()[0];
    let five = defs.iter()
        .filter(|str| str.len() == 5)
        .filter (|&str| four.chars()
                            .filter(|&chr| !one.contains(chr))
                            .all(|chr| str.contains(chr)))
        .collect::<Vec<&String>>()[0];
    let two = defs.iter()
        .filter(|str| str.len() == 5)
        .filter (|&str| str != three && str != five)
        .collect::<Vec<&String>>()[0];
    
    number.iter()
        .map(|digit| match digit {
            i if i == zero => 0,
            i if i == one => 1,
            i if i == two => 2,
            i if i == three => 3,
            i if i == four => 4,
            i if i == five => 5,
            i if i == six => 6,
            i if i == seven => 7,
            i if i == eight => 8,
            i if i == nine => 9,
            _ => 0
        }).fold(0, |acc, digit| acc*10 + digit)
}

pub fn p8_2(input: Vec<(Vec <String>, Vec<String>)>) -> usize {
    input.iter().map(solve_one).sum()
}

pub fn p9_1(input: Vec<Vec<usize>>) -> usize {
    let mut risk_level = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            let mut lowest = true;
            if x > 0
                { lowest = lowest && input[x-1][y] > input[x][y] }
            if x < input.len()-1
                { lowest = lowest && input[x+1][y] > input[x][y] }
            if y > 0
                { lowest = lowest && input[x][y-1] > input[x][y] }
            if y < input[x].len()-1
                { lowest = lowest && input[x][y+1] > input[x][y] }
            if lowest
                { risk_level += input[x][y]+1 }
        }
    };
    risk_level
}

fn find_lowest_points(input: &Vec<Vec<usize>>) -> Vec<Coord> {
    let mut lowest_points = Vec::new();
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            let coord = Coord {x:x, y:y};
            let is_lowest = neighbours(input, &coord)
                .iter()
                .map(|coord| input[coord.x][coord.y])
                .all(|height| height > input[x][y]);
            if is_lowest { lowest_points.push(coord) }
        }
    };
    lowest_points
}

fn neighbours(input: &Vec<Vec<usize>>, point: &Coord) -> Vec<Coord> {
    let mut neighbours = Vec::new();
    if point.x > 0
        { neighbours.push(Coord { x:point.x-1, y:point.y }) }
    if point.x < input.len()-1
        { neighbours.push(Coord { x:point.x+1, y:point.y }) }
    if point.y > 0
        { neighbours.push(Coord { x:point.x, y:point.y-1 }) }
    if point.y < input[point.x].len()-1
        { neighbours.push(Coord { x:point.x, y:point.y+1 }) }
    neighbours
}

pub fn p9_2(input: Vec<Vec<usize>>) -> usize {
    let lowest_points = find_lowest_points(&input);
    let mut basin_sizes = Vec::new();
   
    for point in lowest_points {
        let mut basin_size = 0;
        let mut seen = Vec::from([point]);
        let mut visited: Vec<Coord> = Vec::new();
        while seen.len() > 0 {
            let cur = seen.pop().unwrap();
            visited.push(cur.clone());
            if input[cur.x][cur.y] < 9 {
                basin_size += 1;
                let unvisited_neighbours = neighbours(&input, &cur)
                    .into_iter()
                    .filter(|neighbour| !visited.contains(neighbour))
                    .filter(|neighbour| !seen.contains(neighbour))
                    .collect::<Vec<Coord>>();
                seen.append(&mut unvisited_neighbours.clone())
            }
        }
        basin_sizes.push(basin_size);
    }
    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes[..3].iter()
        .fold(1, |acc, size| acc*size)
}

//--------------------------day10--------------------------
use crate::parse::Delimiter;
use crate::parse::DelimiterType;

fn error_scoring (d_type: &DelimiterType) -> usize {
    match d_type {
        DelimiterType::Parens => 3,
        DelimiterType::Bracket => 57,
        DelimiterType::Brace => 1197,
        DelimiterType::Angle => 25137
    }
}

fn autocomplete_scoring (unclosed_scopes: Vec<&DelimiterType>) -> usize {
    unclosed_scopes.iter()
        .rev()
        .fold(0, |score, delimiter| {
            let pts = match delimiter {
                DelimiterType::Parens => 1,
                DelimiterType::Bracket => 2,
                DelimiterType::Brace => 3,
                DelimiterType::Angle => 4
            };
            score * 5 + pts
        })
}

pub struct ParsingResult<'a> {
    active_scopes: Vec<&'a DelimiterType>,
    first_faulty: Option<&'a DelimiterType>,
}

fn parse(input: &Vec<Delimiter>) -> ParsingResult {
    let mut active_scopes: Vec<&DelimiterType> = Vec::new();
    let mut first_faulty: Option<&DelimiterType> = None;
    for delimiter in input {
        match delimiter {
            Delimiter::Open(d_type) =>
                active_scopes.push(d_type),
            Delimiter::Close(d_type) => {
                let last_opened = active_scopes.pop();
                if last_opened != Some(d_type) {
                    first_faulty = Some(d_type);
                    break;
                };
            }
        }
    };
    ParsingResult {
        active_scopes: active_scopes,
        first_faulty: first_faulty
    }
}

pub fn p10_1(input: Vec<Vec<Delimiter>>) -> usize {
    input.iter()
        .map(|line| parse(line).first_faulty)
        .filter(|&opt| opt != None)
        .map(|opt| opt.unwrap())
        .map(error_scoring)
        .sum()
}

pub fn p10_2(input: Vec<Vec<Delimiter>>) -> usize {
    use itertools::Itertools;
    let sorted_scores = input.iter()
        .map(|line| parse(line))
        .filter(|result| result.first_faulty == None)
        .map(|result| autocomplete_scoring(result.active_scopes))
        .sorted()
        .collect::<Vec<usize>>();
    sorted_scores[sorted_scores.len()/2]
}

//--------------------------day11--------------------------
fn neighbours_8(octopi: &Vec<Vec<usize>>, point: Coord) -> Vec<Coord> {
    use std::cmp::{min, max};
    let mut neighbours = Vec::new();
    for x in max(0, point.x as isize-1) as usize 
           ..min(octopi.len(), point.x+2) {
        for y in max(0, point.y as isize-1) as usize
               ..min(octopi[x].len(), point.y+2) {
            neighbours.push( Coord {x:x, y:y})
        }
    }
    neighbours
}

fn powerup_all(
    octopi: &mut Vec<Vec<usize>>,
    will_flash: &mut Vec<Coord>
    ) -> () {
    for x in 0..octopi.len() {
        for y in 0..octopi[x].len() {
            octopi[x][y]+=1;
            if octopi[x][y] > 9 {
                will_flash.push( Coord {x:x, y:y}) 
            }
        }
    }
}

fn flash(
    octopi: &mut Vec<Vec<usize>>,
    flashing_octopus: Coord,
    will_flash: &mut Vec<Coord>,
    flashed: &mut Vec<Coord>
    ) -> () {
    neighbours_8(&octopi, flashing_octopus).iter()
        .for_each(|neighbour| {
            octopi[neighbour.x][neighbour.y] +=1;
            if octopi[neighbour.x][neighbour.y] > 9 &&
               !will_flash.contains(neighbour) &&
               !flashed.contains(neighbour) {
                will_flash.push(neighbour.clone()) 
            }
        })
}

fn zero(
    octopi: &mut Vec<Vec<usize>>,
    flashed: &Vec<Coord>
    ) -> () {
    flashed.iter()
        .for_each(|octopus| {
            octopi[octopus.x][octopus.y] = 0
        })
}

fn turn(octopi: &mut Vec<Vec<usize>>) {
    let mut will_flash: Vec<Coord> = Vec::new();
    let mut flashed: Vec<Coord> = Vec::new();
    powerup_all(octopi, &mut will_flash);
    while will_flash.len() > 0 {
        let flashing_octopus = will_flash.pop().unwrap();
        flashed.push(flashing_octopus.clone());
        flash( octopi
             , flashing_octopus
             , &mut will_flash
             , &mut flashed)
    };
    zero(octopi, &flashed);
}

fn count_flashes(octopi: &Vec<Vec<usize>>) -> usize {
    octopi.concat().iter().fold(0, |acc, &last| if last == 0 {acc+1} else {acc})
}

pub fn p11_1(input: Vec<Vec<usize>>, nb_turns: usize) -> usize {
    let mut octopi = input.clone();
    let mut flash_count = 0;
    for _turn in 0..nb_turns {
        turn(&mut octopi);
        flash_count += count_flashes(&octopi);
    }
    flash_count
}

fn converged(octopi: &Vec<Vec<usize>>) -> bool {
    let first = octopi[0][0];
    octopi.concat().iter().fold(true, |acc, &last| acc && last == first)
}

pub fn p11_2(input: Vec<Vec<usize>>) -> usize {
    let mut octopi = input.clone();
    let mut nb_turns = 0;
    while !converged(&octopi) {
        nb_turns += 1;
        turn(&mut octopi);
    }
    nb_turns
}
