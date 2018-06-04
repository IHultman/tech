use ::{Color, Tech};
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
}

impl Tech for TechTest {
  fn init_size() -> usize {10}
}


#[test]
#[should_panic]
fn mastertechstate_builder_test_1() {
// fails with MasterTechStateErrs::TechAlreadyMappedToProperty
  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();
  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();
}

#[test]
#[should_panic]
fn mastertechstate_builder_test_2() {
// fails with MasterTechStateErrs::CycleDetected

  /*
    T1 -> T2 -> T3 -> T6
                   -> T7
             -> T4 -> T8 -> T2
             -> T5
  */

  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();

  mts_builder.add_tech_link(TechTest::T1, TechTest::T2).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T3).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T4).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T5).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T8).unwrap();
  mts_builder.add_tech_link(TechTest::T8, TechTest::T2).unwrap();
}

#[test]
fn mastertechstate_builder_test_3() {
// tests add_tech_link()

  /*
    T1 -> T2 -> T3 -> T6
                   -> T7
             -> T4 -> T6
                   -> T7
                   -> T8 -> T10 -> T3
             -> T5 -> T6
                   -> T9 -> T10 -> T3
  */

  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();

  mts_builder.add_tech_link(TechTest::T1, TechTest::T2).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T3).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T4).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T5).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T8).unwrap();
  mts_builder.add_tech_link(TechTest::T5, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T5, TechTest::T9).unwrap();
  mts_builder.add_tech_link(TechTest::T8, TechTest::T10).unwrap();
  mts_builder.add_tech_link(TechTest::T9, TechTest::T10).unwrap();
  mts_builder.add_tech_link(TechTest::T10, TechTest::T3).unwrap();
}

#[test]
#[should_panic]
fn mastertechstate_builder_test_4() {
// fails with MasterTechStateErrs::CycleDetected

  /*
    T1 -> T2 -> T3 -> T6
                   -> T7
             -> T4 -> T6
                   -> T7
                   -> T8 -> T10 -> T2
             -> T5 -> T6
                   -> T9 -> T10 -> T2
  */

  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();

  mts_builder.add_tech_link(TechTest::T1, TechTest::T2).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T3).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T4).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T5).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T8).unwrap();
  mts_builder.add_tech_link(TechTest::T5, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T5, TechTest::T9).unwrap();
  mts_builder.add_tech_link(TechTest::T8, TechTest::T10).unwrap();
  mts_builder.add_tech_link(TechTest::T9, TechTest::T10).unwrap();
  mts_builder.add_tech_link(TechTest::T10, TechTest::T2).unwrap();
}

#[test]
fn mastertechstate_builder_test_5() {
// tests add_tech_link() with respect to simplifying a graph

  /*
    T1 -> T2 -> T3    ==    T1 -> T2 -> T3
       -> T3
  */

  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();

  mts_builder.add_tech_link(TechTest::T1, TechTest::T2).unwrap();
  mts_builder.add_tech_link(TechTest::T1, TechTest::T3).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T1).unwrap(), &[TechTest::T2, TechTest::T3]);

  mts_builder.add_tech_link(TechTest::T2, TechTest::T3).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T1).unwrap(), &[TechTest::T2]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T2).unwrap(), &[TechTest::T1]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T2).unwrap(), &[TechTest::T3]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T3).unwrap(), &[TechTest::T2]);
}

#[test]
fn mastertechstate_builder_test_6() {
// tests add_tech_link() with respect to simplifying a graph

  /*
    T1 -> T2 -> T5                ==    T1 -> T2 -> T3 -> T4 -> T5
             -> T3 -> T4 -> T5
  */

  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();

  mts_builder.add_tech_link(TechTest::T1, TechTest::T2).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T5).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T3).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T4).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T5).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T1).unwrap(), &[TechTest::T2]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T2).unwrap(), &[TechTest::T1]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T2).unwrap(), &[TechTest::T3]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T3).unwrap(), &[TechTest::T2]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T3).unwrap(), &[TechTest::T4]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T4).unwrap(), &[TechTest::T3]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T4).unwrap(), &[TechTest::T5]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T5).unwrap(), &[TechTest::T4]);
}

