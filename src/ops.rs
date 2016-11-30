extern crate rusoto;

use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::ec2::{Ec2Client, DescribeInstancesRequest,Filter};

pub fn get_ec2_ips(pattern: &str) -> Result<Vec<String>, rusoto::ec2::DescribeInstancesError> {
    let provider = DefaultCredentialsProvider::new().unwrap();
    let client = Ec2Client::new(provider, Region::UsEast1);
    let opt = DescribeInstancesRequest {
        dry_run: Some(false),
        instance_ids: None,
        max_results: None,
        filters: Some(vec![Filter {
            name: Some("tag:Name".to_string()),
            values: Some(vec!["*".to_string() + pattern + &"*".to_string()]),
        }]),
        next_token: None
    };


    let result = client.describe_instances(&opt);


    match result {
        Ok(result) => {
            let r = result.reservations.unwrap();
            let v: Vec<String> = r.into_iter().map(|o| {
                let ref a = o.instances.unwrap()[0];
                match a.private_ip_address {
                    Some(ref ip) => {
                        //println!("{}", ip);
                        ip.to_string()
                    }
                    _ => "abc".to_string()
                }
            }).collect();
            Ok(v)
        }

        Err(error) => {
            Err(error)
        }
    }
}
