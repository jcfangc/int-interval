use std::vec;
use std::vec::Vec;

use super::*;

#[test]
fn one_two_iterates_owned_values() {
    assert_eq!(OneTwo::One(1).into_iter().collect::<Vec<_>>(), vec![1]);
    assert_eq!(
        OneTwo::Two(1, 2).into_iter().collect::<Vec<_>>(),
        vec![1, 2]
    );
}

#[test]
fn zero_one_two_iterates_owned_values() {
    assert_eq!(
        ZeroOneTwo::<i32>::Zero.into_iter().collect::<Vec<_>>(),
        vec![]
    );
    assert_eq!(ZeroOneTwo::One(1).into_iter().collect::<Vec<_>>(), vec![1]);
    assert_eq!(
        ZeroOneTwo::Two(1, 2).into_iter().collect::<Vec<_>>(),
        vec![1, 2]
    );
}

#[test]
fn supports_double_ended_iteration() {
    let mut iter = ZeroOneTwo::Two(1, 2).into_iter();

    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
}

#[test]
fn reports_exact_size() {
    let mut iter = OneTwo::Two(1, 2).into_iter();

    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.len(), 0);
}
