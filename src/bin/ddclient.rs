use clap::{App, AppSettings, Arg, SubCommand};
use ddclient::client::{cf::Cloudflare, Client};
use ddclient::ip::get_addr;
use failure::Error;

const ARG_DOMAIN: &str = "DOMAIN";
const ARG_ENDPOINT: &str = "endpoint";

const ARG_CF_EMAIL: &str = "email";
const ARG_CF_KEY: &str = "key";
const ARG_CF_TOKEN: &str = "token";

const CMD_CLOUDFLARE: &str = "cf";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let domain_arg = Arg::with_name(ARG_DOMAIN)
        .help("Domain name to update records for")
        .required(true);

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name(ARG_ENDPOINT)
                .long(ARG_ENDPOINT)
                .short("n")
                .help("Endpoint to use to determine the IP address")
                .default_value("https://api.ipify.org"),
        )
        .subcommand(
            SubCommand::with_name(CMD_CLOUDFLARE)
                .about("Cloudflare DNS via API email/key pair or token")
                .arg(&domain_arg)
                .arg(
                    Arg::with_name(ARG_CF_EMAIL)
                        .long(ARG_CF_EMAIL)
                        .short("e")
                        .env("DDCLIENT_CF_API_EMAIL")
                        .requires(ARG_CF_KEY)
                        .required_unless(ARG_CF_TOKEN)
                        .empty_values(false)
                        .help("Cloudflare account e-mail address"),
                )
                .arg(
                    Arg::with_name(ARG_CF_KEY)
                        .long(ARG_CF_KEY)
                        .short("k")
                        .env("DDCLIENT_CF_API_KEY")
                        .requires(ARG_CF_EMAIL)
                        .required_unless(ARG_CF_TOKEN)
                        .hide_default_value(true)
                        .empty_values(false)
                        .help("Cloudflare account Global API Key"),
                )
                .arg(
                    Arg::with_name(ARG_CF_TOKEN)
                        .long(ARG_CF_TOKEN)
                        .short("t")
                        .env("DDCLIENT_CF_API_TOKEN")
                        .conflicts_with(ARG_CF_EMAIL)
                        .conflicts_with(ARG_CF_KEY)
                        .required_unless_one(&[ARG_CF_EMAIL, ARG_CF_KEY])
                        .hide_default_value(true)
                        .help("Cloudflare account API token"),
                ),
        )
        .get_matches();

    if let (cmd, Some(sub_matches)) = matches.subcommand() {
        let endpoint = matches
            .value_of(ARG_ENDPOINT)
            .expect("could not get endpoint");
        let addr_handle = get_addr(endpoint);

        let domain = sub_matches
            .value_of(ARG_DOMAIN)
            .expect("no domain specified");

        let mut client = match cmd {
            CMD_CLOUDFLARE => {
                let email = sub_matches.value_of(ARG_CF_EMAIL).unwrap();
                let key = sub_matches.value_of(ARG_CF_KEY).unwrap();

                Cloudflare::new(email, key)?
            }
            _ => panic!("could not build client"),
        };

        let check_handle = client.check(domain);

        match tokio::try_join!(addr_handle, check_handle) {
            Ok((addr, _)) => Ok(client.update(domain, addr).await?),
            Err(err) => Err(err),
        }
    } else {
        Err(failure::err_msg("invalid command"))
    }
}