#[test]
fn mastertechstate_builder_test_7() {
// tests add_tech_link() with respect to simplifying a graph

  /*
    T1 -> T2 -> T5                ==    T1 -> T2 -> T3 -> T4 -> T5
             -> T3 -> T4 -> T5
  */

  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();

  mts_builder.add_tech_link(TechTest::T1, TechTest::T2).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T3).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T4).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T5).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T5).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T1).unwrap(), &[TechTest::T2]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T2).unwrap(), &[TechTest::T1]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T2).unwrap(), &[TechTest::T3]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T3).unwrap(), &[TechTest::T2]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T3).unwrap(), &[TechTest::T4]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T4).unwrap(), &[TechTest::T3]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T4).unwrap(), &[TechTest::T5]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T5).unwrap(), &[TechTest::T4]);
}

#[test]
fn mastertechstate_builder_test_8() {
// tests add_tech_link() with respect to simplifying a graph

  /*
    T1 -> T5 -> T3 -> T4    ==    T1 -> T2 -> T3 -> T4
       -> T2 -> T4                   -> T5 ---^
             -> T3
  */

  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();

  mts_builder.add_tech_link(TechTest::T1, TechTest::T5).unwrap();
  mts_builder.add_tech_link(TechTest::T1, TechTest::T2).unwrap();
  mts_builder.add_tech_link(TechTest::T5, TechTest::T3).unwrap(); //
  mts_builder.add_tech_link(TechTest::T3, TechTest::T4).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T4).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T2).unwrap(), &[TechTest::T4]);

  mts_builder.add_tech_link(TechTest::T2, TechTest::T3).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T1).unwrap(), &[TechTest::T2, TechTest::T5]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T2).unwrap(), &[TechTest::T1]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T5).unwrap(), &[TechTest::T1]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T2).unwrap(), &[TechTest::T3]);
  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T5).unwrap(), &[TechTest::T3]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T3).unwrap(), &[TechTest::T2, TechTest::T5]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T3).unwrap(), &[TechTest::T4]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T4).unwrap(), &[TechTest::T3]);
}

#[test]
fn mastertechstate_builder_test_9() {
// tests add_tech_link() with respect to simplifying a graph

  /*
    T1 -> T5 -> T3 -> T4    ==    T1 -> T5 -> T3 -> T4
       -> T2 -> T6 -> T4             -> T2 ---^     ^
             -> T3                         ->T6 ----^
  */

  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();

  mts_builder.add_tech_link(TechTest::T1, TechTest::T5).unwrap();
  mts_builder.add_tech_link(TechTest::T1, TechTest::T2).unwrap();
  mts_builder.add_tech_link(TechTest::T5, TechTest::T3).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T4).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T6, TechTest::T4).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T3).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T1).unwrap(), &[TechTest::T2, TechTest::T5]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T2).unwrap(), &[TechTest::T1]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T5).unwrap(), &[TechTest::T1]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T2).unwrap(), &[TechTest::T3, TechTest::T6]);
  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T5).unwrap(), &[TechTest::T3]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T3).unwrap(), &[TechTest::T2, TechTest::T5]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T6).unwrap(), &[TechTest::T2]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T3).unwrap(), &[TechTest::T4]);
  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T6).unwrap(), &[TechTest::T4]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T4).unwrap(), &[TechTest::T3, TechTest::T6]);
}

#[test]
fn mastertechstate_builder_test_10() {
// tests add_tech_link() with respect to simplifying a graph

  /*
    T1 -> T5 -> T3 -> T4
       -> T2 -> T4
             -> T7 -> T6 -> T3
  */

  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();

  mts_builder.add_tech_link(TechTest::T1, TechTest::T5).unwrap();
  mts_builder.add_tech_link(TechTest::T1, TechTest::T2).unwrap();
  mts_builder.add_tech_link(TechTest::T5, TechTest::T3).unwrap();
  mts_builder.add_tech_link(TechTest::T3, TechTest::T4).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T4).unwrap();
  mts_builder.add_tech_link(TechTest::T2, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T7, TechTest::T6).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T2).unwrap(), &[TechTest::T4, TechTest::T7]);

  mts_builder.add_tech_link(TechTest::T6, TechTest::T3).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T1).unwrap(), &[TechTest::T2, TechTest::T5]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T2).unwrap(), &[TechTest::T1]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T5).unwrap(), &[TechTest::T1]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T2).unwrap(), &[TechTest::T7]);
  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T5).unwrap(), &[TechTest::T3]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T3).unwrap(), &[TechTest::T5, TechTest::T6]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T7).unwrap(), &[TechTest::T2]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T3).unwrap(), &[TechTest::T4]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T4).unwrap(), &[TechTest::T3]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T7).unwrap(), &[TechTest::T6]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T6).unwrap(), &[TechTest::T7]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T6).unwrap(), &[TechTest::T3]);
}

