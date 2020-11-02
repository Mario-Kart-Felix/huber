use clap::{App, Arg, ArgMatches};

use crate::cmd::CommandTrait;
use huber_common::config::Config;
use huber_common::di::{DIContainer, DIObjectTrait, MutableRc};
use huber_common::result::Result;
use tokio::runtime::Runtime;

pub(crate) const CMD_NAME: &str = "uninstall";

pub(crate) struct UninstallCmd {
    container: MutableRc<DIContainer>,
}

impl DIObjectTrait for UninstallCmd {
    fn new_for_di(container: MutableRc<DIContainer>) -> Self {
        Self { container }
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for UninstallCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Uninstall package").arg(
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
