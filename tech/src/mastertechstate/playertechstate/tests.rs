use ::{Color, Tech};
use ::mastertechstate::builder::MasterTechStateBuilder;
use ::mastertechstate::MasterTechState;
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
  T6,
  T7,
  T8,
  T9,
  T10,
  T11,
  T12,
  T13,
  T14,
}

impl Tech for TechTest {
  fn init_size() -> usize {10}
}


#[test]
#[should_panic]
fn playertechstate_test_1() {
// fails with PlayerTechStateErrs::AllPropertiesToColorDiscovered
  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Red, 0, TechTest::T1).unwrap();
  mts_builder.add_property_mapping(ColorTest::Red, 0, TechTest::T3).unwrap();
  mts_builder.add_property_mapping(ColorTest::Red, 1, TechTest::T1).unwrap();
  mts_builder.add_property_mapping(ColorTest::Red, 1, TechTest::T2).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 1, TechTest::T2).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 1, TechTest::T4).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 1, TechTest::T5).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 3, TechTest::T3).unwrap();
  mts_builder.add_property_mapping(ColorTest::Green, 0, TechTest::T4).unwrap();
  mts_builder.add_property_mapping(ColorTest::Green, 1, TechTest::T5).unwrap();
  mts_builder.add_property_mapping(ColorTest::Green, 2, TechTest::T5).unwrap();
  mts_builder.add_property_mapping(ColorTest::Yellow, 0, TechTest::T4).unwrap();
  mts_builder.add_property_mapping(ColorTest::Yellow, 1, TechTest::T1).unwrap();
  mts_builder.add_property_mapping(ColorTest::Yellow, 1, TechTest::T4).unwrap();
  mts_builder.add_property_mapping(ColorTest::Yellow, 1, TechTest::T5).unwrap();
  /*
  (Red, 0)    -> [T1, T3]
  (Red, 1)    -> [T1, T2]
  (Blue, 0)   -> [T1]
  (Blue, 1)   -> [T2, T4, T5]
  (Blue, 3)   -> [T3]
  (Green, 0)  -> [T4]
  (Green, 1)  -> [T5]
  (Green, 2)  -> [T5]
  (Yellow, 0) -> [T4]
  (Yellow, 1) -> [T1, T4, T5]
  */

  mts_builder.add_tech_link(TechTest::T1, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T1, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T8).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T8).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T9).unwrap();
  mts_builder.add_tech_link(TechTest::T5, TechTest::T10).unwrap();
  mts_builder.add_tech_link(TechTest::T6, TechTest::T11).unwrap();
  mts_builder.add_tech_link(TechTest::T6, TechTest::T12).unwrap();
  mts_builder.add_tech_link(TechTest::T7, TechTest::T11).unwrap();
  mts_builder.add_tech_link(TechTest::T7, TechTest::T13).unwrap();
  mts_builder.add_tech_link(TechTest::T8, TechTest::T11).unwrap();
  mts_builder.add_tech_link(TechTest::T9, TechTest::T12).unwrap();
  mts_builder.add_tech_link(TechTest::T10, TechTest::T12).unwrap();
  mts_builder.add_tech_link(TechTest::T10, TechTest::T13).unwrap();
  /*
                     T1  -> [T6, T7]
                     T2  -> [T6, T7, T8]
                     T3  -> [T8]
                     T4  -> [T6, T9]
                     T5  -> [T10]
    [T1, T2, T4]  -> T6  -> [T11, T12]
    [T1, T2]      -> T7  -> [T11, T13]
    [T2, T3]      -> T8  -> [T11]
    [T4]          -> T9  -> [T12]
    [T5]          -> T10 -> [T12, T13]
    [T6, T7, T8]  -> T11
    [T6, T9, T10] -> T12
    [T7, T10]     -> T13
  */

  let mts = mts_builder.build();
  let mut pts = mts.mk_new_player();

  assert!(pts.properties_discovered.is_none() );

  pts.discover_property_rand(ColorTest::Green).unwrap();
  pts.discover_property_rand(ColorTest::Green).unwrap();
  pts.discover_property_rand(ColorTest::Green).unwrap();

  assert!(pts.properties_undiscovered.as_ref().unwrap().get(&ColorTest::Green).is_none() );
  assert_eq!(
    &**pts.properties_discovered.as_ref().unwrap().
    get(&ColorTest::Green).unwrap(),
    &[0, 1, 2]
  );

  pts.discover_property_rand(ColorTest::Green).unwrap();
}

