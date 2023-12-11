/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sagemaker::{config::Region, meta::PKG_VERSION, Client, Error, operation::create_model_card::CreateModelCardOutput, types::ModelCardStatus};
use clap::Parser;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

#[derive(Debug, Parser)]
struct Opt {
    /// The name of the model.
    #[structopt(short, long)]
    model_card_name: String,

    /// The contents of the card.
    #[structopt(short, long)]
    contents: String,

    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Creates a model.
// snippet-start:[sagemaker.rust.create-model]
async fn create_model_card(client: &Client, model_card_name: &String, card_contents: &String) -> Result<CreateModelCardOutput, Error> {
    println!("{}", card_contents);
    let response = client.create_model_card()
        .model_card_name(model_card_name)
        .content(card_contents)
        .model_card_status(ModelCardStatus::Pendingreview)
        .send().await?;

    Ok(response)

}
// snippet-end:[sagemaker.rust.create-model]

/// Lists the name, status, and type of your SageMaker instances in the Region.
/// /// # Arguments
///
/// * `[-r REGION]` - The Region in which the client is created.
///    If not supplied, uses the value of the **AWS_REGION** environment variable.
///    If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let Opt { model_card_name, contents, region, verbose } = Opt::parse();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    println!();

    if verbose {
        println!("SageMaker client version: {}", PKG_VERSION);
        println!(
            "Region:                    {}:",
            region_provider.region().await.unwrap().as_ref()
        );
        println!();
    }

    println!("Reading model card from {}", contents);
    let path = Path::new(&contents);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut card_contents = String::new();
    match file.read_to_string(&mut card_contents) {
        Err(why) => panic!("could not read {}: {}", display, why),
        Ok(_) => print!("{} creating model with card with contents:\n{}", display, card_contents),
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let result = create_model_card(&client, &model_card_name, &card_contents).await;
    println!("{:?}", result.unwrap());
//    println!("{}", result.unwrap_err());
    Ok(())
}
