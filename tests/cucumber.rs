use cucumber::async_trait;
use std::{cell::RefCell, convert::Infallible};

pub struct MyWorld {
  // You can use this struct for mutable context in scenarios.
  foo: String,
  bar: usize,
  some_value: RefCell<u8>,
}

impl MyWorld {
  async fn test_async_fn(&mut self) {
    *self.some_value.borrow_mut() = 123u8;
    self.bar = 123;
  }
}

#[async_trait(?Send)]
impl cucumber::World for MyWorld {
  type Error = Infallible;

  async fn new() -> Result<Self, Infallible> {
    Ok(Self {
      foo: "wat".into(),
      bar: 0,
      some_value: RefCell::new(0),
    })
  }
}

mod example_steps {
  use cucumber::{t, Steps};

  pub fn steps() -> Steps<crate::MyWorld> {
    let mut builder: Steps<crate::MyWorld> = Steps::new();

    builder
      .given_async(
        "a thing",
        t!(|mut world, _step| {
          world.foo = "elho".into();
          world.test_async_fn().await;
          world
        }),
      )
      .when_regex_async("something goes (.*)", t!(|world, _matches, _step| world))
      .given(
        "I am trying out Cucumber",
        |mut world: crate::MyWorld, _step| {
          world.foo = "Some string".to_string();
          world
        },
      )
      .when("I consider what I am doing", |mut world, _step| {
        let new_string = format!("{}.", &world.foo);
        world.foo = new_string;
        world
      })
      .then("I am interested in ATDD", |world, _step| {
        assert_eq!(world.foo, "Some string.");
        world
      })
      .then_regex(
        r"^we can (.*) rules with regex$",
        |world, matches, _step| {
          // And access them as an array
          assert_eq!(matches[1], "implement");
          world
        },
      );

    builder
  }
}

#[tokio::main]
async fn main() {
  // Do any setup you need to do before running the Cucumber runner.
  // e.g. setup_some_db_thing()?;

  cucumber::Cucumber::<MyWorld>::new()
    // Specifies where our feature files exist
    .features(&["./features"])
    // Adds the implementation of our steps to the runner
    .steps(example_steps::steps())
    // Parses the command line arguments if passed
    .cli()
    // Runs the Cucumber tests and then exists
    .run_and_exit()
    .await
}
