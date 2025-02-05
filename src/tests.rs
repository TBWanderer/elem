use super::{conditions::*, operators::*, *};

#[test]
fn show_test() {
    assert_eq!(&list![1, 2, 3].show(), "(1 2 3)");
    assert_eq!(&list![nil!(), 1, 2, "test"].show(), "(() 1 2 test)");
    assert_eq!(&pair!(nil!(), 2).show(), "(() . 2)");
    assert_eq!(&nil!().show(), "()");
}

#[test]
fn test_ladd() {
    assert_eq!(ladd(list![num!(1), num!(2), num!(3)]), num!(6));
    assert_eq!(ladd(list![num!(10), num!(-5), num!(3)]), num!(8));
    assert_eq!(ladd(list![]), num!(0));
}

#[test]
#[should_panic(expected = "Non-number element in list")]
fn test_ladd_invalid() {
    ladd(list![num!(1), name!("x"), num!(2)]);
}

#[test]
fn test_lmul() {
    assert_eq!(lmul(list![num!(2), num!(3), num!(4)]), num!(24));
    assert_eq!(lmul(list![num!(10), num!(-2)]), num!(-20));
    assert_eq!(lmul(list![]), num!(1));
}

#[test]
#[should_panic(expected = "Non-number element in list")]
fn test_lmul_invalid() {
    lmul(list![num!(2), name!("x"), num!(4)]);
}

#[test]
fn test_leq() {
    assert_eq!(leq(list![num!(5), num!(5)]), num!(1));
    assert_eq!(leq(list![num!(3), num!(4)]), nil!());
}

#[test]
#[should_panic(expected = "Too many arguments!")]
fn test_leq_invalid_length() {
    leq(list![num!(1), num!(2), num!(3)]);
}

#[test]
fn test_lgt() {
    assert_eq!(lgt(list![num!(5), num!(3)]), num!(1));
    assert_eq!(lgt(list![num!(2), num!(4)]), nil!());
}

#[test]
#[should_panic(expected = "Arguments must be numbers")]
fn test_lgt_invalid() {
    lgt(list![num!(3), name!("x")]);
}

#[test]
fn test_llt() {
    assert_eq!(llt(list![num!(2), num!(3)]), num!(1));
    assert_eq!(llt(list![num!(4), num!(2)]), nil!());
}

#[test]
#[should_panic(expected = "Arguments must be numbers")]
fn test_llt_invalid() {
    llt(list![num!(3), name!("x")]);
}

#[test]
fn test_ldiv() {
    assert_eq!(ldiv(list![num!(10), num!(2)]), num!(5));
    assert_eq!(ldiv(list![num!(9), num!(3)]), num!(3));
}

#[test]
#[should_panic(expected = "Division by zero")]
fn test_ldiv_zero() {
    ldiv(list![num!(1), num!(0)]);
}

#[test]
#[should_panic(expected = "Arguments must be numbers")]
fn test_ldiv_invalid() {
    ldiv(list![num!(10), name!("x")]);
}

#[test]
fn test_lsub() {
    assert_eq!(lsub(list![num!(10), num!(3)]), num!(7));
    assert_eq!(lsub(list![num!(5), num!(8)]), num!(-3));
}

#[test]
#[should_panic(expected = "Arguments must be numbers")]
fn test_lsub_invalid() {
    lsub(list![num!(5), name!("x")]);
}

#[test]
fn test_lcond_true_case() {
    let conditions = list![
        list![nil!(), num!(42)],
        list![num!(1), num!(99)],
        list![num!(1), num!(123)]
    ];
    assert_eq!(lcond(conditions), num!(99));
}

#[test]
fn test_lcond_false_case() {
    let conditions = list![list![nil!(), num!(42)], list![nil!(), num!(99)]];
    assert_eq!(lcond(conditions), nil!());
}

#[test]
fn test_lcond_empty() {
    let conditions = list![];
    assert_eq!(lcond(conditions), nil!());
}

#[test]
#[should_panic(expected = "cond state error!")]
fn test_lcond_invalid_state() {
    let conditions = list![list![num!(1)]];
    lcond(conditions);
}

#[test]
#[should_panic(expected = "cond syntax error!")]
fn test_lcond_not_a_list() {
    lcond(num!(5));
}

#[test]
fn test_lcond_evaluated_conditions() {
    let conditions = list![
        list![leval(nil!()), num!(42)],
        list![leval(num!(1)), num!(99)]
    ];
    assert_eq!(lcond(conditions), num!(99));
}
