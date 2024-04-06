use crate::solver::{Base, Chain};

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum FactoringError {
    #[error("Attempted to insert base {1:?} on full chain {0:?}")]
    AttemptedInsertionOnFullChain(Chain, Base),
    #[error("Attempted to call finalise on incomplete chain {0:?}")]
    FinaliseCalledOnIncompleteChain(Chain),
    #[error("Expected ambigous base, found {0}")]
    LetterNotAmbiguousBase(char),
}

pub type FResult<T> = Result<T, FactoringError>;
