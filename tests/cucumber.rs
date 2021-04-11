use cucumber_rust::{async_trait, Context, Cucumber, Steps, World};
use std::convert::Infallible;

pub struct MyWorld {
    pub dir: String,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(MyWorld {
            dir: "".to_string(),
        })
    }
}

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(r#"^a file "(.*)" with content:$"#, |world, ctx| {
        match ctx.step.docstring() {
            None => println!("NO DOCSTRING"),
            Some(str) => println!("FILE CONTENT: '{}'", str),
        }
        println!("CREATING FILE {}", ctx.matches[1]);
        world
    });

    steps.when("I consider what I am doing", |world, _ctx| {
        println!("considering");
        world
    });

    steps.then_regex(r#"^that string is now equal to "(.*)"$"#, |world, _ctx| {
        println!("equal");
        world
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
