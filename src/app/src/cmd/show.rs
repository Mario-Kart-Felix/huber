use clap::{App, Arg, ArgMatches};

use crate::cmd::CommandTrait;
use huber_common::config::Config;
use huber_common::di::{DIContainer, DIObjectTrait, MutableRc};
use tokio::runtime::Runtime;
use anyhow::Result;

pub(crate) const CMD_NAME: &str = "show";

pub(crate) struct ShowCmd {
    container: MutableRc<DIContainer>,
}

impl DIObjectTrait for ShowCmd {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self { container }
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for ShowCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Show installed package").arg(
            Arg::with_name("name")
                .help("Package name")
                .required(true)
                .takes_value(true),
        )
    }

    fn run(&self, runtime: &Runtime, config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let name = matches.value_of("name").unwrap();

        unimplemented!()
    }
}
