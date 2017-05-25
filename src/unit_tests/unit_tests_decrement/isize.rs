use std;
use super::super::Decrement;

//
//Valid decrement operations
//
#[test]
fn isize_pre_decrement_works_from_max()
{
    let mut result = std::isize::MAX;
    let delta = 1;
    assert!(result._dec() == std::isize::MAX - delta);
    assert!(result == std::isize::MAX - delta);
}

#[test]
fn isize_pre_decrement_by_2_works_from_max()
{
    let mut result = std::isize::MAX;
    let delta = 2;
    assert!(result._dec_by(delta) == std::isize::MAX - delta);
    assert!(result == std::isize::MAX - delta);
}

#[test]
fn isize_post_decrement_works_from_max()
{
    let mut result = std::isize::MAX;
    let delta = 1;
    assert!(result.dec_() == std::isize::MAX);
    assert!(result == std::isize::MAX - delta);
}

#[test]
fn isize_post_decrement_by_3_works_from_max()
{
    let mut result = std::isize::MAX;
    let delta = 3;
    assert!(result.dec_by_(delta) == std::isize::MAX);
    assert!(result == std::isize::MAX - delta);
}

//
//Invalid decrement operations
//
#[test]
#[should_panic(expected = "overflow")]
fn isize_pre_decrement_panics_from_min()
{
    let mut result = std::isize::MIN;
    assert!(result._dec() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn isize_pre_decrement_by_4_panics_from_min()
{
    let mut result = std::isize::MIN;
    let delta = 4;
    assert!(result._dec_by(delta) == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn isize_post_decrement_panics_from_min()
{
    let mut result = std::isize::MIN;
    assert!(result.dec_() == result);
}

#[test]
#[should_panic(expected = "overflow")]
fn isize_post_decrement_by_5_panics_from_min()
{
    let mut result = std::isize::MIN;
    let delta = 5;
    assert!(result.dec_by_(delta) == result);
}

