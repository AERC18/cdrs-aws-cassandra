use cdrs::authenticators::{StaticPasswordAuthenticator};
use cdrs::cluster::session::{new_ssl, Session};
use cdrs::cluster::{ClusterSslConfig, NodeSslConfigBuilder, SslConnectionPool};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;
use cdrs::query_values;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;

use cdrs_helpers_derive::{TryFromRow};

use std::env;
use std::time::Duration;

use openssl::ssl::{SslConnector, SslMethod};

use uuid::Uuid;


type CurrentSession = Session<RoundRobin<SslConnectionPool<StaticPasswordAuthenticator>>>;

#[derive(Clone, Debug, TryFromRow, PartialEq)]
struct TableTestStruct {
    user_id: Uuid,
    description: String
}

fn main() {
    let session = create_session();
    create_ks(&session);
    create_table(&session);
    insert(&session);
    query(&session);
    update(&session);
    query(&session);
    delete(&session);
}

fn create_session() -> CurrentSession {
    // ------------- Get the cassandra URI and ssl cert path from CASSANDRA_URI and CASSANDRA_SSL_CERT_PATH env vars ---------------------
    let cassandra_uri: String = env::var("CASSANDRA_URI").ok().unwrap();
    let cassandra_ssl_cert_path: String = env::var("CASSANDRA_SSL_CERT_PATH").ok().unwrap();
    // ------------- Authenticator -----------------------
    let user: String = env::var("CASSANDRA_USER").ok().unwrap();
    let password: String = env::var("CASSANDRA_PASSWORD").ok().unwrap();
    let auth = StaticPasswordAuthenticator::new(&user, &password);
    // ------------- SSL connector -----------------------
    let mut ssl_connector_builder = SslConnector::builder(SslMethod::tls()).unwrap();
    ssl_connector_builder.set_ca_file(cassandra_ssl_cert_path).unwrap();
    
    let ssl_connector = ssl_connector_builder.build();
    let node = NodeSslConfigBuilder::new(&cassandra_uri, auth, ssl_connector)
        .connection_timeout(Duration::from_secs(15))
        .max_size(1)
        .min_idle(Some(1))
        .build();

    let cluster_ssl_config = ClusterSslConfig(vec![node]);
    let no_compression = new_ssl(&cluster_ssl_config, RoundRobin::new()).expect("session should be created");
    println!("Connected to cassandra: {}", cassandra_uri);
    no_compression
}

fn create_ks(session: &CurrentSession) {
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS aws_cassandra_test WITH REPLICATION ={ 'class' : 'SimpleStrategy', 'replication_factor' : 1};";
    session.query(create_ks).expect("Keyspace create error");
}

fn create_table(session: &CurrentSession) {
    let create_table_cql = "CREATE TABLE IF NOT EXISTS aws_cassandra_test.table_test(
        user_id UUID,
        description text,
        date timestamp,
        PRIMARY KEY (user_id)
        ); 
        ";
        session
        .query(create_table_cql)
        .expect("Table creation error");
}

fn insert(session: &CurrentSession) {
    println!("Inserting......");
    const INSERT: &'static str = "INSERT INTO aws_cassandra_test.table_test (user_id, description, date) VALUES (?, ?, toTimeStamp(now()));
";
    let user_id: Uuid = Uuid::parse_str("534a87db-df22-48eb-901b-4fac9c392954").unwrap();
    let description = "Some description";

    let values = query_values!(user_id, description);
    session.query_with_values(INSERT, values).unwrap();
}

fn query(session: &CurrentSession) {
    println!("Querying......");
    let user_id: Uuid = Uuid::parse_str("534a87db-df22-48eb-901b-4fac9c392954").unwrap();
    const SELECT: &'static str = "SELECT * FROM aws_cassandra_test.table_test WHERE user_id = ?;";
    let values = query_values!(user_id);

    let rows = session
            .query_with_values(SELECT, values)
            .expect("Query")
            .get_body()
            .expect("GetBody")
            .into_rows()
            .expect("Into Rows");
    
    for row in rows {
        let my_row: TableTestStruct = TableTestStruct::try_from_row(row).expect("Into table test struct failed.");
        println!("Query result: {:?}", my_row);    
    }

}

fn update(session: &CurrentSession) {
    println!("Updating......");
    let user_id: Uuid = Uuid::parse_str("534a87db-df22-48eb-901b-4fac9c392954").unwrap();
    const UPDATE: &'static str = "UPDATE aws_cassandra_test.table_test SET description = ? WHERE user_id = ?;";
    let new_description = "Updated description.";
    session
        .query_with_values(UPDATE, query_values!(new_description, user_id))
        .expect("update");
}

fn delete(session: &CurrentSession) {
    println!("Deleting......");
    const DELETE: &'static str = "DELETE FROM aws_cassandra_test.table_test WHERE user_id = ?;";
    let user_id: Uuid = Uuid::parse_str("534a87db-df22-48eb-901b-4fac9c392954").unwrap();
    let values = query_values!(user_id);
    session.query_with_values(DELETE, values).expect("Deletion error.");
}