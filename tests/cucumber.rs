use cucumber_rust::{async_trait, Cucumber, Steps, World};
use std::io;
use std::path::PathBuf;
use tikibase::core::tikibase::helpers;
use tikibase::core::tikibase::Tikibase;

pub struct MyWorld {
    pub base: Tikibase,
    pub findings: Vec<String>,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = io::Error;
    async fn new() -> Result<Self, io::Error> {
        let base = helpers::testbase();
        Ok(MyWorld {
            base,
            findings: vec![],
        })
    }
}

fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(r#"^file "(.*)" with content:$"#, |mut world, ctx| {
        let filename = ctx.matches.get(1).expect("no filename provided");
        let content = ctx.step.docstring().unwrap().trim_start();
        world.base.create_doc(&PathBuf::from(filename), content);
        world
    });

    steps.when("checking", |mut world, _ctx| {
        world.findings = tikibase::check::process(&mut world.base, false);
        world
    });

    steps.when("fixing", |mut world, _ctx| {
        tikibase::check::process(&mut world.base, true);
        world
    });

    steps.then_regex(r#"^file "(.*)" should contain:$"#, |world, ctx| {
        let expected = ctx.step.docstring().unwrap().trim_start();
        let filename = ctx.matches.get(1).expect("no filename provided");
        let actual = helpers::read_doc(&world.base, filename);
        assert_eq!(actual, expected);
        world
    });

    steps.then_regex("it finds these errors:", |world, ctx| {
        let expected: Vec<&str> = ctx.step.docstring().unwrap().trim().split("\n").collect();
        assert_eq!(&world.findings, &expected);
        world
    });

    steps
}

#[tokio::main]
async fn main() {
    // let pool = "the pool";

    Cucumber::<MyWorld>::new()
        .features(&["./features"])
        .steps(steps())
        // Add some global context for all the tests, like databases.
        // .context(Context::new().add(pool))
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
