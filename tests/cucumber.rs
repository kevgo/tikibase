// TODO: find all features like this and see if we really need them
#![feature(destructuring_assignment)]

use crate::tikibase::testhelpers::{create_file, load_file, tmp_dir};
use ahash::AHashMap;
use cucumber_rust::{async_trait, Cucumber, Steps, World};
use std::io;
use std::path::PathBuf;
use tikibase;
use tikibase::Command;

pub struct MyWorld {
    /// the directory in which the Tikibase under test is located
    pub dir: PathBuf,

    /// the exit code of the Tikibase run
    pub exitcode: i32,

    /// results of the Tikibase run
    pub findings: Vec<String>,

    /// content of the files before the Tikibase command ran
    pub original_contents: AHashMap<PathBuf, String>,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = io::Error;
    async fn new() -> Result<Self, io::Error> {
        Ok(MyWorld {
            dir: tmp_dir(),
            exitcode: 0,
            findings: Vec::new(),
            original_contents: AHashMap::new(),
        })
    }
}

fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(r#"^file "(.*)" with content:$"#, |mut world, ctx| {
        let filename = ctx.matches.get(1).expect("no filename provided");
        let content = ctx.step.docstring().unwrap().trim_start();
        create_file(filename, content, &world.dir);
        world
            .original_contents
            .insert(PathBuf::from(filename), content.into());
        world
    });

    steps.given_regex(r#"^file "(.*)"$"#, |world, ctx| {
        let filename = ctx.matches.get(1).expect("no filename provided");
        create_file(filename, "content", &world.dir);
        world
    });

    steps.when("checking", |mut world, _ctx| {
        let result = tikibase::process(&Command::Check, world.dir.clone());
        (world.findings, world.exitcode) = result;
        world
    });

    steps.when("doing a pitstop", |mut world, _ctx| {
        (world.findings, world.exitcode) = tikibase::process(&Command::Pitstop, world.dir.clone());
        world
    });

    steps.when("fixing", |mut world, _ctx| {
        (world.findings, world.exitcode) = tikibase::process(&Command::Fix, world.dir.clone());
        world
    });

    steps.then("all files are unchanged", |world, _ctx| {
        for (filename, original_content) in &world.original_contents {
            let current_content = load_file(filename, &world.dir);
            assert_eq!(&current_content, original_content);
        }
        world
    });

    steps.then_regex(r#"^file "(.*)" is unchanged$"#, |world, ctx| {
        let filename = ctx.matches.get(1).expect("no filename provided");
        let have = &load_file(&filename, &world.dir);
        let want = world
            .original_contents
            .get(&PathBuf::from(filename))
            .unwrap();
        assert_eq!(have, want);
        world
    });

    steps.then_regex(r#"^file "(.*)" should contain:$"#, |world, ctx| {
        let want = ctx.step.docstring().unwrap().trim_start();
        let filename = ctx.matches.get(1).expect("no filename provided");
        let have = load_file(&filename, &world.dir);
        assert_eq!(have, want);
        world
    });

    steps.then("it prints:", |world, ctx| {
        let have: Vec<&str> = world
            .findings
            .iter()
            .map(|line| line.split('\n'))
            .flatten()
            .collect();
        let want: Vec<&str> = ctx.step.docstring().unwrap().trim().split("\n").collect();
        assert_eq!(have, want);
        world
    });

    steps.then("it prints nothing", |world, _ctx| {
        assert_eq!(world.findings, Vec::<String>::new());
        world
    });

    steps.then("it finds no issues", |world, _ctx| {
        let expected: Vec<&str> = Vec::new();
        assert_eq!(world.findings, expected);
        assert_eq!(world.exitcode, 0);
        world
    });

    steps.then_regex("^the exit code is (\\d+)$", |world, ctx| {
        let want: i32 = ctx
            .matches
            .get(1)
            .expect("no exit code provided")
            .parse()
            .unwrap();
        assert_eq!(world.exitcode, want);
        world
    });

    steps
}

#[tokio::main]
async fn main() {
    Cucumber::<MyWorld>::new()
        .features(&["./features"])
        .steps(steps())
        .cli() // parse command line arguments
        .run_and_exit()
        .await
}
