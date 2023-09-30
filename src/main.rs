use rand::Rng;

#[derive(Debug, Clone)]
struct Game {
    pub middle_levels_count: usize,
    pub actions: Vec<isize>,
    pub probability_of_go_from_center: f64,
}

#[derive(Debug)]
enum Winner {
    Leader,
    Follower,
}

impl Winner {
    fn did_leader_win(&self) -> bool {
        matches!(self, Self::Leader)
    }
}

impl Game {
    fn play(&self) -> Winner {
        let mut current_level = 0;

        for action in self.actions.iter() {
            for _ in 0..action.abs() {
                self.handle_action(&mut current_level, action.is_positive());

                if current_level == self.middle_levels_count + 1 {
                    return Winner::Follower;
                }
            }
        }

        Winner::Leader
    }

    fn handle_action(&self, current_level: &mut usize, agree: bool) {
        if *current_level == 0 {
            *current_level += 1;
            return;
        }

        let go_from_center = self.go_from_center();
        let go_from_center = if agree {
            go_from_center
        } else {
            !go_from_center
        };

        if go_from_center {
            *current_level += 1;
        } else if *current_level > 1 {
            *current_level -= 1;
        }
    }

    fn go_from_center(&self) -> bool {
        let rand_value = rand::thread_rng().sample(rand::distributions::Uniform::new(0.0, 1.0));
        rand_value <= self.probability_of_go_from_center
    }
}

fn main() {
    let _ = rand::thread_rng()
        .sample_iter(rand::distributions::Uniform::new(0.0, 1.0))
        .take(100000)
        .collect::<Vec<_>>();

    let actions = vec![25];
    let middle_levels_count = 3;
    let probability_of_go_from_center = 0.1;

    let game = Game {
        middle_levels_count,
        actions,
        probability_of_go_from_center,
    };

    for n in 10_000..10_001usize {
        let mut leader_wins_first_half = 0;
        let mut leader_wins_second_half = 0;

        for i in 0..n {
            if !game.play().did_leader_win() {
                continue;
            }

            if i < n / 2 {
                leader_wins_first_half += 1;
            } else {
                leader_wins_second_half += 1;
            }
        }

        let first_half_statistic = leader_wins_first_half as f64 / (n / 2) as f64;
        let second_half_statistic = leader_wins_second_half as f64 / (n / 2) as f64;
        let whole_statistic = (leader_wins_first_half + leader_wins_second_half) as f64 / n as f64;
        dbg!(
            first_half_statistic,
            second_half_statistic,
            whole_statistic,
            ""
        );

        let factor = 100.0;
        let rounded_first_half_statistic = (first_half_statistic * factor).round();
        let rounded_second_half_statistic = (second_half_statistic * factor).round();
        let rounded_whole_statistic = (whole_statistic * factor).round();

        if rounded_first_half_statistic == rounded_second_half_statistic
            && rounded_second_half_statistic == rounded_whole_statistic
        {
            println!(
                "leader wins {}% of games",
                (first_half_statistic * 100.0).round()
            );
            println!(
                "follower wins {}% of games",
                100.0 - (first_half_statistic * 100.0).round()
            );
            println!("n: {}", n);
            break;
        }
    }
}
