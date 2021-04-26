use cucumber_rust::{async_trait, Cucumber, Steps, World};
use std::io;
use std::path::PathBuf;
use tikibase::core::persistence;
use tikibase::core::tikibase::Tikibase;

pub struct MyWorld {
    pub base: Tikibase,
    pub findings: Vec<String>,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = io::Error;
    async fn new() -> Result<Self, io::Error> {
        let base = persistence::tmpbase();
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
        world.base.create_doc(PathBuf::from(filename), content);
        world
    });

    steps.given_regex(r#"^resource file "(.*)"$"#, |mut world, ctx| {
        let filename = ctx.matches.get(1).expect("no filename provided");
        world
            .base
            .create_resource(PathBuf::from(filename), "binary content");
        world
    });

    steps.when("checking", |mut world, _ctx| {
        world.findings = tikibase::probes::run(&mut world.base, false);
        world
    });

    steps.when("doing a pitstop", |mut world, _ctx| {
        world.findings = tikibase::probes::run(&mut world.base, true);
        world
    });

    steps.when("fixing", |mut world, _ctx| {
        tikibase::probes::run(&mut world.base, true);
        world
    });

    steps.then_regex(r#"^file "(.*)" should contain:$"#, |world, ctx| {
        let expected = ctx.step.docstring().unwrap().trim_start();
        let filename = ctx.matches.get(1).expect("no filename provided");
        let actual = persistence::load_file(&world.base.dir.join(filename));
        assert_eq!(actual, expected);
        world
    });

    steps.then("it prints:", |world, ctx| {
        let expected: Vec<&str> = ctx.step.docstring().unwrap().trim().split("\n").collect();
        assert_eq!(&world.findings, &expected);
        world
    });

    steps.then("it finds no issues", |world, _ctx| {
        let expected: Vec<&str> = vec![];
        assert_eq!(world.findings, expected);
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
