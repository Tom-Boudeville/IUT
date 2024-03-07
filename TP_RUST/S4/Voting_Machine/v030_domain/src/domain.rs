use std::collections::BTreeSet as Set;
use std::collections::BTreeMap as Map;
use std::hint::black_box;
use clap::builder::Str;

#[derive(Debug)]
#[derive(Ord, PartialOrd, PartialEq, Clone)]
#[derive(Eq)]
pub struct Voter(pub String);
#[derive(Ord, PartialOrd, PartialEq, Debug, Clone)]
#[derive(Eq)]
pub struct Candidate(pub String);
pub struct Score(pub usize);
pub struct AttendanceSheet(pub Set<Voter>);

pub struct ScoreBoard{
    pub scores: Map<Candidate, Score>,
    pub blank_score: Score,
    pub invalid_score: Score,
}
#[derive(Clone)]
pub struct BallotPaper {
    pub voter: Voter,
    pub candidate: Option<Candidate>,
}

#[derive(PartialEq, Debug)]
pub enum VoteOutcome{
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}

pub struct VotingMachine{
    voters: AttendanceSheet,
    scoreboard: ScoreBoard,
}

impl ScoreBoard{
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let mut scores = Map::new();
        for candidate in candidates {
            scores.insert(candidate, Score(0));
        }

        let blank_score = Score(0);
        let invalid_score = Score(0);

        ScoreBoard {
            scores,
            blank_score,
            invalid_score,
        }
    }
}

impl VotingMachine{
    pub fn new(votersSet: Set<Voter>, candidates: Vec<Candidate>) -> Self {

        let mut votersSett = Set::new();

        for voter in &votersSet {
            votersSett.insert(voter);
        }
        let mut voters = AttendanceSheet(votersSet);

        VotingMachine {
            voters,
            scoreboard : ScoreBoard::new(candidates)
        }
    }

    pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome {
        if self.voters.0.contains(&ballot_paper.voter) {
            println!("Le votant {:?} a déjà voté", ballot_paper.voter);
            return VoteOutcome::HasAlreadyVoted(ballot_paper.voter);
        }
    
        if ballot_paper.candidate.is_none() {
            println!("Vote blanc");
            return VoteOutcome::BlankVote(ballot_paper.voter);
        }
    
        let candidate = ballot_paper.candidate.unwrap();
        if self.scoreboard.scores.contains_key(&candidate) {
            self.voters.0.insert(ballot_paper.voter.clone());
            println!("{:?} a voté pour {:?}", ballot_paper.voter, candidate);
            return VoteOutcome::AcceptedVote(ballot_paper.voter, candidate);
        }
    
        println!("Vote invalide");
        return VoteOutcome::InvalidVote(ballot_paper.voter);
    }
       
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet as Set;
    use crate::domain::{BallotPaper, Candidate, VoteOutcome, Voter, VotingMachine};

    #[test]
    fn itWorks(){
        assert_eq!(1+1,2);
    }

    #[test]
    fn test1(){
        let mut voterSet = Set::new();
        let mut votingMachine = VotingMachine::new(voterSet, vec![Candidate("matia".parse().unwrap())]);
        let ballotPaper = BallotPaper{
            voter: Voter("Lucas".parse().unwrap()),
            candidate: Option::from(Candidate("matia".parse().unwrap())),
        };
        assert_eq!(votingMachine.vote(ballotPaper.clone()), VoteOutcome::AcceptedVote(ballotPaper.voter.clone(), Candidate("matia".parse().unwrap())));

    }
    #[test]
    fn test2(){
        let mut voterSet = Set::new();
        voterSet.insert(Voter("Lucas".parse().unwrap()));
        let mut votingMachine = VotingMachine::new(voterSet, vec![Candidate("matia".parse().unwrap())]);
        let ballotPaper = BallotPaper{
            voter: Voter("Lucas".parse().unwrap()),
            candidate: Option::from(Candidate("matia".parse().unwrap())),
        };
        assert_eq!(votingMachine.vote(ballotPaper.clone()), VoteOutcome::HasAlreadyVoted(ballotPaper.voter.clone()));
    }
    #[test]
    fn test3(){
        let mut voterSet = Set::new();
        let mut votingMachine = VotingMachine::new(voterSet, vec![Candidate("matia".parse().unwrap())]);
        let ballotPaper = BallotPaper{
            voter: Voter("Lucas".parse().unwrap()),
            candidate: None,
        };
        assert_eq!(votingMachine.vote(ballotPaper.clone()), VoteOutcome::BlankVote(ballotPaper.voter.clone()));
    }
    #[test]
    fn test4(){
        let mut voterSet = Set::new();
        let mut votingMachine = VotingMachine::new(voterSet, vec![Candidate("matia".parse().unwrap())]);
        let ballotPaper = BallotPaper{
            voter: Voter("Lucas".parse().unwrap()),
            candidate: Option::from(Candidate("bibi".parse().unwrap())),
        };
        assert_eq!(votingMachine.vote(ballotPaper.clone()), VoteOutcome::InvalidVote(ballotPaper.voter.clone()));
    }
}