extern crate games_microservice;
use games_microservice::games::rps::Weapons;

#[test]
fn test_comparision() {
    assert_eq!(Weapons::Rock, Weapons::Rock);
    assert!(Weapons::Rock > Weapons::Scissors);
    assert!(Weapons::Rock < Weapons::Paper);
    assert_eq!(Weapons::Paper, Weapons::Paper);
    assert!(Weapons::Paper > Weapons::Rock);
    assert!(Weapons::Paper < Weapons::Scissors);
    assert_eq!(Weapons::Scissors, Weapons::Scissors);
    assert!(Weapons::Scissors > Weapons::Paper);
    assert!(Weapons::Scissors < Weapons::Rock);
}
