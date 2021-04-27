use cucumber_rust::{async_trait, Cucumber, Steps, World};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use tikibase::core::tikibase::Tikibase;

pub struct MyWorld {
    pub base: Tikibase,
    pub findings: Vec<String>,
    pub original_contents: HashMap<PathBuf, String>,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = io::Error;
    async fn new() -> Result<Self, io::Error> {
        let base = Tikibase::tmp();
        Ok(MyWorld {
            base,
            findings: vec![],
            original_contents: HashMap::new(),
        })
    }
}

fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(r#"^file "(.*)" with content:$"#, |mut world, ctx| {
        let filename = ctx.matches.get(1).expect("no filename provided");
        let content = ctx.step.docstring().unwrap().trim_start();
        let filepath = PathBuf::from(filename);
        world.base.create_doc(filepath.clone(), content);
        world
            .original_contents
            .insert(filepath, content.to_string());
        world
    });

    steps.given_regex(r#"^binary file "(.*)"$"#, |mut world, ctx| {
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

    steps.then("all files are unchanged", |world, _ctx| {
        for (filename, original_content) in &world.original_contents {
            let current_content = world.base.get_doc(filename).unwrap().text();
            assert_eq!(&current_content, original_content);
        }
        world
    });

    steps.then_regex(r#"^file "(.*)" should contain:$"#, |world, ctx| {
        let expected = ctx.step.docstring().unwrap().trim_start();
        let filename = ctx.matches.get(1).expect("no filename provided");
        let actual = load_file(&world.base.dir.join(filename));
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

pub fn load_file(filepath: &Path) -> String {
    let mut result = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end()
        .to_string();
    result.push('\n');
    result
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
