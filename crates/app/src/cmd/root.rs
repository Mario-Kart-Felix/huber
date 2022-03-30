use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use simpledi_rs::di::DIContainer;

use huber_common::model::config::Config;
use huber_common::result::Result;

use crate::cmd::config::{ARG_GITHUB_KEY, ARG_GITHUB_TOKEN, ARG_LOG_LEVEL, ARG_OUTPUT_TYPE};
use crate::cmd::{CommandAsyncTrait, CommandTrait};

#[derive(Debug)]
pub(crate) struct RootCmd;

unsafe impl Send for RootCmd {}

unsafe impl Sync for RootCmd {}

impl RootCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'help> CommandTrait<'help> for RootCmd {
    fn app(&self) -> Command<'help> {
        Command::new(env!("CARGO_BIN_NAME"))
            .long_version(env!("HUBER_LONG_VERSION"))
            .about("Huber, simplify github package management")
            .args(&[
                Arg::new(ARG_LOG_LEVEL)
                    .value_name("string")
                    .short('l')
                    .long(ARG_LOG_LEVEL)
                    .help("Log level")
                    .takes_value(true)
                    .global(true)
                    .default_value("error")
                    .possible_values(&["off", "error", "warn", "info", "debug", "trace"]),
                Arg::new(ARG_OUTPUT_TYPE)
                    .value_name("string")
                    .short('o')
                    .long(ARG_OUTPUT_TYPE)
                    .help("Output format")
                    .takes_value(true)
                    .global(true)
                    .default_value("console")
                    .possible_values(&["console", "json", "yaml"]),
                Arg::new(ARG_GITHUB_TOKEN)
                    .value_name("string")
                    .short('t')
                    .long(ARG_GITHUB_TOKEN)
                    .env("GITHUB_TOKEN")
                    .help("Github token, used for authorized access instead of limited public access")
                    .takes_value(true)
                    .global(true),
                Arg::new(ARG_GITHUB_KEY)
                    .value_name("string")
                    .short('k')
                    .long(ARG_GITHUB_KEY)
                    .env("GITHUB_KEY")
                    .help("Github SSH private key path for authenticating public/private github repository access. This is required if you connect github w/ SSH instead of https")
                    .takes_value(true)
                    .global(true),
            ])
    }
}

#[async_trait]
impl CommandAsyncTrait for RootCmd {
    async fn run(
        &self,
        _config: &Config,
        _container: &DIContainer,
        _matches: &ArgMatches,
    ) -> Result<()> {
        unimplemented!()
    }
}
