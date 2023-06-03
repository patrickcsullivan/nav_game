use crate::TurnDirection;

use super::{
    syntax::{
        DistanceNounPhrase, LeftRightTurnPrepPhrase, StreetNounPhrase, TurnDirectionNoun,
        TurnDirectionNounPhrase, TurnableNounPhrase,
    },
    Sentence,
};

/// Navigation command where distances are specified in real world terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealWorldCommand {
    Forward(RealWorldCommandDistance),
    Rotate(TurnDirection),
}

/// Navigation command to move forward.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum RealWorldCommandDistance {
    ThisOrNextStreet(Option<TurnDirection>),
    NthStreet(usize, Option<TurnDirection>),
}

impl From<Sentence> for Vec<RealWorldCommand> {
    fn from(s: Sentence) -> Self {
        match s {
            Sentence::EstáTurnPp(pp) => {
                let rot = pp.into();
                vec![RealWorldCommand::Rotate(rot)]
            }
            Sentence::EstáEnNpPp(np, pp) => {
                let rot = pp.into();
                let dist = RealWorldCommandDistance::from_turnable_np(np, rot);
                vec![
                    RealWorldCommand::Forward(dist),
                    RealWorldCommand::Rotate(rot),
                ]
            }
            Sentence::GiraPp(pp) => {
                let rot = pp.into();
                let dist = RealWorldCommandDistance::ThisOrNextStreet(Some(rot));
                vec![
                    RealWorldCommand::Forward(dist),
                    RealWorldCommand::Rotate(rot),
                ]
            }
            Sentence::GiraNpPp(np, pp) => {
                let rot = pp.into();
                let dist = RealWorldCommandDistance::from_turnable_np(np, rot);
                vec![
                    RealWorldCommand::Forward(dist),
                    RealWorldCommand::Rotate(rot),
                ]
            }
            Sentence::TomaNpPp(np, pp) => {
                let rot = pp.into();
                let dist = RealWorldCommandDistance::from_turnable_np(np, rot);
                vec![
                    RealWorldCommand::Forward(dist),
                    RealWorldCommand::Rotate(rot),
                ]
            }
            Sentence::ContinúaNpNp(_, pp) => {
                let dist = pp.into();
                vec![RealWorldCommand::Forward(dist)]
            }
            Sentence::ContinúaNpHastaNp(_, pp) => {
                let dist = pp.into();
                vec![RealWorldCommand::Forward(dist)]
            }
        }
    }
}

impl From<LeftRightTurnPrepPhrase> for TurnDirection {
    fn from(pp: LeftRightTurnPrepPhrase) -> Self {
        let np = pp.0;
        np.into()
    }
}

impl From<TurnDirectionNounPhrase> for TurnDirection {
    fn from(np: TurnDirectionNounPhrase) -> Self {
        let n = np.0;
        n.into()
    }
}

impl From<TurnDirectionNoun> for TurnDirection {
    fn from(n: TurnDirectionNoun) -> Self {
        match n {
            TurnDirectionNoun::Izquierda => Self::Left,
            TurnDirectionNoun::Derecha => Self::Right,
        }
    }
}

impl From<DistanceNounPhrase> for RealWorldCommandDistance {
    fn from(np: DistanceNounPhrase) -> Self {
        match np {
            DistanceNounPhrase::NQuadras(n) => Self::NthStreet(n, None),
        }
    }
}

impl From<StreetNounPhrase> for RealWorldCommandDistance {
    fn from(np: StreetNounPhrase) -> Self {
        match np {
            StreetNounPhrase::LaCalle => Self::ThisOrNextStreet(None),
            StreetNounPhrase::LaCalleOrd(ord) => Self::NthStreet(ord.value(), None),
        }
    }
}

impl RealWorldCommandDistance {
    fn from_turnable_np(t_np: TurnableNounPhrase, dir: TurnDirection) -> Self {
        match t_np {
            TurnableNounPhrase::Street(StreetNounPhrase::LaCalle) => {
                RealWorldCommandDistance::ThisOrNextStreet(Some(dir))
            }
            TurnableNounPhrase::Street(StreetNounPhrase::LaCalleOrd(ord)) => {
                RealWorldCommandDistance::NthStreet(ord.value(), Some(dir))
            }
        }
    }
}
