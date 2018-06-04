use ::{Color, Property, Tech};
use super::*;


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum ColorTest {
  Red,
  Blue,
  Green,
  Yellow,
}

impl Color for ColorTest {
  type Prop = usize;

  fn num_colors() -> usize {4}
}


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum TechTest {
  T1,
  T2,
  T3,
  T4,
  T5,
}

impl Tech for TechTest {
  fn init_size() -> usize {5}
}


#[test]
fn crystalmap_test_1() {
// tests add_mapping()
  let mut crystalmap = CrystalMap::<ColorTest, TechTest>::new();

  crystalmap.add_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();
  crystalmap.add_mapping(ColorTest::Green, 0, TechTest::T1).unwrap();
  crystalmap.add_mapping(ColorTest::Blue, 1, TechTest::T1).unwrap();
  crystalmap.add_mapping(ColorTest::Red, 0, TechTest::T2).unwrap();
  crystalmap.add_mapping(ColorTest::Yellow, 1, TechTest::T2).unwrap();
  crystalmap.add_mapping(ColorTest::Red, 2, TechTest::T3).unwrap();
  crystalmap.add_mapping(ColorTest::Green, 4, TechTest::T3).unwrap();
  crystalmap.add_mapping(ColorTest::Blue, 0, TechTest::T5).unwrap();
  crystalmap.add_mapping(ColorTest::Green, 4, TechTest::T5).unwrap();
  crystalmap.add_mapping(ColorTest::Green, 4, TechTest::T4).unwrap();

  // red    0 - T2
  // red    2 - T3
  //-----------------------
  // blue   0 - T1, T5
  // blue   1 - T1
  //-----------------------
  // green  0 - T1
  // green  4 - T3, T4, T5
  //-----------------------
  // yellow 1 - T2
  /////////////////////////

  // red    0 - T2
  assert_eq!(
    &**crystalmap.property_to_tech.get(&ColorTest::Red).unwrap().get(&0).unwrap(),
    &[TechTest::T2]
  );
  // red    2 - T3
  assert_eq!(
    &**crystalmap.property_to_tech.get(&ColorTest::Red).unwrap().get(&2).unwrap(),
    &[TechTest::T3]
  );
  // blue   0 - T1, T5
  assert_eq!(
    &**crystalmap.property_to_tech.get(&ColorTest::Blue).unwrap().get(&0).unwrap(),
    &[TechTest::T1, TechTest::T5]
  );
  // blue   1 - T1
  assert_eq!(
    &**crystalmap.property_to_tech.get(&ColorTest::Blue).unwrap().get(&1).unwrap(),
    &[TechTest::T1]
  );
  // green  0 - T1
  assert_eq!(
    &**crystalmap.property_to_tech.get(&ColorTest::Green).unwrap().get(&0).unwrap(),
    &[TechTest::T1]
  );
  // green  4 - T3, T4, T5
  assert_eq!(
    &**crystalmap.property_to_tech.get(&ColorTest::Green).unwrap().get(&4).unwrap(),
    &[TechTest::T3, TechTest::T4, TechTest::T5]
  );
  // yellow 1 - T2
  assert_eq!(
    &**crystalmap.property_to_tech.get(&ColorTest::Yellow).unwrap().get(&1).unwrap(),
    &[TechTest::T2]
  );
}

#[test]
#[should_panic]
fn crystalmap_test_2() {
// fails with CrystalMapErrs::TechAlreadyMappedToProperty
  let mut crystalmap = CrystalMap::<ColorTest, TechTest>::new();

  crystalmap.add_mapping(ColorTest::Blue, 1, TechTest::T1).unwrap();
  crystalmap.add_mapping(ColorTest::Blue, 1, TechTest::T1).unwrap();
}

#[test]
#[should_panic]
fn crystalmap_test_3_1() {
// fails with CrystalMapErrs::InconsistencyPropMappedToTech
  let mut crystalmap = CrystalMap::<ColorTest, TechTest>::new();

  crystalmap.add_mapping_prop_tech(ColorTest::Blue, 1, TechTest::T1);
  crystalmap.add_mapping(ColorTest::Blue, 1, TechTest::T1).unwrap();
}

#[test]
#[should_panic]
fn crystalmap_test_3_2() {
// fails with CrystalMapErrs::TechAlreadyMappedToProperty
  let mut crystalmap = CrystalMap::<ColorTest, TechTest>::new();

  crystalmap.add_mapping_prop_tech(ColorTest::Blue, 1, TechTest::T1);
  crystalmap.add_mapping(ColorTest::Blue, 1, TechTest::T1);
  crystalmap.add_mapping(ColorTest::Blue, 1, TechTest::T1).unwrap();
}

#[test]
#[should_panic]
fn crystalmap_test_4_1() {
// fails with CrystalMapErrs::InconsistencyTechMappedToProp
  let mut crystalmap = CrystalMap::<ColorTest, TechTest>::new();

  crystalmap.add_mapping_tech_prop(ColorTest::Blue, 1, TechTest::T1);
  crystalmap.add_mapping(ColorTest::Blue, 1, TechTest::T1).unwrap();
}

#[test]
#[should_panic]
fn crystalmap_test_4_2() {
// fails with CrystalMapErrs::TechAlreadyMappedToProperty
  let mut crystalmap = CrystalMap::<ColorTest, TechTest>::new();

  crystalmap.add_mapping_tech_prop(ColorTest::Blue, 1, TechTest::T1);
  crystalmap.add_mapping(ColorTest::Blue, 1, TechTest::T1);
  crystalmap.add_mapping(ColorTest::Blue, 1, TechTest::T1).unwrap();
}
