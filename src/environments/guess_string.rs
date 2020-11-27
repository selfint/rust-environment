use rand::{thread_rng, Rng, distributions::Alphanumeric, seq::{IteratorRandom, SliceRandom}};
use rand::prelude::ThreadRng;


#[derive(Clone)]
pub struct Agent {
    pub guess: String
}

impl Agent {
    fn new(target_len: usize) -> Agent {
        let guess = thread_rng().sample_iter(&Alphanumeric).take(target_len).collect();
        Agent { guess }
    }

    fn act(&self) -> String {
        self.guess.clone()
    }

    fn crossover(&self, other: &Agent) -> Agent {
        let mut rng = thread_rng();
        let options = self.guess.chars().into_iter().zip(other.guess.chars().into_iter());
        assert_eq!(options.clone().count(), other.guess.chars().count());
        assert_eq!(options.clone().count(), self.guess.chars().count());
        let guess: String = options.map(|(a, b)| {
            [a, b].choose(&mut rng).unwrap().clone()
        }).collect();

        assert_eq!(guess.chars().count(), self.guess.chars().count());

        Agent {guess}
    }

    fn mutate(&mut self) {
        let new_index = thread_rng().gen_range(0, self.guess.len());
        let new_char: char = thread_rng().sample_iter(&Alphanumeric).take(1).collect::<Vec<char>>()[0];
        self.guess = self.guess.chars().into_iter().enumerate().map(
            |(index, c)| if index == new_index { new_char } else { c }
        ).collect();
    }
}

pub struct GuessStringEnvironment {
    agents: Vec<Agent>,
    target: String,
}

impl GuessStringEnvironment {
    fn new(total_guesses: usize, target: String) -> GuessStringEnvironment {
        let agents = (0..total_guesses).map(|_| {
            Agent::new(target.chars().count())
        }).collect();
        GuessStringEnvironment {
            agents,
            target,
        }
    }

    fn get_champion(&self, agent_scores: &Vec<u16>) -> &Agent {
        let max_score = agent_scores.iter().max().unwrap();
        let champion_index = agent_scores.iter().position(|score| score == max_score).unwrap();
        &self.agents[champion_index]
    }

    fn run(&mut self, generations: usize) -> &Agent {
        assert!(generations > 0);

        let mut rng = thread_rng();
        let mut agent_scores = self.score_agents();
        for _ in 0..(generations - 1) {
            self.agents = self.new_generation(&mut rng, &agent_scores);
            agent_scores = self.score_agents();
        }

        self.get_champion(&agent_scores)
    }

    fn new_generation(&mut self, mut rng: &mut ThreadRng, agent_scores: &Vec<u16>) -> Vec<Agent> {
        let mut new_agents = vec![];
        let agent_weights: Vec<(&Agent, &u16)> = self.agents.iter().zip(agent_scores.iter()).collect();
        for _ in 0..(self.agents.len() - 1) {
            let (parent_a, parent_b) = GuessStringEnvironment::get_random_parents(&mut rng, &agent_weights);
            let mut child = parent_a.crossover(parent_b);
            if rng.gen_range(0.0, 1.0) < 0.01 {
                child.mutate();
            }
            new_agents.push(child);
        }
        new_agents.push(self.get_champion(agent_scores).clone());
        new_agents
    }

    fn get_random_parents<'a>(rng: &mut ThreadRng, agent_weights: &'a Vec<(&Agent, &u16)>) -> (&'a Agent, &'a Agent) {
        let parent_a = agent_weights.choose_weighted(rng, |item| item.1).unwrap().0;
        let parent_b = agent_weights.choose_weighted(rng, |item| item.1).unwrap().0;
        (parent_a, parent_b)
    }

    fn score_agents(&self) -> Vec<u16> {
        self.agents.iter().map(|a| self.score_action(a.act())).collect()
    }

    fn score_action(&self, agent_action: String) -> u16 {
        self.target.chars().into_iter().zip(
            agent_action.chars().into_iter()).map(|(real, guess)|
                if real == guess { 1 } else { 0 }
        ).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::environments::guess_string::GuessStringEnvironment;

    #[test]
    fn test_constructor() {
        let target = "H3ll0RustEnvir0nm3nt";
        let gse = GuessStringEnvironment::new(
            10,
            String::from(target)
        );
        for agent in gse.agents {
            assert_eq!(20, agent.guess.chars().count());
        }
    }

    #[test]
    fn test_run() {
        let target = "H3ll0RustEnvir0nm3nt";
        let mut gse = GuessStringEnvironment::new(
            250,
            String::from(target)
        );
        let champion = gse.run(2_000);
        assert_eq!(target, champion.guess);
    }
}