#[test]
fn mastertechstate_builder_test_11() {
// tests add_tech_link() with respect to simplifying a graph

  /*
    T1 -> T4 -> T5 -> T6          T3 ---v
       -> T7 -> T9 -> T10         T1 -> T7 -> T9 -> T10
    T2 -> T5                        \_> T4 -> T8 -> T5 -> T6
       -> T6                      T2 ---^     ^
       -> T4 -> T8 -> T5    ==    T3 ---------^
    T3 -> T6
       -> T8
       -> T10
       -> T7
  */

  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();

  mts_builder.add_tech_link(TechTest::T1, TechTest::T4).unwrap();
  mts_builder.add_tech_link(TechTest::T1, TechTest::T7).unwrap();
  mts_builder.add_tech_link(TechTest::T4, TechTest::T5).unwrap();
  mts_builder.add_tech_link(TechTest::T5, TechTest::T6).unwrap();
  mts_builder.add_tech_link(TechTest::T7, TechTest::T9).unwrap();
  mts_builder.add_tech_link(TechTest::T9, TechTest::T10).unwrap();

  mts_builder.add_property_mapping(ColorTest::Green, 0, TechTest::T2).unwrap();

  mts_builder.add_tech_link(TechTest::T2, TechTest::T5).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T2).unwrap(), &[TechTest::T5]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T5).unwrap(), &[TechTest::T2, TechTest::T4]);

  mts_builder.add_tech_link(TechTest::T2, TechTest::T6).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T2).unwrap(), &[TechTest::T5]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T6).unwrap(), &[TechTest::T5]);

  mts_builder.add_tech_link(TechTest::T2, TechTest::T4).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T2).unwrap(), &[TechTest::T4]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T5).unwrap(), &[TechTest::T4]);

  mts_builder.add_tech_link(TechTest::T4, TechTest::T8).unwrap();
  mts_builder.add_tech_link(TechTest::T8, TechTest::T5).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T4).unwrap(), &[TechTest::T8]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T5).unwrap(), &[TechTest::T8]);

  mts_builder.add_property_mapping(ColorTest::Red, 0, TechTest::T3).unwrap();

  mts_builder.add_tech_link(TechTest::T3, TechTest::T6).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T3).unwrap(), &[TechTest::T6]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T6).unwrap(), &[TechTest::T3, TechTest::T5]);

  mts_builder.add_tech_link(TechTest::T3, TechTest::T8).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T3).unwrap(), &[TechTest::T8]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T6).unwrap(), &[TechTest::T5]);

  mts_builder.add_tech_link(TechTest::T3, TechTest::T10).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T3).unwrap(), &[TechTest::T8, TechTest::T10]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T10).unwrap(), &[TechTest::T3, TechTest::T9]);

  mts_builder.add_tech_link(TechTest::T3, TechTest::T7).unwrap();

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T3).unwrap(), &[TechTest::T7, TechTest::T8]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T10).unwrap(), &[TechTest::T9]);

  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T1).unwrap(), &[TechTest::T4, TechTest::T7]);
  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T2).unwrap(), &[TechTest::T4]);
  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T3).unwrap(), &[TechTest::T7, TechTest::T8]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T4).unwrap(), &[TechTest::T1, TechTest::T2]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T7).unwrap(), &[TechTest::T1, TechTest::T3]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T8).unwrap(), &[TechTest::T3, TechTest::T4]);
  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T4).unwrap(), &[TechTest::T8]);
  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T7).unwrap(), &[TechTest::T9]);
  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T8).unwrap(), &[TechTest::T5]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T5).unwrap(), &[TechTest::T8]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T9).unwrap(), &[TechTest::T7]);
  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T5).unwrap(), &[TechTest::T6]);
  assert_eq!(mts_builder.mastergraph.get_out_edges(TechTest::T9).unwrap(), &[TechTest::T10]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T6).unwrap(), &[TechTest::T5]);
  assert_eq!(mts_builder.mastergraph.get_in_edges(TechTest::T10).unwrap(), &[TechTest::T9]);
}