#[test]
fn playertechstate_test_2() {
// tests discover_property_rand()
  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Red, 0, TechTest::T1).unwrap();
  mts_builder.add_property_mapping(ColorTest::Red, 0, TechTest::T3).unwrap();
  mts_builder.add_property_mapping(ColorTest::Red, 1, TechTest::T1).unwrap();
  mts_builder.add_property_mapping(ColorTest::Red, 1, TechTest::T2).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 1, TechTest::T2).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 1, TechTest::T4).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 1, TechTest::T5).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 3, TechTest::T3).unwrap();
  mts_builder.add_property_mapping(ColorTest::Green, 0, TechTest::T4).unwrap();
  mts_builder.add_property_mapping(ColorTest::Green, 1, TechTest::T5).unwrap();
  mts_builder.add_property_mapping(ColorTest::Green, 2, TechTest::T5).unwrap();
  mts_builder.add_property_mapping(ColorTest::Yellow, 0, TechTest::T4).unwrap();
  mts_builder.add_property_mapping(ColorTest::Yellow, 1, TechTest::T1).unwrap();
  mts_builder.add_property_mapping(ColorTest::Yellow, 1, TechTest::T4).unwrap();
  mts_builder.add_property_mapping(ColorTest::Yellow, 1, TechTest::T5).unwrap();
  /*
  (Red, 0)    -> [T1, T3]
  (Red, 1)    -> [T1, T2]
  (Blue, 0)   -> [T1]
  (Blue, 1)   -> [T2, T4, T5]
  (Blue, 3)   -> [T3]
  (Green, 0)  -> [T4]
  (Green, 1)  -> [T5]
  (Green, 2)  -> [T5]
  (Yellow, 0) -> [T4]
  (Yellow, 1) -> [T1, T4, T5]
  */

  mts_builder.add_tech_link(TechTest::T1, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T1, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T8).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T8).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T9).unwrap();
  mts_builder.add_tech_link(TechTest::T5, TechTest::T10).unwrap();
  mts_builder.add_tech_link(TechTest::T6, TechTest::T11).unwrap();
  mts_builder.add_tech_link(TechTest::T6, TechTest::T12).unwrap();
  mts_builder.add_tech_link(TechTest::T7, TechTest::T11).unwrap();
  mts_builder.add_tech_link(TechTest::T7, TechTest::T13).unwrap();
  mts_builder.add_tech_link(TechTest::T8, TechTest::T11).unwrap();
  mts_builder.add_tech_link(TechTest::T9, TechTest::T12).unwrap();
  mts_builder.add_tech_link(TechTest::T10, TechTest::T12).unwrap();
  mts_builder.add_tech_link(TechTest::T10, TechTest::T13).unwrap();
  /*
                     T1  -> [T6, T7]
                     T2  -> [T6, T7, T8]
                     T3  -> [T8]
                     T4  -> [T6, T9]
                     T5  -> [T10]
    [T1, T2, T4]  -> T6  -> [T11, T12]
    [T1, T2]      -> T7  -> [T11, T13]
    [T2, T3]      -> T8  -> [T11]
    [T4]          -> T9  -> [T12]
    [T5]          -> T10 -> [T12, T13]
    [T6, T7, T8]  -> T11
    [T6, T9, T10] -> T12
    [T7, T10]     -> T13
  */

  let mts = mts_builder.build();
  let mut pts = mts.mk_new_player();

  pts.discover_property_rand(ColorTest::Green).unwrap();
  pts.discover_property_rand(ColorTest::Green).unwrap();
  pts.discover_property_rand(ColorTest::Green).unwrap();
  pts.discover_property_rand(ColorTest::Red).unwrap();
  pts.discover_property_rand(ColorTest::Red).unwrap();
  pts.discover_property_rand(ColorTest::Yellow).unwrap();
  pts.discover_property_rand(ColorTest::Yellow).unwrap();
  pts.discover_property_rand(ColorTest::Blue).unwrap();
  pts.discover_property_rand(ColorTest::Blue).unwrap();
  pts.discover_property_rand(ColorTest::Blue).unwrap();

  assert_eq!(
    &**pts.properties_discovered.as_ref().unwrap().
    get(&ColorTest::Red).unwrap(),
    &[0, 1]
  );

  assert_eq!(
    &**pts.properties_discovered.as_ref().unwrap().
    get(&ColorTest::Blue).unwrap(),
    &[0, 1, 3]
  );

  assert_eq!(
    &**pts.properties_discovered.as_ref().unwrap().
    get(&ColorTest::Green).unwrap(),
    &[0, 1, 2]
  );

  assert_eq!(
    &**pts.properties_discovered.as_ref().unwrap().
    get(&ColorTest::Yellow).unwrap(),
    &[0, 1]
  );

  assert!(pts.properties_undiscovered.is_none() );

  assert_eq!(
    pts.discover_property_rand(ColorTest::Red).unwrap_err(),
    PlayerTechStateErrs::AllPropertiesDiscovered
  );

  assert_eq!(
    pts.discover_property_rand(ColorTest::Blue).unwrap_err(),
    PlayerTechStateErrs::AllPropertiesDiscovered
  );

  assert_eq!(
    pts.discover_property_rand(ColorTest::Green).unwrap_err(),
    PlayerTechStateErrs::AllPropertiesDiscovered
  );

  assert_eq!(
    pts.discover_property_rand(ColorTest::Yellow).unwrap_err(),
    PlayerTechStateErrs::AllPropertiesDiscovered::<ColorTest, TechTest>
  );
}

