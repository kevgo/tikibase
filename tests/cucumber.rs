use crate::tikibase::testhelpers;
use ahash::AHashMap;
use async_trait::async_trait;
use cucumber::{gherkin::Step, given, then, when, World, WorldInit};
use std::convert::Infallible;
use std::path::PathBuf;
use tikibase::cli::Command;
use tikibase::testhelpers::{create_file, load_file, tmp_dir};
use tikibase::{self, Message, Messages};

#[derive(Debug, WorldInit)]
pub struct MyWorld {
    /// the directory in which the Tikibase under test is located
    pub dir: PathBuf,

    /// the result of the Tikibase run
    pub output: Messages,

    /// content of the files before the Tikibase command ran
    pub original_contents: AHashMap<PathBuf, String>,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(MyWorld {
            dir: testhelpers::tmp_dir(),
            output: Messages::default(),
            original_contents: AHashMap::new(),
        })
    }
}

#[given(regex = r#"^file "(.*)" with content:$"#)]
fn file_with_content(world: &mut MyWorld, step: &Step, filename: String) {
    let content = step.docstring.as_ref().unwrap().trim();
    testhelpers::create_file(&filename, &content, &world.dir);
    world
        .original_contents
        .insert(PathBuf::from(filename), content.into());
}

#[given(regex = r#"^file "(.*)"$"#)]
fn file(world: &mut MyWorld, filename: String) {
    testhelpers::create_file(&filename, "content", &world.dir);
}

#[when("checking")]
fn checking(world: &mut MyWorld) {
    world.output = tikibase::run(Command::Check, world.dir.clone());
}

#[when("doing a pitstop")]
fn doing_a_pitstop(world: &mut MyWorld) {
    world.output = tikibase::run(Command::Pitstop, world.dir.clone());
}

#[when("fixing")]
fn fixing(world: &mut MyWorld) {
    world.output = tikibase::run(Command::Fix, world.dir.clone());
}

#[then("all files are unchanged")]
fn all_files_unchanged(world: &mut MyWorld) {
    for (filename, original_content) in &world.original_contents {
        let current_content = testhelpers::load_file(filename, &world.dir);
        assert_eq!(&current_content.trim(), original_content);
    }
}

#[then(regex = r#"^file "(.*)" is unchanged$"#)]
fn file_is_unchanged(world: &mut MyWorld, filename: String) {
    let have = testhelpers::load_file(&filename, &world.dir);
    let want = world
        .original_contents
        .get(&PathBuf::from(filename))
        .unwrap();
    assert_eq!(have.trim(), want);
}

#[then(regex = r#"^file "(.*)" should contain:$"#)]
fn file_should_contain(world: &mut MyWorld, step: &Step, filename: String) {
    let want = step.docstring.as_ref().unwrap();
    let have = testhelpers::load_file(&filename, &world.dir);
    assert_eq!(have.trim(), want.trim());
}

#[then("it prints:")]
fn it_prints(world: &mut MyWorld, step: &Step) {
    // TODO: make both of these strings
    let mut have = String::new();
    for message in &world.output.messages {
        have.push_str(message.to_text().trim());
        have.push_str("\n");
    }
    have = have.trim().to_string();
    let want = step.docstring.as_ref().unwrap().trim();
    assert_eq!(have, want);
}

#[then("it prints nothing")]
fn it_prints_nothing(world: &mut MyWorld) {
    assert_eq!(world.output.messages, Vec::<Message>::new());
}

#[then("it finds no issues")]
fn it_finds_no_issues(world: &mut MyWorld) {
    assert_eq!(world.output.messages, Vec::<Message>::new());
    assert_eq!(world.output.exit_code, 0);
}

#[then(regex = "^the exit code is (\\d+)$")]
fn the_exit_code_is(world: &mut MyWorld, exit_code: i32) {
    assert_eq!(world.output.exit_code, exit_code);
}

fn main() {
    futures::executor::block_on(MyWorld::run("features"));
}
