# Aws Cassandra Rust connection example

This example shows how to connect your Rust application with the Amazon Managed Apache Cassandra Service now called Amazon Keyspace.

## Getting Started 

### Prerequisites
1. The stable installation of Rust:

```
rustup 1.21.1 (7832b2ebe 2019-12-20) or greater
cargo 1.40.0 (bc8e4c8be 2019-11-22) or greater 
rustc 1.40.0 (73528e339 2019-12-16) or greater
```
2. Amazon Managed Apache Cassandra Service deployment
[Getting Started](https://docs.aws.amazon.com/mcs/latest/devguide/getting-started.html)

3. Set this environment variables: 

```bash
export CASSANDRA_URI="cassandra.us-east-2.amazonaws.com:9142"
export CASSANDRA_SSL_CERT_PATH="certs/AmazonRootCA1.pem"
export CASSANDRA_USER="YOUR CASSANDRA USER"
export CASSANDRA_PASSWORD="YOUR CASSANDRA PASSWORD"
```

The CASSANDRA_URI env var depends on the region where your Cassandra is deployed, please look at the AWS documentation: [AWS MCS](https://docs.aws.amazon.com/mcs/latest/devguide/cqlsh.html) - Using cqlsh and Cassandra Drivers

### Installation
Go inside the project path and just run: 
```bash
cargo run
```