use crate::TurnDirection;

use super::{
    syntax::{
        DistanceNounPhrase, LeftRightTurnPrepPhrase, StreetNounPhrase, TurnDirectionNoun,
        TurnDirectionNounPhrase, TurnableNounPhrase,
    },
    Sentence,
};

/// Navigation command where distances in terms of the context of the player's
/// current pose in the game map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CtxCommand {
    Forward(CtxCommandDistance),
    Rotate(TurnDirection),
}

/// Navigation command to move forward.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum CtxCommandDistance {
    ThisOrNextStreet(Option<TurnDirection>),
    NthStreet(usize, Option<TurnDirection>),
}

impl From<Sentence> for Vec<CtxCommand> {
    fn from(s: Sentence) -> Self {
        match s {
            Sentence::EstáTurnPp(pp) => {
                let rot = pp.into();
                vec![CtxCommand::Rotate(rot)]
            }
            Sentence::EstáEnNpPp(np, pp) => {
                let rot = pp.into();
                let dist = CtxCommandDistance::from_turnable_np(np, rot);
                vec![CtxCommand::Forward(dist), CtxCommand::Rotate(rot)]
            }
            Sentence::GiraPp(pp) => {
                let rot = pp.into();
                let dist = CtxCommandDistance::ThisOrNextStreet(Some(rot));
                vec![CtxCommand::Forward(dist), CtxCommand::Rotate(rot)]
            }
            Sentence::GiraNpPp(np, pp) => {
                let rot = pp.into();
                let dist = CtxCommandDistance::from_turnable_np(np, rot);
                vec![CtxCommand::Forward(dist), CtxCommand::Rotate(rot)]
            }
            Sentence::TomaNpPp(np, pp) => {
                let rot = pp.into();
                let dist = CtxCommandDistance::from_turnable_np(np, rot);
                vec![CtxCommand::Forward(dist), CtxCommand::Rotate(rot)]
            }
            Sentence::ContinúaNpNp(_, pp) => {
                let dist = pp.into();
                vec![CtxCommand::Forward(dist)]
            }
            Sentence::ContinúaNpHastaNp(_, pp) => {
                let dist = pp.into();
                vec![CtxCommand::Forward(dist)]
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

impl From<DistanceNounPhrase> for CtxCommandDistance {
    fn from(np: DistanceNounPhrase) -> Self {
        match np {
            DistanceNounPhrase::NQuadras(n) => Self::NthStreet(n, None),
        }
    }
}

impl From<StreetNounPhrase> for CtxCommandDistance {
    fn from(np: StreetNounPhrase) -> Self {
        match np {
            StreetNounPhrase::LaCalle => Self::ThisOrNextStreet(None),
            StreetNounPhrase::LaCalleOrd(ord) => Self::NthStreet(ord.value(), None),
        }
    }
}

impl CtxCommandDistance {
    fn from_turnable_np(t_np: TurnableNounPhrase, dir: TurnDirection) -> Self {
        match t_np {
            TurnableNounPhrase::Street(StreetNounPhrase::LaCalle) => {
                CtxCommandDistance::ThisOrNextStreet(Some(dir))
            }
            TurnableNounPhrase::Street(StreetNounPhrase::LaCalleOrd(ord)) => {
                CtxCommandDistance::NthStreet(ord.value(), Some(dir))
            }
        }
    }
}
