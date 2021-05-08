use cucumber_rust::{async_trait, Cucumber, Steps, World};
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use tikibase::core::tikibase::Tikibase;
use tikibase::testhelpers;

pub struct MyWorld {
    /// the directory in which the Tikibase under test is located
    pub dir: PathBuf,
    /// results of the Tikibase run
    pub findings: Vec<String>,
    /// content of the files before the Tikibase command ran
    pub original_contents: HashMap<PathBuf, String>,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = io::Error;
    async fn new() -> Result<Self, io::Error> {
        Ok(MyWorld {
            dir: testhelpers::tmp_dir(),
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
        testhelpers::create_file(filename, content, &world.dir);
        world
            .original_contents
            .insert(filepath, content.to_string());
        world
    });

    steps.given_regex(r#"^binary file "(.*)"$"#, |world, ctx| {
        let filename = ctx.matches.get(1).expect("no filename provided");
        testhelpers::create_file(filename, "binary content", &world.dir);
        world
    });

    steps.when("checking", |mut world, _ctx| {
        let (base, errs) = Tikibase::load(world.dir.clone());
        world.findings = errs;
        world
            .findings
            .append(&mut tikibase::probes::run(base, false));
        world
    });

    steps.when("doing a pitstop", |mut world, _ctx| {
        let (base, errs) = Tikibase::load(world.dir.clone());
        world.findings = errs;
        world
            .findings
            .append(&mut tikibase::probes::run(base, true));
        world
    });

    steps.when("fixing", |mut world, _ctx| {
        let (base, errs) = Tikibase::load(world.dir.clone());
        world.findings = errs;
        tikibase::probes::run(base, true);
        world
    });

    steps.then("all files are unchanged", |world, _ctx| {
        for (filename, original_content) in &world.original_contents {
            let current_content = testhelpers::load_file(filename, &world.dir);
            assert_eq!(&current_content, original_content);
        }
        world
    });

    steps.then_regex(r#"^file "(.*)" is unchanged$"#, |world, ctx| {
        let filename = ctx.matches.get(1).expect("no filename provided");
        let have = &testhelpers::load_file(&filename, &world.dir);
        let want = world
            .original_contents
            .get(&PathBuf::from(filename))
            .unwrap();
        assert_eq!(have, want);
        world
    });

    steps.then_regex(r#"^file "(.*)" should contain:$"#, |world, ctx| {
        // TODO: rename to want
        let expected = ctx.step.docstring().unwrap().trim_start();
        let filename = ctx.matches.get(1).expect("no filename provided");
        // TODO: rename to have
        let actual = testhelpers::load_file(&filename, &world.dir);
        assert_eq!(actual, expected);
        world
    });

    steps.then("it prints:", |world, ctx| {
        let expected: Vec<&str> = ctx.step.docstring().unwrap().trim().split("\n").collect();
        assert_eq!(&world.findings, &expected);
        world
    });

    steps.then("it prints nothing", |world, _ctx| {
        assert_eq!(world.findings, Vec::<String>::new());
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
