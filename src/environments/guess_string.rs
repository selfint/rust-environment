use rand::{thread_rng, Rng, distributions::Alphanumeric, seq::{IteratorRandom, SliceRandom}};
use rand::prelude::ThreadRng;


pub struct Agent {
    pub guess: String
}

impl Agent {
    fn new(target_len: usize) -> Agent {
        Agent {
            guess: thread_rng().sample_iter(&Alphanumeric).take(target_len).collect()
        }
    }

    fn act(&self) -> String {
        self.guess.clone()
    }

    fn crossover(&self, other: &Agent) -> Agent {
        let mut rng = thread_rng();
        let options = self.guess.chars().into_iter().zip(other.guess.chars().into_iter());
        let guess = options.map(|(a,b)| {
            [a, b].choose(&mut rng).unwrap().clone()
        }).collect();

        Agent {guess}
    }

    fn mutate(&mut self) {
        let index = thread_rng().gen_range(0, self.guess.len());
        let new_char: char = thread_rng().sample_iter(&Alphanumeric).take(1).collect::<Vec<char>>()[0];
        self.guess.insert(index, new_char);
    }
}

pub struct GuessStringEnvironment {
    agents: Vec<Agent>,
    target: String,
}

impl GuessStringEnvironment {
    fn new(total_guesses: usize, target: String) -> GuessStringEnvironment {
        let agents = (0..total_guesses).map(|_| {
            Agent::new(target.len())
        }).collect();
        GuessStringEnvironment {
            agents,
            target,
        }
    }

    fn run(&mut self, generations: usize) {
        let mut rng = thread_rng();
        for g in 0..generations {
            let agent_scores = self.score_agents();
            let new_agents = self.new_generation(&mut rng, agent_scores);

            self.agents = new_agents;
        }
    }

    fn new_generation(&mut self, mut rng: &mut ThreadRng, agent_scores: Vec<u16>) -> Vec<Agent> {
        let mut new_agents = vec![];
        let agent_weights: Vec<(&Agent, &u16)> = self.agents.iter().zip(agent_scores.iter()).collect();
        for i in 0..self.agents.len() {
            let parent_a = agent_weights.choose_weighted(&mut rng, |item| item.1).unwrap().0;
            let parent_b = agent_weights.choose_weighted(&mut rng, |item| item.1).unwrap().0;
            let mut child = parent_a.crossover(parent_b);
            if rng.gen_range(0.0, 1.0) < 0.5 {
                child.mutate();
            }
            new_agents.push(child);
        }
        new_agents
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
        let target = "Hello Rust Environment!";
        let gse = GuessStringEnvironment::new(
            10,
            String::from(target)
        );
        for agent in gse.agents {
            assert_eq!(agent.guess.len(), target.len());
        }
    }
}
