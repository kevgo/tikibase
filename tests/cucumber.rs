use cucumber_rust::Steps;
use cucumber_rust::{async_trait, Context, Cucumber, World};
use std::convert::Infallible;

pub enum MyWorld {
    Nothing,
    SomeString(String),
    SuffixedString(String),
    TwoStrings(String, String),
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::Nothing)
    }
}

pub fn steps() -> Steps<crate::MyWorld> {
    let mut steps: Steps<crate::MyWorld> = Steps::new();

    steps.given("a string with some particular value", |_world, _ctx| {
        MyWorld::SomeString("hello".to_string())
    });

    steps.when(
        "I append a known suffix to the value",
        |world, _ctx| match world {
            MyWorld::SomeString(_) => MyWorld::SuffixedString("two".to_string()),
            _ => panic!("Invalid world state"),
        },
    );

    steps.then_regex(r#"^that string is now equal to "(.*)"$"#, |world, ctx| {
        match world {
            MyWorld::SuffixedString(x) => assert_eq!(x, ctx.matches[1]),
            _ => panic!("Invalid world state"),
        }
        MyWorld::Nothing
    });

    steps.then("we find we somehow had the same string", |world, _| {
        match world {
            MyWorld::TwoStrings(a, b) => assert_eq!(a, b),
            _ => panic!("Invalid world state"),
        }
        MyWorld::Nothing
    });

    steps
}

#[tokio::main]
async fn main() {
    let pool = "the pool";

    Cucumber::<MyWorld>::new()
        // Specifies where our feature files exist
        .features(&["./features"])
        // Adds the implementation of our steps to the runner
        .steps(steps())
        // Add some global context for all the tests, like databases.
        .context(Context::new().add(pool))
        // Add some lifecycle functions to manage our database nightmare
        // .before(feature("Example feature"), |ctx| println!("").boxed())
        // .after(feature("Example feature"), |ctx| {
        //     async move { drop_tables(&pool).await }.boxed()
        // })
        // Parses the command line arguments if passed
        .cli()
        // Runs the Cucumber tests and then exists
        .run_and_exit()
        .await
}
