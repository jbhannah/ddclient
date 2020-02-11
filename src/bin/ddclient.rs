use clap::{App, AppSettings, Arg, SubCommand};
use ddclient::ip::get_addr;

const ARG_DOMAIN: &str = "DOMAIN";
const ARG_ENDPOINT: &str = "endpoint";

const ARG_CF_EMAIL: &str = "email";
const ARG_CF_KEY: &str = "key";
const ARG_CF_TOKEN: &str = "token";

const CMD_CLOUDFLARE: &str = "cf";

#[tokio::main]
async fn main() {
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

    let endpoint = matches
        .value_of(ARG_ENDPOINT)
        .expect("could not get endpoint");
    if let Ok(addr) = get_addr(endpoint).await {
        println!("{}", addr);
    }
}
