
## rust by example doc:
https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html

### tests and ?
allows to test with ?
``` rust
    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
```

### testing panic
``` rust
    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
```

### running specific test and displaying console output
```
cargo test test_name -- --nocapture
```

## Mockall
https://docs.rs/mockall/latest/mockall/

### mock
``` rust
use mockall::*;
use mockall::predicate::*;

#[automock]
trait MyTrait {
  fn foo(&self) -> u32;
  fn bar(&self, x: u32) -> u32;
}

let mut mock = MockMyTrait::new();

mock.expect_foo()
      .return_const(44u32);
  
  mock.expect_bar()
      .with(predicate::eq(4))
      .returning(|x| x + 1);

  assert_eq!(49, function_to_test(&mock));

```


## rstest
https://docs.rs/rstest/latest/rstest/

add fixture and parametrized tests capabilities

``` rust
mod empty_cases {
    use super::*;

    #[fixture]
    fn repository() -> impl Repository {
        DataSet::default()
    }

    #[rstest]
    fn should_do_nothing(repository: impl Repository) {
        //.. test impl ..
    }
}

mod non_trivial_case {
    use super::*;

    #[fixture]
    fn repository() -> impl Repository {
        let mut ds = DataSet::default();
        // Fill your dataset with interesting case
        ds
    }

    #[rstest]
    fn should_notify_all_entries(repository: impl Repository) {
        //.. test impl ..
    }
}
```

``` rust
use rstest::rstest;

#[rstest]
#[case(0, 0)]
#[case(1, 1)]
#[case(2, 1)]
#[case(3, 2)]
#[case(4, 3)]
#[case(5, 5)]
#[case(6, 8)]
fn fibonacci_test(#[case] input: u32,#[case] expected: u32) {
    assert_eq!(expected, fibonacci(input))
}

fn fibonacci(input: u32) -> u32 {
    match input {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 2) + fibonacci(n - 1)
    }
}
```


## test-case
https://crates.io/crates/test-case

```cargo add test-case```

``` rust
#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(-2, -4 ; "when both operands are negative")]
    #[test_case(2,  4  ; "when both operands are positive")]
    #[test_case(4,  2  ; "when operands are swapped")]
    fn multiplication_tests(x: i8, y: i8) {
        let actual = (x * y).abs();

        assert_eq!(8, actual)
    }
}
```

