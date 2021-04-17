use cucumber_rust::{async_trait, Context, Cucumber, Steps, World};
use rand::{distributions::Alphanumeric, Rng};
use std::convert::Infallible;
use std::fs;
use std::io::prelude::*;

pub struct MyWorld {
    pub dir: String,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        let rand: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let dir = format!("./tmp/{}", &rand);
        if let Err(e) = fs::create_dir_all(&dir) {
            panic!("Cannot create root dir for World: {}", e)
        }
        Ok(MyWorld { dir })
    }
}

fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given_regex(r#"^file "(.*)" with content:$"#, |world, ctx| {
        let filepath = format!("{}/{}", world.dir, &ctx.matches[1]);
        let content = ctx.step.docstring().unwrap().trim_start();
        let mut file = fs::File::create(filepath).unwrap();
        file.write_all(content.as_bytes()).unwrap();
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

    steps.when("checking", |world, _ctx| {
        println!("checking");
        world
    });

    steps
}

#[tokio::main]
async fn main() {
    let pool = "the pool";

    Cucumber::<MyWorld>::new()
        .features(&["./features"])
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
