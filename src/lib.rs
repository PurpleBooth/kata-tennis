use std::collections::HashMap;
use std::fmt::Error;
use std::fmt::Formatter;

const PLAYER_1_ID: bool = false;
const PLAYER_2_ID: bool = true;

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct Score(u8);

impl Score {
    fn new(score: u8) -> Result<Score, String> {
        score_to_call(score)
            .map(|_| Score(score))
            .ok_or(format!("Score {} is not a valid tennis score", score))
    }

    fn add(&self, point: Point) -> Result<Score, String> {
        Score::new(self.0 + 1)
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", score_to_call(self.0).unwrap())
    }
}

trait Call {
    fn to_string(&self) -> String;
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.to_string())
    }
}

struct CallName(String);

impl Call for CallName {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

struct CallNumber(u8);

impl Call for CallNumber {
    fn to_string(&self) -> String {
        format!("{}", self.0).to_string()
    }
}

struct GameScore<'gs> {
    scores: HashMap<&'gs Player, Score>,
}

impl<'gs> GameScore<'gs> {
    fn new() -> GameScore<'gs> {
        let mut game = GameScore {
            scores: HashMap::new(),
        };

        game.scores
            .insert(Player::player_1(), Score::new(0).unwrap());
        game.scores
            .insert(Player::player_2(), Score::new(0).unwrap());

        game
    }

    fn scored(&self, point: Point<'gs>) -> GameScore<'gs> {
        let mut game = GameScore::new();

        let old_scores: &HashMap<&Player, Score> = &self.scores;
        let mut new_scores: HashMap<&Player, Score> =
            old_scores.into_iter().map(|(k, v)| (*k, *v)).collect();

        let scoring_player = point.player;
        let new_score = old_scores.get(scoring_player).unwrap().add(point).unwrap();
        new_scores.insert(scoring_player, new_score);

        game.scores = new_scores;

        game
    }
}

impl<'gs> std::fmt::Display for GameScore<'gs> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}-{}",
            self.scores.get(Player::player_1()).unwrap(),
            self.scores.get(Player::player_2()).unwrap()
        )
    }
}

pub struct Point<'p> {
    player: &'p Player,
}

impl<'p> Point<'p> {
    fn new(player: &'p Player) -> Point<'p> {
        Point { player }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct Player {
    id: bool,
}

impl Player {
    fn player_1<'p>() -> &'p Player {
        return &Player { id: PLAYER_1_ID };
    }
    fn player_2<'p>() -> &'p Player {
        return &Player { id: PLAYER_2_ID };
    }
}

fn score_to_call(score: u8) -> Option<Box<Call>> {
    return match score {
        0 => Some(Box::new(CallName("love".to_string()))),
        1 => Some(Box::new(CallNumber(15))),
        2 => Some(Box::new(CallNumber(30))),
        3 => Some(Box::new(CallNumber(40))),
        4 => Some(Box::new(CallName("game".to_string()))),
        _ => None,
    };
}

pub fn score_game(points: Vec<Point>) -> String {
    let mut game = GameScore::new();

    for point in points {
        game = game.scored(point)
    }

    return game.to_string();
}

#[cfg(test)]
mod test {
    use GameScore;
    use Player;
    use PLAYER_1_ID;
    use PLAYER_2_ID;
    use Point;
    use Score;
    use score_game;

    #[test]
    fn score_should_be_love_for_0() {
        assert_eq!(format!("{}", Score::new(0).unwrap()), "love")
    }

    #[test]
    fn score_should_be_15_for_1() {
        assert_eq!(format!("{}", Score::new(1).unwrap()), "15")
    }

    #[test]
    fn score_should_be_30_for_2() {
        assert_eq!(format!("{}", Score::new(2).unwrap()), "30")
    }

    #[test]
    fn score_should_be_40_for_3() {
        assert_eq!(format!("{}", Score::new(3).unwrap()), "40")
    }

    #[test]
    fn score_should_be_game_for_4() {
        assert_eq!(format!("{}", Score::new(4).unwrap()), "game")
    }

    #[test]
    fn score_should_never_be_more_than_4() {
        assert_eq!(Score::new(5).is_err(), true)
    }

    #[test]
    fn score_should_of_0_plus_one_point_is_15() {
        assert_eq!(
            format!(
                "{}",
                Score::new(0)
                    .unwrap()
                    .add(Point {
                        player: &Player::player_1(),
                    })
                    .unwrap()
            ),
            "15"
        )
    }

    #[test]
    fn score_should_of_1_plus_one_point_is_30() {
        assert_eq!(
            format!(
                "{}",
                Score::new(1)
                    .unwrap()
                    .add(Point {
                        player: &Player::player_1(),
                    })
                    .unwrap()
            ),
            "30"
        )
    }

    #[test]
    fn score_should_of_2_plus_one_point_is_40() {
        assert_eq!(
            format!(
                "{}",
                Score::new(2)
                    .unwrap()
                    .add(Point {
                        player: Player::player_1(),
                    })
                    .unwrap()
            ),
            "40"
        )
    }

    #[test]
    fn score_should_of_3_plus_one_point_is_game() {
        assert_eq!(
            format!(
                "{}",
                Score::new(3)
                    .unwrap()
                    .add(Point {
                        player: Player::player_1(),
                    })
                    .unwrap()
            ),
            "game"
        )
    }

    #[test]
    fn score_should_of_4_plus_one_point_errors() {
        assert!(
            Score::new(4)
                .unwrap()
                .add(Point {
                    player: &Player::player_1(),
                })
                .is_err()
        )
    }

    #[test]
    fn player_should_have_a_player_1() {
        assert_eq!(Player::player_1(), &Player { id: PLAYER_1_ID })
    }

    #[test]
    fn player_should_have_a_player_2() {
        assert_eq!(Player::player_2(), &Player { id: PLAYER_2_ID })
    }

    #[test]
    fn game_score_should_be_love_love_for_no_scores() {
        assert_eq!(format!("{}", GameScore::new()), "love-love")
    }

    #[test]
    fn game_score_should_allow_adding_one_point_to_player_1() {
        let game_score = GameScore::new().scored(Point::new(Player::player_1()));

        assert_eq!(format!("{}", game_score), "15-love")
    }

    #[test]
    fn game_score_should_allow_adding_one_point_to_player_2() {
        let game_score = GameScore::new().scored(Point::new(Player::player_2()));

        assert_eq!(format!("{}", game_score), "love-15")
    }

    #[test]
    fn game_score_should_allow_adding_two_points_to_player_2() {
        let game_score = GameScore::new()
            .scored(Point::new(Player::player_2()))
            .scored(Point::new(Player::player_2()));

        assert_eq!(game_score.to_string(), "love-30")
    }

    #[test]
    fn score_game_should_be_love_love_for_no_scores() {
        assert_eq!(score_game(vec![]), "love-love")
    }

    #[test]
    fn score_game_should_be_love_15_for_a_single_player_2_point() {
        assert_eq!(score_game(vec![Point::new(Player::player_2())]), "love-15")
    }
}
