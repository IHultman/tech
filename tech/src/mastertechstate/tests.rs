use ::{Color, Tech};
use super::*;
use super::builder::*;


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
fn mastertechstate_test_1() {
// tests all funtionality and state for MasterTechState
  let mut mts_builder = MasterTechStateBuilder::<ColorTest, TechTest>::new();

  {
    mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T1).unwrap();
    mts_builder.add_property_mapping(ColorTest::Blue, 0, TechTest::T2).unwrap();
    mts_builder.add_property_mapping(ColorTest::Blue, 1, TechTest::T1).unwrap();

    mts_builder.add_property_mapping(ColorTest::Yellow, 0, TechTest::T4).unwrap();
    mts_builder.add_property_mapping(ColorTest::Yellow, 0, TechTest::T5).unwrap();
    mts_builder.add_property_mapping(ColorTest::Yellow, 1, TechTest::T4).unwrap();
    mts_builder.add_property_mapping(ColorTest::Yellow, 1, TechTest::T5).unwrap();
    mts_builder.add_property_mapping(ColorTest::Yellow, 2, TechTest::T3).unwrap();

    mts_builder.add_property_mapping(ColorTest::Red, 0, TechTest::T1).unwrap();
    mts_builder.add_property_mapping(ColorTest::Red, 0, TechTest::T3).unwrap();
    mts_builder.add_property_mapping(ColorTest::Red, 1, TechTest::T2).unwrap();

    mts_builder.add_property_mapping(ColorTest::Green, 0, TechTest::T1).unwrap();
    mts_builder.add_property_mapping(ColorTest::Green, 1, TechTest::T2).unwrap();
    mts_builder.add_property_mapping(ColorTest::Green, 2, TechTest::T3).unwrap();
    mts_builder.add_property_mapping(ColorTest::Green, 3, TechTest::T1).unwrap();
    mts_builder.add_property_mapping(ColorTest::Green, 3, TechTest::T2).unwrap();
  }

  {
    mts_builder.add_tech_link(TechTest::T1, TechTest::T6).unwrap();
    mts_builder.add_tech_link(TechTest::T1, TechTest::T7).unwrap();

    mts_builder.add_tech_link(TechTest::T2, TechTest::T6).unwrap();

    mts_builder.add_tech_link(TechTest::T3, TechTest::T6).unwrap();
    mts_builder.add_tech_link(TechTest::T3, TechTest::T7).unwrap();

    mts_builder.add_tech_link(TechTest::T4, TechTest::T7).unwrap();
    mts_builder.add_tech_link(TechTest::T4, TechTest::T8).unwrap();

    mts_builder.add_tech_link(TechTest::T5, TechTest::T8).unwrap();

    mts_builder.add_tech_link(TechTest::T6, TechTest::T9).unwrap();

    mts_builder.add_tech_link(TechTest::T7, TechTest::T9).unwrap();

    mts_builder.add_tech_link(TechTest::T8, TechTest::T9).unwrap();
    mts_builder.add_tech_link(TechTest::T8, TechTest::T10).unwrap();
  }

  let mts = mts_builder.build();

  {
    assert_eq!(mts.get_tech_list_from_property(ColorTest::Blue, 0).unwrap(), &[TechTest::T1, TechTest::T2]);
    assert_eq!(mts.get_tech_list_from_property(ColorTest::Blue, 1).unwrap(), &[TechTest::T1]);

    assert_eq!(mts.get_tech_list_from_property(ColorTest::Red, 0).unwrap(), &[TechTest::T1, TechTest::T3]);
    assert_eq!(mts.get_tech_list_from_property(ColorTest::Red, 1).unwrap(), &[TechTest::T2]);

    assert_eq!(mts.get_tech_list_from_property(ColorTest::Green, 0).unwrap(), &[TechTest::T1]);
    assert_eq!(mts.get_tech_list_from_property(ColorTest::Green, 1).unwrap(), &[TechTest::T2]);
    assert_eq!(mts.get_tech_list_from_property(ColorTest::Green, 2).unwrap(), &[TechTest::T3]);
    assert_eq!(mts.get_tech_list_from_property(ColorTest::Green, 3).unwrap(), &[TechTest::T1, TechTest::T2]);

    assert_eq!(mts.get_tech_list_from_property(ColorTest::Yellow, 0).unwrap(), &[TechTest::T4, TechTest::T5]);
    assert_eq!(mts.get_tech_list_from_property(ColorTest::Yellow, 1).unwrap(), &[TechTest::T4, TechTest::T5]);
    assert_eq!(mts.get_tech_list_from_property(ColorTest::Yellow, 2).unwrap(), &[TechTest::T3]);
  }

  {
    let mut v = mts.get_properties_for_tech(TechTest::T1).
            unwrap().
            iter().
            map(|(&c, rv)| (c, rv) ).
            collect::<Vec<(ColorTest, &Vec<usize>)> >();
    v.sort_by_key(|&(c, _)| c);
    let mut iter = v.iter();

    let &(c, s) = iter.next().unwrap();
    assert_eq!(c, ColorTest::Red);
    assert_eq!(&**s, &[0]);

    let &(c, s) = iter.next().unwrap();
    assert_eq!(c, ColorTest::Blue);
    assert_eq!(&**s, &[0, 1]);

    let &(c, s) = iter.next().unwrap();
    assert_eq!(c, ColorTest::Green);
    assert_eq!(&**s, &[0, 3]);

    assert!(iter.next().is_none() )
  }

  {
    let mut v = mts.get_properties_for_tech(TechTest::T2).
            unwrap().
            iter().
            map(|(&c, rv)| (c, rv) ).
            collect::<Vec<(ColorTest, &Vec<usize>)> >();
    v.sort_by_key(|&(c, _)| c);
    let mut iter = v.iter();

    let &(c, s) = iter.next().unwrap();
    assert_eq!(c, ColorTest::Red);
    assert_eq!(&**s, &[1]);

    let &(c, s) = iter.next().unwrap();
    assert_eq!(c, ColorTest::Blue);
    assert_eq!(&**s, &[0]);

    let &(c, s) = iter.next().unwrap();
    assert_eq!(c, ColorTest::Green);
    assert_eq!(&**s, &[1, 3]);

    assert!(iter.next().is_none() )
  }

  {
    let mut v = mts.get_properties_for_tech(TechTest::T3).
            unwrap().
            iter().
            map(|(&c, rv)| (c, rv) ).
            collect::<Vec<(ColorTest, &Vec<usize>)> >();
    v.sort_by_key(|&(c, _)| c);
    let mut iter = v.iter();

    let &(c, s) = iter.next().unwrap();
    assert_eq!(c, ColorTest::Red);
    assert_eq!(&**s, &[0]);

    let &(c, s) = iter.next().unwrap();
    assert_eq!(c, ColorTest::Green);
    assert_eq!(&**s, &[2]);

    let &(c, s) = iter.next().unwrap();
    assert_eq!(c, ColorTest::Yellow);
    assert_eq!(&**s, &[2]);

    assert!(iter.next().is_none() )
  }

  {
    let mut v = mts.get_properties_for_tech(TechTest::T4).
            unwrap().
            iter().
            map(|(&c, rv)| (c, rv) ).
            collect::<Vec<(ColorTest, &Vec<usize>)> >();
    v.sort_by_key(|&(c, _)| c);
    let mut iter = v.iter();

    let &(c, s) = iter.next().unwrap();
    assert_eq!(c, ColorTest::Yellow);
    assert_eq!(&**s, &[0, 1]);

    assert!(iter.next().is_none() )
  }

  {
    let mut v = mts.get_properties_for_tech(TechTest::T5).
            unwrap().
            iter().
            map(|(&c, rv)| (c, rv) ).
            collect::<Vec<(ColorTest, &Vec<usize>)> >();
    v.sort_by_key(|&(c, _)| c);
    let mut iter = v.iter();

    let &(c, s) = iter.next().unwrap();
    assert_eq!(c, ColorTest::Yellow);
    assert_eq!(s, &[0, 1]);

    assert!(iter.next().is_none() )
  }

  {
    assert_eq!(mts.get_advanced_tech_list(TechTest::T1).unwrap(), &[TechTest::T6, TechTest::T7]);
    assert!(mts.get_prereq_tech_list(TechTest::T1).is_none() );

    assert_eq!(mts.get_advanced_tech_list(TechTest::T2).unwrap(), &[TechTest::T6]);
    assert!(mts.get_prereq_tech_list(TechTest::T2).is_none() );

    assert_eq!(mts.get_advanced_tech_list(TechTest::T3).unwrap(), &[TechTest::T6, TechTest::T7]);
    assert!(mts.get_prereq_tech_list(TechTest::T3).is_none() );

    assert_eq!(mts.get_advanced_tech_list(TechTest::T4).unwrap(), &[TechTest::T7, TechTest::T8]);
    assert!(mts.get_prereq_tech_list(TechTest::T4).is_none() );

    assert_eq!(mts.get_advanced_tech_list(TechTest::T5).unwrap(), &[TechTest::T8]);
    assert!(mts.get_prereq_tech_list(TechTest::T5).is_none() );

    assert_eq!(mts.get_advanced_tech_list(TechTest::T6).unwrap(), &[TechTest::T9]);
    assert_eq!(mts.get_prereq_tech_list(TechTest::T6).unwrap(), &[TechTest::T1, TechTest::T2, TechTest::T3]);

    assert_eq!(mts.get_advanced_tech_list(TechTest::T7).unwrap(), &[TechTest::T9]);
    assert_eq!(mts.get_prereq_tech_list(TechTest::T7).unwrap(), &[TechTest::T1, TechTest::T3, TechTest::T4]);

    assert_eq!(mts.get_advanced_tech_list(TechTest::T8).unwrap(), &[TechTest::T9, TechTest::T10]);
    assert_eq!(mts.get_prereq_tech_list(TechTest::T8).unwrap(), &[TechTest::T4, TechTest::T5]);

    assert!(mts.get_advanced_tech_list(TechTest::T9).is_none() );
    assert_eq!(mts.get_prereq_tech_list(TechTest::T9).unwrap(), &[TechTest::T6, TechTest::T7, TechTest::T8]);

    assert!(mts.get_advanced_tech_list(TechTest::T10).is_none() );
    assert_eq!(mts.get_prereq_tech_list(TechTest::T10).unwrap(), &[TechTest::T8]);
  }
}