#[test]
fn playertechstate_test_3() {
//
  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Red, 0, TechTest::T1).unwrap();
  mts_builder.add_property_mapping(ColorTest::Red, 0, TechTest::T3).unwrap();
  mts_builder.add_property_mapping(ColorTest::Red, 1, TechTest::T1).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 1, TechTest::T2).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 1, TechTest::T4).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 1, TechTest::T5).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 3, TechTest::T3).unwrap();
  mts_builder.add_property_mapping(ColorTest::Green, 0, TechTest::T5).unwrap();
  mts_builder.add_property_mapping(ColorTest::Green, 1, TechTest::T5).unwrap();
  mts_builder.add_property_mapping(ColorTest::Green, 2, TechTest::T5).unwrap();
  mts_builder.add_property_mapping(ColorTest::Yellow, 0, TechTest::T4).unwrap();
  mts_builder.add_property_mapping(ColorTest::Yellow, 1, TechTest::T4).unwrap();
  mts_builder.add_property_mapping(ColorTest::Yellow, 1, TechTest::T5).unwrap();
  /*
  (Red, 0)    -> [T1, T3]
  (Red, 1)    -> [T1]
  (Blue, 0)   -> [T1]
  (Blue, 1)   -> [T2, T4, T5]
  (Blue, 3)   -> [T3]
  (Green, 0)  -> [T5]
  (Green, 1)  -> [T5]
  (Green, 2)  -> [T5]
  (Yellow, 0) -> [T4]
  (Yellow, 1) -> [T4, T5]
  */

  mts_builder.add_tech_link(TechTest::T1, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T1, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T8).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T8).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T9).unwrap();
  mts_builder.add_tech_link(TechTest::T5, TechTest::T10).unwrap();
  mts_builder.add_tech_link(TechTest::T6, TechTest::T11).unwrap();
  mts_builder.add_tech_link(TechTest::T6, TechTest::T12).unwrap();
  mts_builder.add_tech_link(TechTest::T7, TechTest::T11).unwrap();
  mts_builder.add_tech_link(TechTest::T7, TechTest::T13).unwrap();
  mts_builder.add_tech_link(TechTest::T8, TechTest::T11).unwrap();
  mts_builder.add_tech_link(TechTest::T8, TechTest::T14).unwrap();
  mts_builder.add_tech_link(TechTest::T9, TechTest::T12).unwrap();
  mts_builder.add_tech_link(TechTest::T10, TechTest::T12).unwrap();
  mts_builder.add_tech_link(TechTest::T10, TechTest::T13).unwrap();
  /*
                     T1  -> [T6, T7]
                     T2  -> [T6, T7, T8]
                     T3  -> [T8]
                     T4  -> [T6, T9]
                     T5  -> [T10]
    [T1, T2, T4]  -> T6  -> [T11, T12]
    [T1, T2]      -> T7  -> [T11, T13]
    [T2, T3]      -> T8  -> [T11, T14]
    [T4]          -> T9  -> [T12]
    [T5]          -> T10 -> [T12, T13]
    [T6, T7, T8]  -> T11
    [T6, T9, T10] -> T12
    [T7, T10]     -> T13
    [T8]          -> T14
  */

  let mts = mts_builder.build();
  let mut pts = mts.mk_new_player();

  pts.discover_prop_and_update(ColorTest::Blue).unwrap();
  pts.discover_prop_and_update(ColorTest::Blue).unwrap();
  pts.discover_prop_and_update(ColorTest::Blue).unwrap();

  pts.player_graph.check_tech(TechTest::T2).unwrap();
  assert!(pts.player_graph.check_tech(TechTest::T1).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T3).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T4).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T5).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T6).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T7).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T8).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T9).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T10).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T11).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T12).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T13).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T14).is_err() );

  pts.discover_prop_and_update(ColorTest::Red).unwrap();
  pts.discover_prop_and_update(ColorTest::Red).unwrap();

  pts.player_graph.check_tech(TechTest::T1).unwrap();
  pts.player_graph.check_tech(TechTest::T2).unwrap();
  pts.player_graph.check_tech(TechTest::T3).unwrap();
  pts.player_graph.check_tech(TechTest::T7).unwrap();
  pts.player_graph.check_tech(TechTest::T8).unwrap();
  pts.player_graph.check_tech(TechTest::T14).unwrap();
  assert!(pts.player_graph.check_tech(TechTest::T4).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T5).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T6).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T9).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T10).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T11).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T12).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T13).is_err() );

  pts.research_tech(TechTest::T2).unwrap();
  pts.discover_prop_and_update(ColorTest::Yellow).unwrap();
  pts.discover_prop_and_update(ColorTest::Yellow).unwrap();

  pts.player_graph.check_tech(TechTest::T1).unwrap();
  pts.player_graph.check_tech(TechTest::T2).unwrap();
  pts.player_graph.check_tech(TechTest::T3).unwrap();
  pts.player_graph.check_tech(TechTest::T4).unwrap();
  pts.player_graph.check_tech(TechTest::T6).unwrap();
  pts.player_graph.check_tech(TechTest::T7).unwrap();
  pts.player_graph.check_tech(TechTest::T8).unwrap();
  pts.player_graph.check_tech(TechTest::T9).unwrap();
  pts.player_graph.check_tech(TechTest::T11).unwrap();
  pts.player_graph.check_tech(TechTest::T14).unwrap();
  assert!(pts.player_graph.check_tech(TechTest::T5).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T10).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T12).is_err() );
  assert!(pts.player_graph.check_tech(TechTest::T13).is_err() );

  pts.research_tech(TechTest::T3).unwrap();
  pts.research_tech(TechTest::T8).unwrap();
  pts.research_tech(TechTest::T14).unwrap();

  pts.discover_prop_and_update(ColorTest::Green).unwrap();
  pts.discover_prop_and_update(ColorTest::Green).unwrap();
  pts.discover_prop_and_update(ColorTest::Green).unwrap();

  pts.player_graph.check_tech(TechTest::T1).unwrap();
  pts.player_graph.check_tech(TechTest::T2).unwrap();
  pts.player_graph.check_tech(TechTest::T3).unwrap();
  pts.player_graph.check_tech(TechTest::T4).unwrap();
  pts.player_graph.check_tech(TechTest::T5).unwrap();
  pts.player_graph.check_tech(TechTest::T6).unwrap();
  pts.player_graph.check_tech(TechTest::T7).unwrap();
  pts.player_graph.check_tech(TechTest::T8).unwrap();
  pts.player_graph.check_tech(TechTest::T9).unwrap();
  pts.player_graph.check_tech(TechTest::T10).unwrap();
  pts.player_graph.check_tech(TechTest::T11).unwrap();
  pts.player_graph.check_tech(TechTest::T12).unwrap();
  pts.player_graph.check_tech(TechTest::T13).unwrap();
  pts.player_graph.check_tech(TechTest::T14).unwrap();

  assert!(pts.research_tech(TechTest::T6).is_err() );
  assert!(pts.research_tech(TechTest::T7).is_err() );
  assert!(pts.research_tech(TechTest::T9).is_err() );
  assert!(pts.research_tech(TechTest::T10).is_err() );
  assert!(pts.research_tech(TechTest::T11).is_err() );
  assert!(pts.research_tech(TechTest::T12).is_err() );
  assert!(pts.research_tech(TechTest::T13).is_err() );

  pts.research_tech(TechTest::T1).unwrap();

  assert!(pts.research_tech(TechTest::T6).is_err() );
  assert!(pts.research_tech(TechTest::T9).is_err() );
  assert!(pts.research_tech(TechTest::T10).is_err() );
  assert!(pts.research_tech(TechTest::T11).is_err() );
  assert!(pts.research_tech(TechTest::T12).is_err() );
  assert!(pts.research_tech(TechTest::T13).is_err() );

  pts.research_tech(TechTest::T7).unwrap();
  pts.research_tech(TechTest::T4).unwrap();
  pts.research_tech(TechTest::T5).unwrap();
  pts.research_tech(TechTest::T6).unwrap();
  pts.research_tech(TechTest::T9).unwrap();
  pts.research_tech(TechTest::T10).unwrap();
  pts.research_tech(TechTest::T11).unwrap();
  pts.research_tech(TechTest::T12).unwrap();
  pts.research_tech(TechTest::T13).unwrap();
}
