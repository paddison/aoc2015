use std::ops::{Deref, DerefMut};

struct Reindeer {
    _name: &'static str,
    speed: usize,
    uptime: usize,
    downtime: usize,
}

impl Reindeer {
    fn new(name: &'static str, speed: usize, uptime: usize, downtime: usize) -> Self {
        Self { _name: name, speed, uptime, downtime }
    }

    fn distance_after_n_secs(&self, mut secs: isize) -> usize {
        // iteratively
        let mut dist = 0;
        while secs > 0 {
            let flight_time = std::cmp::min(secs, self.uptime as isize);
            dist += self.speed * flight_time as usize;
            secs -= flight_time;
            secs -= self.downtime as isize;
        }

        dist
    }
}

// Wrapper to avoid changing reindeer struct
struct ReindeerData {
    dist: usize,
    score: usize,
    reindeer: Reindeer,
    time_to_switch: usize,
    is_up: bool,
}

impl ReindeerData {
    fn new(reindeer: Reindeer) -> Self {
        let time_to_switch = reindeer.uptime;
        Self { dist: 0, score: 0, reindeer, time_to_switch, is_up: true }
    }

    fn update(&mut self) {
        if self.is_up {
            self.dist += self.speed;
        }
        self.time_to_switch -= 1;
        if self.time_to_switch == 0 {
            self.is_up = !self.is_up;
            self.time_to_switch = match self.is_up {
                true => self.uptime,
                false => self.downtime,
            }
        }
    }
}

impl Deref for ReindeerData {
    type Target = Reindeer;

    fn deref(&self) -> &Self::Target {
        &self.reindeer
    }
}

impl DerefMut for ReindeerData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.reindeer
    }
}

fn parse(input: &'static str) -> Vec<Reindeer> {
    input.split('\n')
         .map(|line| line.split_whitespace().collect::<Vec<&str>>())
         .map(|data| Reindeer::new(
            data[0], 
            data[3].parse::<usize>().unwrap(), 
            data[6].parse::<usize>().unwrap(), 
            data[13].parse::<usize>().unwrap()))
         .collect()
}

fn update_score(reindeer_data: &mut[ReindeerData]) {
    for deer in reindeer_data.iter_mut() {
        deer.update()
    }

    let highest_dist = reindeer_data.iter_mut()
                                    .max_by(|a, b| a.dist.cmp(&b.dist))
                                    .unwrap()
                                    .dist; 
    reindeer_data.iter_mut()
                 .filter(|deer| deer.dist == highest_dist)
                 .for_each(|deer| deer.score += 1)
}

fn determine_winning_score(mut reindeer_data: Vec<ReindeerData>, secs: usize) -> usize {
    for _ in 0..secs {
        update_score(&mut reindeer_data)
    }

    reindeer_data.iter().max_by(|a, b| a.score.cmp(&b.score)).unwrap().score
}

pub fn get_solution_1() -> usize {
    let reindeers = parse(include_str!("../data/d14.txt"));
    let mut dists = Vec::new();

    for reindeer in reindeers {
        dists.push(reindeer.distance_after_n_secs(2503));
    } 

    *dists.iter().max().unwrap()
}

pub fn get_solution_2() -> usize {
    let reindeers = parse(include_str!("../data/d14.txt"));
    let mut reindeer_data = Vec::new();
    for deer in reindeers {
        reindeer_data.push(ReindeerData::new(deer));
    }

    determine_winning_score(reindeer_data, 2503)
}

#[test]
fn test_dist_after_n_secs() {
    let reindeers = parse(include_str!("../data/d14_test.txt"));
    let mut dists = Vec::new(); 
    for reindeer in reindeers {
        dists.push(reindeer.distance_after_n_secs(1000));
    }

    assert_eq!(*dists.iter().max().unwrap(), 1120);
}

#[test]
fn test_solution_with_test_data() {
    let reindeers = parse(include_str!("../data/d14_test.txt"));
    let mut reindeer_data = Vec::new();
    for deer in reindeers {
        reindeer_data.push(ReindeerData::new(deer));
    }

    assert_eq!(determine_winning_score(reindeer_data, 1000), 689)
}