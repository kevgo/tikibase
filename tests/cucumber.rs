use ahash::AHashMap;
use big_s::S;
use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use tikibase::input::Command;
use tikibase::{self, test, Messages};

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct MyWorld {
  /// the directory in which the Tikibase under test is located
  pub dir: String,

  /// the result of the Tikibase run
  pub output: Messages,

  /// content of the files before the Tikibase command ran
  pub original_contents: AHashMap<String, String>,
}

impl MyWorld {
  fn new() -> Self {
    Self {
      dir: test::tmp_dir(),
      output: Messages::default(),
      original_contents: AHashMap::new(),
    }
  }
}

#[given(expr = "file {string} with content:")]
fn file_with_content(world: &mut MyWorld, step: &Step, filename: String) {
  let content = step.docstring.as_ref().unwrap().trim();
  test::create_file(&filename, &content, &world.dir);
  world.original_contents.insert(filename, content.into());
}

#[given(expr = "file {string}")]
fn file(world: &mut MyWorld, filename: String) {
  test::create_file(&filename, "content", &world.dir);
}

#[when("checking")]
fn checking(world: &mut MyWorld) {
  world.output = tikibase::run(Command::Check, &world.dir);
}

#[when("doing a pitstop")]
fn doing_a_pitstop(world: &mut MyWorld) {
  world.output = tikibase::run(Command::P, &world.dir);
}

#[when("fixing")]
fn fixing(world: &mut MyWorld) {
  world.output = tikibase::run(Command::Fix, &world.dir);
}

#[when("initializing")]
fn initializing(world: &mut MyWorld) {
  world.output = tikibase::run(Command::Init, &world.dir);
}

#[then("all files are unchanged")]
fn all_files_unchanged(world: &mut MyWorld) {
  for (filename, original_content) in &world.original_contents {
    let current_content = test::load_file(filename, &world.dir);
    pretty::assert_eq!(&current_content.trim(), original_content);
  }
}

#[then(expr = "file {string} is unchanged")]
fn file_is_unchanged(world: &mut MyWorld, filename: String) {
  let have = test::load_file(&filename, &world.dir);
  let want = world.original_contents.get(&filename).unwrap();
  pretty::assert_eq!(have.trim(), want);
}

#[then(expr = "file {string} should contain:")]
fn file_should_contain(world: &mut MyWorld, step: &Step, filename: String) {
  let want = step.docstring.as_ref().unwrap();
  let have = test::load_file(&filename, &world.dir);
  pretty::assert_eq!(have.trim(), want.trim());
}

#[then("it prints:")]
fn it_prints(world: &mut MyWorld, step: &Step) {
  let mut have = S("");
  for message in &world.output.issues {
    have.push_str(message.to_text().trim());
    have.push_str("\n");
  }
  for message in &world.output.fixes {
    have.push_str(message.to_text().trim());
    have.push_str("\n");
  }
  let want = step.docstring.as_ref().unwrap();
  pretty::assert_eq!(have.trim(), want.trim());
}

#[then("it prints nothing")]
fn it_prints_nothing(world: &mut MyWorld) {
  assert!(world.output.is_empty())
}

#[then("it finds no issues")]
fn it_finds_no_issues(world: &mut MyWorld) {
  pretty::assert_eq!(world.output, Messages::default());
}

#[then(expr = "the exit code is {int}")]
fn the_exit_code_is(world: &mut MyWorld, exit_code: u8) {
  assert_eq!(world.output.exit_code, exit_code);
}

fn main() {
  futures::executor::block_on(MyWorld::run("features"));
}
