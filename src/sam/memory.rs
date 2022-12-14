// ███████     █████     ███    ███    
// ██         ██   ██    ████  ████    
// ███████    ███████    ██ ████ ██    
//      ██    ██   ██    ██  ██  ██    
// ███████ ██ ██   ██ ██ ██      ██ ██ 
// Copyright 2021-2022 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rouille::Response;
use serde::{Serialize, Deserialize};
use std::env;
use std::fmt;
use std::str::FromStr;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_postgres::{Error, Row};
use std::path::Path;

// store application version as a const
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub postgres: PostgresServer,
    pub version_installed: String
}
impl Config {
    pub fn new() -> Config {
        Config{
            postgres: PostgresServer::new(),
            version_installed: VERSION.unwrap_or("unknown").to_string()
        }
    }
    pub async fn init(&self){

        match self.create_db().await{
            Ok(_) => log::info!("Database created successfully"),
            Err(e) => log::error!("failed to create database: {}", e),
        }

        match self.build_tables().await{
            Ok(_) => log::info!("Tables created successfully"),
            Err(e) => log::error!("failed to create tables: {}", e),
        }
    

        let _config = self.clone();
        thread::spawn(move || {

            rouille::start_server(format!("0.0.0.0:8000").as_str(), move |request| {
            
                match crate::sam::http::handle(request){
                    Ok(request) => {
                        return request;
                    },
                    Err(err) => {
                        log::error!("HTTP_ERROR: {}", err);
                        return Response::empty_404();
                    }
                }

            });
        });
    }
    pub async fn build_tables(&self) -> Result<(), Error>{
    
        let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
        builder.set_verify(SslVerifyMode::NONE);
        let connector = MakeTlsConnector::new(builder.build());
    
        let (client, connection) = tokio_postgres::connect(format!("postgresql://{}:{}@{}/{}?sslmode=prefer", &self.postgres.username, &self.postgres.password, &self.postgres.address, &self.postgres.db_name).as_str(), connector).await?;
        
        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                log::error!("connection error: {}", e);
            }
        });

        // TODO - Build tables
        let c1 = Self::build_table(client, CachedWikipediaSummary::sql_table_name(), CachedWikipediaSummary::sql_build_statement(), CachedWikipediaSummary::migrations()).await;
        let c2 = Self::build_table(c1, Human::sql_table_name(), Human::sql_build_statement(), Human::migrations()).await;
        let c3 = Self::build_table(c2, HumanFaceEncoding::sql_table_name(), HumanFaceEncoding::sql_build_statement(), HumanFaceEncoding::migrations()).await;
        let c4 = Self::build_table(c3, Location::sql_table_name(), Location::sql_build_statement(), Location::migrations()).await;
        let c5 = Self::build_table(c4, Room::sql_table_name(), Room::sql_build_statement(), Room::migrations()).await;
        let c6 = Self::build_table(c5, Service::sql_table_name(), Service::sql_build_statement(), Service::migrations()).await;
        let c7 = Self::build_table(c6, Thing::sql_table_name(), Thing::sql_build_statement(), Thing::migrations()).await;
        let c8 = Self::build_table(c7, Observation::sql_table_name(), Observation::sql_build_statement(), Observation::migrations()).await;
        let c9 = Self::build_table(c8, Setting::sql_table_name(), Setting::sql_build_statement(), Setting::migrations()).await;
        let c10 = Self::build_table(c9, WebSessions::sql_table_name(), WebSessions::sql_build_statement(), WebSessions::migrations()).await;
        let c11 = Self::build_table(c10, StorageLocation::sql_table_name(), StorageLocation::sql_build_statement(), StorageLocation::migrations()).await;
        let c12 = Self::build_table(c11, FileStorage::sql_table_name(), FileStorage::sql_build_statement(), FileStorage::migrations()).await;
        let _c13 = Self::build_table(c12, Notification::sql_table_name(), Notification::sql_build_statement(), Notification::migrations()).await;

        
        return Ok(());
    }    
    pub async fn create_db(&self) -> Result<(), Error>{

        let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
        builder.set_verify(SslVerifyMode::NONE);
        let connector = MakeTlsConnector::new(builder.build());
    
        let (client, connection) = tokio_postgres::connect(format!("postgresql://{}:{}@{}?sslmode=prefer", &self.postgres.username, &self.postgres.password, &self.postgres.address).as_str(), connector).await?;
        
        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                log::error!("connection error: {}", e);
            }
        });
        
        client.batch_execute(format!("CREATE DATABASE {}", self.postgres.db_name).as_str()).await?;
    
        Ok(())
    }
    pub async fn build_table(client: tokio_postgres::Client, table_name: String, build_statement: &str, migrations: Vec<&str>) -> tokio_postgres::Client{
        let db = client.batch_execute(build_statement.clone()).await;
        match db {
            Ok(_v) => log::info!("POSTGRES: CREATED '{}' TABLE", table_name.clone()),
            Err(e) => log::error!("POSTGRES: {:?}", e),
        }
        for migration in migrations {
            let migrations_db = client.batch_execute(migration).await;
            match migrations_db {
                Ok(_v) => log::info!("POSTGRES: MIGRATED '{}' TABLE", table_name.clone()),
                Err(e) => log::error!("POSTGRES: {:?}", e),
            }
        }
        return client;
    }
    pub fn destroy_row(oid: String, table_name: String) -> Result<bool, Error>{
        let mut client = Config::client()?;

        let _destroy_rows = client.query(format!("DELETE FROM {} WHERE oid = '{}' ", table_name, oid).as_str(), &[]).unwrap();
        let rows = client.query(format!("SELECT * FROM {} WHERE oid = '{}'", table_name, oid).as_str(), &[]).unwrap();
    
        if rows.len() == 0 {
           
            return Ok(true);
        
        } else {
            return Ok(false);
        
        }
    
    }
    pub async fn nuke_async() -> Result<(), Error>{
        let config = crate::sam::memory::Config::new();
        // Get a copy of the master key and postgres info
        let postgres = config.postgres.clone();
    
    
        // Build SQL adapter that skips verification for self signed certificates
        let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
        builder.set_verify(SslVerifyMode::NONE);
    
        // Build connector with the adapter from above
        let connector = MakeTlsConnector::new(builder.build());
    
        let (client, connection) = tokio_postgres::connect(format!("postgresql://{}:{}@{}/{}?sslmode=prefer", &postgres.username, &postgres.password, &postgres.address, &postgres.db_name).as_str(), connector).await?;
        
        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                log::error!("connection error: {}", e);
            }
        });
        
    
    
        
        client.batch_execute("DO $$ 
        DECLARE 
        r RECORD;
        BEGIN
            FOR r IN 
            (
                SELECT table_name 
                FROM information_schema.tables 
                WHERE table_schema=current_schema()
            ) 
            LOOP
            EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.table_name) || ' CASCADE';
            END LOOP;
        END $$ ;").await?;
    
    
        client.query("DO $$ 
        DECLARE 
        r RECORD;
        BEGIN
            FOR r IN 
            (
                SELECT table_name 
                FROM information_schema.tables 
                WHERE table_schema=current_schema()
            ) 
            LOOP
            EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.table_name) || ' CASCADE';
            END LOOP;
        END $$ ;", &[]).await?;
    
        Ok(())
    }
    pub fn pg_select(table_name: String, coulmns: Option<String>, limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<String>, Error>{
      
        let mut client = Config::client()?;

        let mut execquery = format!("SELECT * FROM {}", table_name);
    
        if coulmns.is_some(){
            execquery  = format!("SELECT {} FROM {}", coulmns.clone().unwrap(), table_name);
        }
    
    
        match query.clone() {
            Some(pg_query) => {
    
                let mut counter = 1;
    
                for col in pg_query.query_coulmns{
                    if counter == 1 {
                        execquery = format!("{} {} {} ${}", execquery, "WHERE", col, counter);
                    } else {
                        execquery = format!("{} {} ${}", execquery, col, counter);
                    }
                    counter = counter + 1;
                }
            },
            None => {
    
            }
        }
    
        match order {
            Some(order_val) => {
                execquery = format!("{} {} {}", execquery, "ORDER BY", order_val);
            },
            None => {
                execquery = format!("{} {} {}", execquery, "ORDER BY", "id DESC");
            }
        }
        match limit {
            Some(limit_val) => {
                execquery = format!("{} {} {}", execquery, "LIMIT", limit_val);
            },
            None => {}
        }
        match offset {
            Some(offset_val) => {
                execquery = format!("{} {} {}", execquery, "OFFSET", offset_val);
            },
            None => {}
        }
    
        let mut parsed_rows: Vec<String> = Vec::new();
        match query {
            Some(pg_query) => {
    
                let query_values: Vec<_> = pg_query.queries.iter().map(|x| {
                    match x {
                        PGCol::String(y) => y as &(dyn postgres::types::ToSql + Sync),
                        PGCol::Number(y) => y as &(dyn postgres::types::ToSql + Sync),
                        PGCol::Boolean(y) => y as &(dyn postgres::types::ToSql + Sync)
                    }
                }).collect();

                
                for row in client.query(execquery.as_str(), query_values.as_slice())? {
                    if table_name == CachedWikipediaSummary::sql_table_name(){
                        let j = serde_json::to_string(&CachedWikipediaSummary::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Human::sql_table_name(){
                        let j = serde_json::to_string(&Human::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == HumanFaceEncoding::sql_table_name(){
                        let j = serde_json::to_string(&HumanFaceEncoding::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Location::sql_table_name(){
                        let j = serde_json::to_string(&Location::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Notification::sql_table_name(){
                        let j = serde_json::to_string(&Notification::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Room::sql_table_name(){
                        let j = serde_json::to_string(&Room::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Service::sql_table_name(){
                        let j = serde_json::to_string(&Service::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Thing::sql_table_name(){
                        let j = serde_json::to_string(&Thing::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Observation::sql_table_name() && coulmns.is_none(){
                        let j = serde_json::to_string(&Observation::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Observation::sql_table_name() && coulmns.is_some(){
                        let j = serde_json::to_string(&Observation::from_row_lite(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Setting::sql_table_name(){
                        let j = serde_json::to_string(&Setting::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == WebSessions::sql_table_name(){
                        let j = serde_json::to_string(&WebSessions::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == StorageLocation::sql_table_name(){
                        let j = serde_json::to_string(&StorageLocation::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == FileStorage::sql_table_name() && coulmns.is_none(){
                        let j = serde_json::to_string(&FileStorage::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == FileStorage::sql_table_name() && coulmns.is_some(){
                        let j = serde_json::to_string(&FileStorage::from_row_lite(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                }
    
            },
            None => {
    
                for row in client.query(execquery.as_str(), &[])? {
                    if table_name == CachedWikipediaSummary::sql_table_name(){
                        let j = serde_json::to_string(&CachedWikipediaSummary::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Human::sql_table_name(){
                        let j = serde_json::to_string(&Human::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == HumanFaceEncoding::sql_table_name(){
                        let j = serde_json::to_string(&HumanFaceEncoding::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Location::sql_table_name(){
                        let j = serde_json::to_string(&Location::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Notification::sql_table_name(){
                        let j = serde_json::to_string(&Notification::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Room::sql_table_name(){
                        let j = serde_json::to_string(&Room::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Service::sql_table_name(){
                        let j = serde_json::to_string(&Service::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Thing::sql_table_name(){
                        let j = serde_json::to_string(&Thing::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Observation::sql_table_name() && coulmns.is_none(){
                        let j = serde_json::to_string(&Observation::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Observation::sql_table_name() && coulmns.is_some(){
                        let j = serde_json::to_string(&Observation::from_row_lite(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == Setting::sql_table_name(){
                        let j = serde_json::to_string(&Setting::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == WebSessions::sql_table_name(){
                        let j = serde_json::to_string(&WebSessions::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == StorageLocation::sql_table_name(){
                        let j = serde_json::to_string(&StorageLocation::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == FileStorage::sql_table_name() && coulmns.is_none(){
                        let j = serde_json::to_string(&FileStorage::from_row(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                    if table_name == FileStorage::sql_table_name() && coulmns.is_some(){
                        let j = serde_json::to_string(&FileStorage::from_row_lite(&row)?).unwrap();
                        parsed_rows.push(j);
                    }
                }
            }
        }
    
    
        match client.close(){
            Ok(_) => {},
            Err(e) => log::error!("Failed to close PG-SQL Client: {}", e),
        }
    
        // std::mem::drop(client);
        // std::mem::drop(connector);
    
        Ok(parsed_rows)
    }
    pub fn client() -> Result<crate::postgres::Client, Error> {
        let config = Config::new();
        
        let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
        builder.set_verify(SslVerifyMode::NONE);
        let connector = MakeTlsConnector::new(builder.build());
        return Ok(crate::postgres::Client::connect(format!("postgresql://{}:{}@{}/{}?sslmode=prefer", &config.postgres.username, &config.postgres.password, &config.postgres.address, &config.postgres.db_name).as_str(), connector)?);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedWikipediaSummary {
    pub id: i32,
    pub oid: String,
    pub topics: Vec<String>,
    pub summary: String,
    pub timestamp: i64
}
impl CachedWikipediaSummary {
    pub fn new() -> CachedWikipediaSummary {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        let topics: Vec<String> = Vec::new();
        CachedWikipediaSummary { 
            id: 0,
            oid: oid,
            topics,
            summary: String::new(),
            timestamp
        }
    }
    pub fn sql_table_name() -> String {
        return format!("cached_wikipedia_summaries")
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.cached_wikipedia_summaries (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            topics varchar NULL,
            summary varchar NULL,
            timestamp BIGINT DEFAULT 0,
            CONSTRAINT cached_wikipedia_summaries_pkey PRIMARY KEY (id));"
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
            "",
        ]
    }
    pub fn save(object: Self) -> Result<Self, Error>{
        let mut client = Config::client()?;
        
        let mut pg_query = PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(object.oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));


        // Search for OID matches
        let rows = Self::select(
            None, 
            None, 
            None, 
            Some(pg_query.clone())
        ).unwrap();

        if rows.len() == 0 {


            client.execute("INSERT INTO cached_wikipedia_summaries (oid, topics, summary, timestamp) VALUES ($1, $2, $3, $4)",
                &[&object.oid.clone(),
                &object.topics.join(","),
                &object.summary,
                &object.timestamp]
            ).unwrap();

    
            // Search for OID matches
            let rows_two = Self::select(
                None, 
                None, 
                None, 
                Some(pg_query)
            ).unwrap();

            return Ok(rows_two[0].clone());
        
        } else {
            let ads = rows[0].clone();


            // Only save if newer than stored information
            // if objec.updated_at > ads.updated_at {
                client.execute("UPDATE cached_wikipedia_summaries SET topics = $1, summary = $2, timestamp = $3 WHERE oid = $4;", 
                &[&object.topics.join(","),
                &object.summary,
                &object.timestamp,
                &ads.oid])?;
            // }

            let statement_two = client.prepare("SELECT * FROM cached_wikipedia_summaries WHERE oid = $1")?;
            let rows_two = client.query(&statement_two, &[
                &object.oid, 
            ])?;

            return Ok(Self::from_row(&rows_two[0])?);
        }
        
    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {


        let mut topics: Vec<String> = Vec::new();
        let sql_topics: Option<String> = row.get("topics");
        match sql_topics {
            Some(ts) => {
                let split = ts.split(',');
                let vec = split.collect::<Vec<&str>>();
                let mut newvec: Vec<String> = Vec::new();
                for v in vec{
                    newvec.push(v.to_string());
                }
                topics = newvec;
            },
            None => {}
        }
   


        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            topics: topics, 
            summary: row.get("summary"),
            timestamp: row.get("timestamp"),
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("cached_wikipedia_summaries"));
    }
}

// A human can have many face encodings for accuracy
// A human may or may not have an email address
// Unknown humans will be assiged a name
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Human {
    pub id: i32,
    pub oid: String,
    pub name: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub heard_count: i64,
    pub seen_count: i64,
    pub authorization_level: i64,
    pub created_at: i64,
    pub updated_at: i64
}
impl Human {
    pub fn new() -> Human {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        Human { 
            id: 0,
            oid: oid.clone(),
            name: format!("unknown-{}", oid), 
            email: None,
            password: None,
            phone_number: None,
            heard_count: 0,
            seen_count: 0,
            authorization_level: 0,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        }
    }
    pub fn sql_table_name() -> String {
        return format!("humans")
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.humans (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            name varchar NULL,
            email varchar NULL,
            password varchar NULL,
            phone_number varchar NULL,
            heard_count BIGINT NULL,
            seen_count BIGINT NULL,
            authorization_level BIGINT NULL,
            created_at BIGINT NULL,
            updated_at BIGINT NULL,
            CONSTRAINT humans_pkey PRIMARY KEY (id));"
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
            "ALTER TABLE public.humans ADD COLUMN password varchar NULL;",
            "ALTER TABLE public.humans ADD COLUMN created_at BIGINT NULL;",
            "ALTER TABLE public.humans ADD COLUMN updated_at BIGINT NULL;"
        ]
    }
    pub fn count() -> Result<i64, Error>{

        let mut client = Config::client()?;


        let execquery = format!("SELECT COUNT(*)
        FROM {}", Self::sql_table_name());


        let mut counter: i64 = 0;
        for row in client.query(execquery.as_str(), &[])? {
           counter = row.get("count");
        }

        match client.close(){
            Ok(_) => {},
            Err(e) => log::error!("failed to close connection to database: {}", e),
        }

        Ok(counter)
    }
    pub fn save(&self) -> Result<&Self, Error>{
        let mut client = Config::client()?;
        
        // Search for OID matches
        let mut pg_query = PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(self.oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));


        // Search for OID matches
        let rows = Self::select(
            None, 
            None, 
            None, 
            Some(pg_query.clone())
        ).unwrap();

        if rows.len() == 0 {


            client.execute("INSERT INTO humans (oid, name, heard_count, seen_count, authorization_level, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                &[  &self.oid.clone(),
                    &self.name,
                    &self.heard_count,
                    &self.seen_count,
                    &self.authorization_level,
                    &self.created_at,
                    &self.updated_at
                ]
            ).unwrap();

            if self.phone_number.is_some() {
                client.execute("UPDATE humans SET phone_number = $1 WHERE oid = $2;", 
                &[
                    &self.phone_number.clone().unwrap(),
                    &self.oid
                ])?;
            }

            if self.email.is_some() {
                client.execute("UPDATE humans SET email = $1 WHERE oid = $2;", 
                &[
                    &self.email.clone().unwrap(),
                    &self.oid
                ])?;
            }

            if self.password.is_some() {
                client.execute("UPDATE humans SET password = $1 WHERE oid = $2;", 
                &[
                    &self.password.clone().unwrap(),
                    &self.oid
                ])?;
            }
    
   
            
            return Ok(self);
        
         
        } else {
            // TODO Impliment Update

            let ads = rows[0].clone();


            // Only save if newer than stored information
            if self.updated_at > ads.updated_at {
                client.execute("UPDATE humans SET name = $1, heard_count = $2, seen_count = $3, authorization_level = $4, updated_at = $5 WHERE oid = $6;", 
                &[
                    &self.name,
                    &self.heard_count,
                    &self.seen_count,
                    &self.authorization_level,
                    &self.updated_at,
                    &ads.oid
                ])?;


                if self.phone_number.is_some() {
                    client.execute("UPDATE humans SET phone_number = $1 WHERE oid = $2;", 
                    &[
                        &self.phone_number.clone().unwrap(),
                        &ads.oid
                    ])?;
                }

                if self.email.is_some() {
                    client.execute("UPDATE humans SET email = $1 WHERE oid = $2;", 
                    &[
                        &self.email.clone().unwrap(),
                        &ads.oid
                    ])?;
                }

                if self.password.is_some() {
                    client.execute("UPDATE humans SET password = $1 WHERE oid = $2;", 
                    &[
                        &self.password.clone().unwrap(),
                        &self.oid
                    ])?;
                }
        
            }


            return Ok(self);

        }
        
    
   
    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {

        let sql_email: Option<String> = row.get("email");

        let sql_password: Option<String> = row.get("password");

        let sql_phone_number: Option<String> = row.get("phone_number");

        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            name: row.get("name"), 
            email: sql_email,
            password: sql_password,
            phone_number: sql_phone_number,
            heard_count: row.get("heard_count"),
            seen_count: row.get("seen_count"),
            authorization_level: row.get("authorization_level"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at")
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("humans"));
    }
}

// Face encodings for humans
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanFaceEncoding {
    pub id: i32,
    pub oid: String,
    pub encoding: Vec<u8>,
    pub human_oid: String,
    pub timestamp: i64
}
impl HumanFaceEncoding {
    pub fn new() -> HumanFaceEncoding {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        let encoding: Vec<u8> = Vec::new();
        HumanFaceEncoding { 
            id: 0,
            oid: oid,
            encoding: encoding, 
            human_oid: String::new(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        }
    }
    pub fn sql_table_name() -> String {
        return format!("human_face_encodings")
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.human_face_encodings (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            encoding bytea NULL,
            human_oid varchar NULL,
            timestamp BIGINT NULL,
            CONSTRAINT human_face_encodings_pkey PRIMARY KEY (id));"
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
            "ALTER TABLE public.human_face_encodings ADD COLUMN timestamp BIGINT NULL;"
        ]
    }
    pub fn save(object: Self) -> Result<Self, Error>{

        let mut client = Config::client()?;

        // Search for OID matches
        let mut pg_query = PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(object.oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));


        // Search for OID matches
        let rows = Self::select(
            None, 
            None, 
            None, 
            Some(pg_query.clone())
        ).unwrap();

        if rows.len() == 0 {
            client.execute("INSERT INTO human_face_encodings (oid, encoding, human_oid, timestamp) VALUES ($1, $2, $3, $4)",
                &[&object.oid.clone(),
                &object.encoding,
                &object.human_oid,
                &object.timestamp]
            ).unwrap();
            
             let rows_two = Self::select(
                None, 
                None, 
                None, 
                Some(pg_query)
            ).unwrap();
        
            return Ok(rows_two[0].clone());
        
        }
        
    
        Ok(object)
    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {




        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            encoding: row.get("encoding"), 
            human_oid:  row.get("human_oid"),
            timestamp: row.get("timestamp"),
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("human_face_encodings"));
    }
}

// Locations can have many rooms
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub id: i32,
    pub oid: String,
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub lifx_api_key: Option<String>,
    pub created_at: i64,
    pub updated_at: i64
}
impl Location {
    pub fn new() -> Location {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        Location { 
            id: 0,
            oid: oid,
            name: String::new(), 
            address: String::new(),
            city: String::new(),
            state: String::new(),
            zip_code: String::new(),
            lifx_api_key: None,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        }
    }
    pub fn sql_table_name() -> String {
        return format!("locations")
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.locations (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            name varchar NULL,
            address varchar NULL,
            city varchar NULL,
            state varchar NULL,
            zip_code varchar NULL,
            lifx_api_key varchar NULL,
            created_at BIGINT NULL,
            updated_at BIGINT NULL,
            CONSTRAINT locations_pkey PRIMARY KEY (id));"
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
            "ALTER TABLE public.locations ADD COLUMN created_at BIGINT NULL;",
            "ALTER TABLE public.locations ADD COLUMN updated_at BIGINT NULL;",
            "ALTER TABLE public.locations ADD COLUMN lifx_api_key VARCHAR NULL;",
            "ALTER TABLE public.locations ADD COLUMN city VARCHAR NULL;",
            "ALTER TABLE public.locations ADD COLUMN state VARCHAR NULL;",
            "ALTER TABLE public.locations ADD COLUMN zip_code VARCHAR NULL;"
        ]
    }
    pub fn count() -> Result<i64, Error>{

        let mut client = Config::client()?;

        let execquery = format!("SELECT COUNT(*)
        FROM {}", Self::sql_table_name());

        let mut counter: i64 = 0;
        for row in client.query(execquery.as_str(), &[])? {
           counter = row.get("count");
        }

        match client.close(){
            Ok(_) => {},
            Err(e) => log::error!("failed to close connection to database: {}", e),
        }

        Ok(counter)
    }
    pub fn save(&self) -> Result<&Self, Error>{

        let mut client = Config::client()?;

        // Search for OID matches
        let statement = client.prepare("SELECT * FROM locations WHERE oid = $1 OR name ilike $2")?;
        let rows = client.query(&statement, &[
            &self.oid, 
            &self.name,
        ])?;

        if rows.len() == 0 {
            client.execute("INSERT INTO locations (oid, name, address, city, state, zip_code, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);",
                &[&self.oid.clone(),
                &self.name,
                &self.address,
                &self.city,
                &self.state,
                &self.zip_code,
                &self.created_at,
                &self.updated_at]
            ).unwrap();


            if self.lifx_api_key.is_some() {
                client.execute("UPDATE locations SET lifx_api_key = $1 WHERE oid = $2;", 
                &[
                    &self.lifx_api_key.clone().unwrap(),
                    &self.oid
                ])?;
            }

            
            let statement = client.prepare("SELECT * FROM locations WHERE oid = $1")?;
            let _rows_two = client.query(&statement, &[
                &self.oid, 
            ])?;
        
            return Ok(self);
        
        } else {
            let ads = Self::from_row(&rows[0]).unwrap();

            // Only save if newer than stored information
            if self.updated_at > ads.updated_at {
   
                client.execute("UPDATE locations SET name = $1, address = $2, city = $3, state = $4, zip_code = $5, updated_at = $6 WHERE oid = $7;", 
                &[
                    &self.name,
                    &self.address,
                    &self.city,
                    &self.state,
                    &self.zip_code,
                    &self.updated_at,
                    &ads.oid
                ])?;


                if self.lifx_api_key.is_some() {
                    client.execute("UPDATE locations SET lifx_api_key = $1 WHERE oid = $2;", 
                    &[
                        &self.lifx_api_key.clone().unwrap(),
                        &ads.oid
                    ])?;
                }

            }

            let statement_two = client.prepare("SELECT * FROM locations WHERE oid = $1")?;
            let _rows_two = client.query(&statement_two, &[
                &self.oid, 
            ])?;

            return Ok(self);
        }
        

    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {


        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            name: row.get("name"), 
            address: row.get("address"), 
            city: row.get("city"), 
            state: row.get("state"), 
            zip_code: row.get("zip_code"),
            lifx_api_key: row.get("lifx_api_key"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at")
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("locations"));
    }
}




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    pub id: i32,
    pub oid: String,
    pub sid: String,
    pub human_oid: String,
    pub message: String,
    pub seen: bool,
    pub timestamp: i64
}
impl Notification {
    pub fn new() -> Notification {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        Notification { 
            id: 0,
            oid: oid,
            sid: String::new(),
            human_oid: String::new(),
            message: String::new(),
            seen: false,
            timestamp
        }
    }
    pub fn sql_table_name() -> String {
        return format!("notifications")
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.notifications (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            sid varchar NULL,
            human_oid varchar NULL,
            message varchar NULL,
            seen bool DEFAULT false,
            timestamp BIGINT DEFAULT 0,
            CONSTRAINT notifications_pkey PRIMARY KEY (id));"
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
            "",
        ]
    }
    pub fn save(&self) -> Result<Self, Error>{
        let mut client = Config::client()?;
        
        let mut pg_query = PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(self.oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));


        // Search for OID matches
        let rows = Self::select(
            None, 
            None, 
            None, 
            Some(pg_query.clone())
        ).unwrap();

        if rows.len() == 0 {

            client.execute("INSERT INTO notifications (oid, sid, human_oid, message, seen, timestamp) VALUES ($1, $2, $3, $4, $5, $6)",
                &[&self.oid.clone(),
                &self.sid,
                &self.human_oid,
                &self.message,
                &self.seen,
                &self.timestamp]
            ).unwrap();

    
            // Search for OID matches
            let rows_two = Self::select(
                None, 
                None, 
                None, 
                Some(pg_query)
            ).unwrap();

            return Ok(rows_two[0].clone());
        
        } else {
            let ads = rows[0].clone();


 
            client.execute("UPDATE notifications SET message = $1, seen = $2 WHERE oid = $3;", 
            &[&self.message,
            &self.seen,
            &ads.oid])?;


            let statement_two = client.prepare("SELECT * FROM notifications WHERE oid = $1")?;
            let rows_two = client.query(&statement_two, &[
                &self.oid, 
            ])?;

            return Ok(Self::from_row(&rows_two[0])?);
        }
        
    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {
        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            sid: row.get("sid"),
            human_oid: row.get("human_oid"),
            message: row.get("message"),
            seen: row.get("seen"),
            timestamp: row.get("timestamp")
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("notifications"));
    }
}








#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub id: i32,
    pub oid: String,
    pub name: String,
    pub icon: String,
    pub location_oid: String,
    pub created_at: i64,
    pub updated_at: i64
}
impl Room {
    pub fn new() -> Room {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        Room { 
            id: 0,
            oid: oid,
            name: String::new(), 
            icon: format!("fa fa-solid fa-cube"),
            location_oid: String::new(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        }
    }
    pub fn sql_table_name() -> String {
        return format!("rooms")
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.rooms (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            name varchar NULL,
            icon varchar NULL,
            location_oid varchar NULL,
            created_at BIGINT NULL,
            updated_at BIGINT NULL,
            CONSTRAINT rooms_pkey PRIMARY KEY (id));"
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
            "ALTER TABLE public.rooms ADD COLUMN icon varchar NULL;",
            "ALTER TABLE public.rooms ADD COLUMN created_at BIGINT NULL;",
            "ALTER TABLE public.rooms ADD COLUMN updated_at BIGINT NULL;"
        ]
    }
    pub fn save(&self) -> Result<&Self, Error>{

        let mut client = Config::client()?;
        
        // Search for OID matches
        let statement = client.prepare("SELECT * FROM rooms WHERE oid = $1 OR (location_oid = $2 AND name = $3)")?;
        let rows = client.query(&statement, &[
            &self.oid, 
            &self.location_oid,
            &self.name,
        ])?;

        if rows.len() == 0 {
            client.execute("INSERT INTO rooms (oid, name, icon, location_oid, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)",
                &[&self.oid.clone(),
                &self.name,
                &self.icon,
                &self.location_oid,
                &self.created_at,
                &self.updated_at]
            ).unwrap();
        } else {
            let ads = Self::from_row(&rows[0]).unwrap();

            // Only save if newer than stored information
            if self.updated_at > ads.updated_at {
                client.execute("UPDATE rooms SET name = $1, icon = $2, location_oid = $3 WHERE oid = $4;", 
                &[
                    &self.name,
                    &self.icon,
                    &self.location_oid,
                    &ads.oid
                ])?;
            }
        }
        return Ok(self);
        
    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {

        let mut icon: String = format!("fa fa-solid fa-cube");

        match row.get("icon"){
            Some(val) => {
                icon = val;
            },
            None => {}
        }

        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            name: row.get("name"), 
            icon: icon, 
            location_oid: row.get("location_oid"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at")
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("rooms"));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceSetting {
    pub tag: String,
    pub value: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Service {
    pub id: i32,
    pub oid: String,
    pub identifier: String,
    pub key: String,
    pub secret: String,
    pub username: String,
    pub password: String,
    pub endpoint: String,
    pub settings: Vec<ServiceSetting>,
    pub created_at: i64,
    pub updated_at: i64
}
impl Service {
    pub fn new() -> Service {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        let settings: Vec<ServiceSetting> = Vec::new();
        Service { 
            id: 0,
            oid: oid,
            identifier: String::new(),
            key: String::new(),
            secret: String::new(),
            username: String::new(),
            password: String::new(),
            endpoint: String::new(),
            settings: settings,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64

        }
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
            "ALTER TABLE public.services ADD COLUMN created_at BIGINT NULL;",
            "ALTER TABLE public.services ADD COLUMN updated_at BIGINT NULL;",
            "ALTER TABLE public.services ADD COLUMN username varchar NULL;",
            "ALTER TABLE public.services ADD COLUMN password varchar NULL;",
            "ALTER TABLE public.services ADD COLUMN settings varchar NULL;",
        ]
    }
    pub fn sql_table_name() -> String {
        return format!("services")
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.services (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            identifier varchar NULL,
            key varchar NULL,
            secret varchar NULL,
            endpoint varchar NULL,
            settings varchar NULL,
            created_at BIGINT NULL,
            updated_at BIGINT NULL,
            CONSTRAINT services_pkey PRIMARY KEY (id));"
    }
    pub fn save(&self) -> Result<&Self, Error>{

        let mut client = Config::client()?;

        // Search for OID matches
        let mut pg_query = PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(self.oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));
        pg_query.queries.push(crate::sam::memory::PGCol::String(self.identifier.clone()));
        pg_query.query_coulmns.push(format!(" OR identifier ="));
        let rows = Self::select(
            None, 
            None, 
            None, 
            Some(pg_query)
        ).unwrap();

        // Save New Service
        if rows.len() == 0 {
            let settings = serde_json::to_string(&self.settings).unwrap();
            client.execute("INSERT INTO services (oid, identifier, key, secret, username, password, endpoint, settings, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                &[&self.oid.clone(),
                &self.identifier,
                &self.key,
                &self.secret,
                &self.username,
                &self.password,
                &self.endpoint,
                &settings,
                &self.created_at,
                &self.updated_at]
            ).unwrap();
        
            return Ok(self);
        
        } 
        // Update existing service
        else {

            let ads = rows[0].clone();
            let settings = serde_json::to_string(&self.settings).unwrap();
            // Only save if newer than stored information
            client.execute("UPDATE services SET key = $1, secret = $2, settings = $3, updated_at = $4 WHERE oid = $5;", 
            &[
                &self.key,
                &self.secret,
                &settings,
                &(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64),
                &ads.oid
            ])?;
            


            return Ok(self);


        }
        
    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {


        let mut settings: Vec<ServiceSetting> = Vec::new();

        match row.get("settings"){
            Some(settings_str) => {
                settings = serde_json::from_str(settings_str).unwrap();  
            },
            None => {}
        }

        return Ok(Self {
            id: row.get("id"),
            oid:  row.get("oid"),
            identifier: row.get("identifier"),
            key: row.get("key"),
            secret: row.get("secret"),
            username: row.get("username"),
            password: row.get("password"),
            endpoint: row.get("endpoint"),
            settings: settings,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("services"));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thing {
    pub id: i32,
    pub oid: String,
    pub name: String,
    pub room_oid: String,
    pub thing_type: String, // lifx, rtsp, etc
    pub username: String,
    pub password: String,
    pub ip_address: String,
    pub online_identifiers: Vec<String>,
    pub local_identifiers: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64
}
impl Thing {
    pub fn new() -> Thing {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        let empty_vec: Vec<String> = Vec::new();
        Thing { 
            id: 0,
            oid: oid,
            name: String::new(), 
            room_oid: String::new(),
            thing_type: String::new(),
            username: String::new(), 
            password: String::new(), 
            ip_address: String::new(), 
            online_identifiers: empty_vec.clone(),
            local_identifiers: empty_vec.clone(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        }
    }
    pub fn sql_table_name() -> String {
        return format!("things")
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.things (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            name varchar NULL,
            room_oid varchar NULL,
            thing_type varchar NULL,
            username varchar NULL,
            password varchar NULL,
            ip_address varchar NULL,
            online_identifiers varchar NULL,
            local_identifiers varchar NULL,
            created_at BIGINT NULL,
            updated_at BIGINT NULL,
            CONSTRAINT things_pkey PRIMARY KEY (id));"
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
            "ALTER TABLE public.things ADD COLUMN username varchar NULL;",
            "ALTER TABLE public.things ADD COLUMN password varchar NULL;",
            "ALTER TABLE public.things ADD COLUMN ip_address varchar NULL;",
            "ALTER TABLE public.things ADD COLUMN created_at BIGINT NULL;",
            "ALTER TABLE public.things ADD COLUMN updated_at BIGINT NULL;"
        ]
    }
    pub fn save(&self) -> Result<&Self, Error>{

        let mut client = Config::client()?;
        
        // Search for OID matches
        let mut pg_query = PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(self.oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));

        let rows = Self::select(
            None, 
            None, 
            None, 
            Some(pg_query)
        ).unwrap();

        if rows.len() == 0 {
            client.execute("INSERT INTO things (oid, name, room_oid, thing_type, username, password, ip_address, online_identifiers, local_identifiers, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
                &[&self.oid.clone(),
                &self.name,
                &self.room_oid,
                &self.thing_type,
                &self.username,
                &self.password,
                &self.ip_address,
                &self.online_identifiers.join(","),
                &self.local_identifiers.join(","),
                &self.created_at,
                &self.updated_at]
            )?;        
        } else {
            let ads = rows[0].clone();

            // Only save if newer than stored information
            if self.updated_at > ads.updated_at {
                client.execute("UPDATE things SET name = $1, room_oid = $2, thing_type = $3, username = $4, password = $5, ip_address = $6, online_identifiers = $7, local_identifiers = $8 WHERE oid = $9;", 
                &[
                    &self.name,
                    &self.room_oid,
                    &self.thing_type,
                    &self.username,
                    &self.password,
                    &self.ip_address,
                    &self.online_identifiers.join(","),
                    &self.local_identifiers.join(","),
                    &ads.oid
                ])?;
            }
        }
        
        
    
        Ok(self)
    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {
        let mut online_identifiers: Vec<String> = Vec::new();
        let sql_online_identifiers: Option<String> = row.get("online_identifiers");
        match sql_online_identifiers{
            Some(ts) => {
                let split = ts.split(',');
                let vec = split.collect::<Vec<&str>>();
                let mut newvec: Vec<String> = Vec::new();
                for v in vec{
                    newvec.push(v.to_string());
                }
                online_identifiers = newvec;
            },
            None => {}
        }  
            

           
        let mut local_identifiers: Vec<String> = Vec::new();
        let sql_local_identifiers: Option<String> = row.get("local_identifiers");
        match sql_local_identifiers{
            Some(ts) => {
                let split = ts.split(',');
                let vec = split.collect::<Vec<&str>>();
                let mut newvec: Vec<String> = Vec::new();
                for v in vec{
                    newvec.push(v.to_string());
                }
                local_identifiers = newvec;
            },
            None => {}
        }  

        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            name: row.get("name"), 
            room_oid: row.get("room_oid"),
            thing_type: row.get("thing_type"),
            username: row.get("username"),
            password: row.get("password"),
            ip_address: row.get("ip_address"),
            online_identifiers: online_identifiers,
            local_identifiers: local_identifiers,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at")
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("things"));
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Observation {
    pub id: i32,
    pub oid: String,
    pub timestamp: i64,
    pub observation_type: ObservationType,
    pub observation_objects: Vec<ObservationObjects>,
    pub observation_humans: Vec<Human>,
    pub observation_notes: Vec<String>,
    pub observation_file: Option<Vec<u8>>,
    pub deep_vision: Vec<DeepVisionResult>,
    pub deep_vision_json: Option<String>,
    pub thing: Option<Thing>,
    pub web_session: Option<WebSessions>,
}
impl Observation {
    pub fn new() -> Observation {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        let observation_objects: Vec<ObservationObjects> = Vec::new();
        let observation_humans: Vec<Human> = Vec::new();
        let observation_notes: Vec<String> = Vec::new();
        let deep_vision: Vec<DeepVisionResult> = Vec::new();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        Observation { 
            id: 0,
            oid: oid,
            timestamp: timestamp,
            observation_type: ObservationType::UNKNOWN,
            observation_objects: observation_objects,
            observation_humans: observation_humans,
            observation_notes: observation_notes,
            observation_file: None,
            deep_vision,
            deep_vision_json: None,
            thing: None,
            web_session: None,
        }
    }
    pub fn sql_table_name() -> String {
        return format!("observations")
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
            "ALTER TABLE public.observations ADD COLUMN observation_file bytea NULL;",
            "ALTER TABLE public.observations ADD COLUMN deep_vision_json varchar NULL;",
            "ALTER TABLE public.observations ADD COLUMN thing_oid varchar NULL;",
            "ALTER TABLE public.observations ADD COLUMN web_session_id varchar NULL;",
        ]
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.observations (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            timestamp BIGINT NULL,
            observation_type varchar NULL,
            observation_objects varchar NULL,
            observation_humans varchar NULL,
            observation_notes varchar NULL,
            observation_file bytea NULL,
            deep_vision_json varchar NULL,
            thing_oid varchar NULL,
            web_session_id varchar NULL,
            CONSTRAINT observations_pkey PRIMARY KEY (id));"
    }
    pub fn save(&self) -> Result<Self, Error>{

        let mut client = Config::client()?;

        // Search for OID matches
        let mut pg_query = PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(self.oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));
        let rows = Self::select(
            None, 
            None, 
            None, 
            Some(pg_query)
        ).unwrap();

        if rows.len() == 0 {

            let mut obb_obv_str = String::new();
            for obv in &self.observation_objects{
                obb_obv_str += format!("{},", obv.to_string()).as_str();
            }

            let mut obb_humans_str = String::new();
            for hum in &self.observation_humans{
                obb_humans_str += format!("{},", hum.oid.to_string()).as_str();
            }


            let mut obb_thing_str = String::new();
            match &self.thing{
                Some(thing) => {
                    obb_thing_str = thing.oid.clone();
                },
                None => {}
            }

            let mut obb_web_session_str = String::new();
            match &self.web_session{
                Some(web_session) => {
                    obb_web_session_str = web_session.sid.clone();
                },
                None => {}
            }

            client.execute("INSERT INTO observations (oid, timestamp, observation_type, thing_oid, web_session_id, observation_objects, observation_humans, observation_notes, observation_file) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                &[&self.oid.clone(),
                &self.timestamp,
                &self.observation_type.to_string(), 
                &obb_thing_str, 
                &obb_web_session_str,
                &obb_obv_str, 
                &obb_humans_str, 
                &self.observation_notes.join(","),
                &self.observation_file]
            ).unwrap();


            if self.deep_vision_json.is_some() {
                client.execute("UPDATE observations SET deep_vision_json = $1 WHERE oid = $2;", 
                &[
                    &self.deep_vision_json.clone().unwrap(),
                    &self.oid
                ])?;
            }


            let mut pg_query = PostgresQueries::default();
            pg_query.queries.push(crate::sam::memory::PGCol::String(self.oid.clone()));
            pg_query.query_coulmns.push(format!("oid ="));
             let rows_two = Self::select(
                None, 
                None, 
                None, 
                Some(pg_query)
            ).unwrap();
        
            return Ok(rows_two[0].clone());
        
        } else {


            let ads = rows[0].clone();


            let mut obb_obv_str = String::new();
            for obv in &self.observation_objects{
                obb_obv_str += format!("{},", obv.to_string()).as_str();
            }

            let mut obb_humans_str = String::new();
            for hum in &self.observation_humans{
                obb_humans_str += format!("{},", hum.oid.to_string()).as_str();
            }




            client.execute("UPDATE observations SET observation_type = $1, observation_objects = $2, observation_humans = $3, observation_notes = $4, observation_file = $5 WHERE oid = $6;", 
            &[&self.observation_type.to_string(), 
            &obb_obv_str, 
            &obb_humans_str, 
            &self.observation_notes.join(","),
            &self.observation_file,
            &ads.oid])?;

            if self.deep_vision_json.is_some() {
                client.execute("UPDATE observations SET deep_vision_json = $1 WHERE oid = $2;", 
                &[
                    &self.deep_vision_json.clone().unwrap(),
                    &self.oid
                ])?;
            }


    

            let statement_two = client.prepare("SELECT * FROM observations WHERE oid = $1")?;
            let rows_two = client.query(&statement_two, &[
                &self.oid, 
            ])?;

            return Ok(Self::from_row(&rows_two[0])?);

        }
        
    
      
    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    pub fn select_lite(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = Config::pg_select(Self::sql_table_name(), Some(format!("id, oid, timestamp, scout_oid, observation_type, observation_objects, observation_humans, observation_notes, deep_vision_json")), limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {

        let mut deep_vision: Vec<DeepVisionResult> = Vec::new();

        let deep_vision_json = row.get("deep_vision_json");

        match deep_vision_json{
            Some(deep_vision_json_val) => {
                deep_vision = serde_json::from_str(deep_vision_json_val).unwrap();
            },
            None => {
                
            }
        }


    
        let mut observation_type = ObservationType::UNKNOWN;
        let sql_observation_type: Option<String> = row.get("observation_type");
        match sql_observation_type {
            Some(object) => {
                let obj = ObservationType::from_str(&object).unwrap();
                observation_type = obj.clone();
            }, 
            None => {}
        }
        


        let mut observation_objects: Vec<ObservationObjects> = Vec::new();
        let sql_observation_objects: Option<String> = row.get("observation_objects");
        match sql_observation_objects {
            Some(object) => {
                let split = object.split(",");
                for s in split {
                    let obj = ObservationObjects::from_str(&s);
                    match obj{
                        Ok(obj) => observation_objects.push(obj),
                        Err(err) => log::error!("{:?}", err)
                    }
                }
            }, 
            None => {}
        }
        

        let mut observation_humans: Vec<Human> = Vec::new();
        let sql_observation_humans: Option<String> = row.get("observation_humans");
        match sql_observation_humans {
            Some(object) => {
                let split = object.split(",");
                let vec = split.collect::<Vec<&str>>();
                for oidx in vec {

                    // Search for OID matches
                    let mut pg_query = PostgresQueries::default();
                    pg_query.queries.push(crate::sam::memory::PGCol::String(oidx.clone().to_string()));
                    pg_query.query_coulmns.push(format!("oid ilike"));


                    let observation_humansx = Human::select(
                        None, 
                        None, 
                        None, 
                        Some(pg_query)
                    ).unwrap(); 

                    for human in observation_humansx{
                        observation_humans.push(human);
                    }

                    // if rows.len() > 0 {
                    //     observation_humans.push(rows[0].clone());
                    // }
                }
            }, 
            None => {}
        }
        

        let mut observation_notes: Vec<String> = Vec::new();
        let sql_observation_notes: Option<String> = row.get("observation_notes");
        match sql_observation_notes {
            Some(object) => {
                let split = object.split(",");
                for s in split {
                    observation_notes.push(s.to_string());
                }
            }, 
            None => {}
        }
        

        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            timestamp: row.get("timestamp"), 
            observation_type: observation_type,
            observation_objects: observation_objects,
            observation_humans: observation_humans,
            observation_notes: observation_notes,
            observation_file: row.get("observation_file"),
            deep_vision,
            deep_vision_json: row.get("deep_vision_json"),
            thing: None,
            web_session: None,
        });
    }
    fn from_row_lite(row: &Row) -> Result<Self, Error> {

        let mut deep_vision: Vec<DeepVisionResult> = Vec::new();

        let deep_vision_json = row.get("deep_vision_json");

        match deep_vision_json{
            Some(deep_vision_json_val) => {
                deep_vision = serde_json::from_str(deep_vision_json_val).unwrap();
            },
            None => {
                
            }
        }


    
        let mut observation_type = ObservationType::UNKNOWN;
        let sql_observation_type: Option<String> = row.get("observation_type");
        match sql_observation_type {
            Some(object) => {
                let obj = ObservationType::from_str(&object).unwrap();
                observation_type = obj.clone();
            }, 
            None => {}
        }
        


        let mut observation_objects: Vec<ObservationObjects> = Vec::new();
        let sql_observation_objects: Option<String> = row.get("observation_objects");
        match sql_observation_objects {
            Some(object) => {
                let split = object.split(",");
                for s in split {
                    let obj = ObservationObjects::from_str(&s);
                    match obj{
                        Ok(obj) => observation_objects.push(obj),
                        Err(err) => log::error!("{:?}", err)
                    }
                }
            }, 
            None => {}
        }
        

        let mut observation_humans: Vec<Human> = Vec::new();
        let sql_observation_humans: Option<String> = row.get("observation_humans");
        match sql_observation_humans {
            Some(object) => {
                let split = object.split(",");
                let vec = split.collect::<Vec<&str>>();
                for oidx in vec {

                    // Search for OID matches
                    let mut pg_query = PostgresQueries::default();
                    pg_query.queries.push(crate::sam::memory::PGCol::String(oidx.clone().to_string()));
                    pg_query.query_coulmns.push(format!("oid ilike"));


                    let observation_humansx = Human::select(
                        None, 
                        None, 
                        None, 
                        Some(pg_query)
                    ).unwrap(); 

                    for human in observation_humansx{
                        observation_humans.push(human);
                    }

                    // if rows.len() > 0 {
                    //     observation_humans.push(rows[0].clone());
                    // }
                }
            }, 
            None => {}
        }
        

        let mut observation_notes: Vec<String> = Vec::new();
        let sql_observation_notes: Option<String> = row.get("observation_notes");
        match sql_observation_notes {
            Some(object) => {
                let split = object.split(",");
                for s in split {
                    observation_notes.push(s.to_string());
                }
            }, 
            None => {}
        }
        

        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            timestamp: row.get("timestamp"), 
            observation_type: observation_type,
            observation_objects: observation_objects,
            observation_humans: observation_humans,
            observation_notes: observation_notes,
            observation_file: None,
            deep_vision,
            deep_vision_json: row.get("deep_vision_json"),
            thing: None,
            web_session: None,
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("observations"));
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    pub id: i32,
    pub oid: String,
    pub key: String,
    pub values: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64
}
impl Setting {
    pub fn new() -> Setting {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        let empty_vec: Vec<String> = Vec::new();
        Setting { 
            id: 0,
            oid: oid,
            key: String::new(), 
            values: empty_vec,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        }
    }
    pub fn sql_table_name() -> String {
        return format!("settings")
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.settings (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            key varchar NULL,
            values varchar NULL,
            created_at BIGINT NULL,
            updated_at BIGINT NULL,
            CONSTRAINT settings_pkey PRIMARY KEY (id));"
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
            "ALTER TABLE public.settings ADD COLUMN created_at BIGINT NULL;",
            "ALTER TABLE public.settings ADD COLUMN updated_at BIGINT NULL;"
        ]
    }
    pub fn save(&self) -> Result<&Self, Error>{

        let mut client = Config::client()?;
        
        // Search for OID matches
        let mut pg_query = PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(self.oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));
        pg_query.queries.push(crate::sam::memory::PGCol::String(self.key.clone()));
        pg_query.query_coulmns.push(format!(" OR key ="));
        let rows = Self::select(
            None, 
            None, 
            None, 
            Some(pg_query)
        ).unwrap();

        if rows.len() == 0 {
            client.execute("INSERT INTO settings (oid, key, values, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)",
                &[&self.oid.clone(),
                &self.key,
                &self.values.join(","),
                &self.created_at,
                &self.updated_at]
            )?;        
            return Ok(self);
        
        } else {
            let ads = rows[0].clone();

            // Only save if newer than stored information
            if self.updated_at > ads.updated_at {
                client.execute("UPDATE settings SET key = $1, values = $2, updated_at = $3 WHERE oid = $4;", 
                &[
                    &self.key,
                    &self.values.join(","),
                    &self.updated_at,
                    &ads.oid
                ])?;


             
            }

   
            return Ok(self);

        }
        
        

    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {
     

           
        let mut values: Vec<String> = Vec::new();
        let sql_values: Option<String> = row.get("values");
        match sql_values{
            Some(ts) => {
                let split = ts.split(',');
                let vec = split.collect::<Vec<&str>>();
                let mut newvec: Vec<String> = Vec::new();
                for v in vec{
                    newvec.push(v.to_string());
                }
                values = newvec;
            },
            None => {}
        }  

        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            key: row.get("key"), 
            values: values,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at")
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("settings"));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StorageLocation {
    pub id: i32,
    pub oid: String,
    pub storge_type: String, // unique
    pub endpoint: String,
    pub username: String,
    pub password: String,
    pub created_at: i64,
    pub updated_at: i64
}
impl StorageLocation {
    pub fn new() -> StorageLocation {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        StorageLocation { 
            id: 0,
            oid: oid,
            storge_type: String::new(), 
            endpoint: String::new(), 
            username: String::new(), 
            password: String::new(), 
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        }
    }
    pub fn sql_table_name() -> String {
        return format!("storage_locations")
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.storage_locations (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            storge_type varchar NULL,
            endpoint varchar NULL,
            username varchar NULL,
            password varchar NULL,
            created_at BIGINT NULL,
            updated_at BIGINT NULL,
            CONSTRAINT storage_locations_pkey PRIMARY KEY (id));"
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
            "ALTER TABLE public.storage_locations ADD COLUMN created_at BIGINT NULL;",
            "ALTER TABLE public.storage_locations ADD COLUMN updated_at BIGINT NULL;"
        ]
    }
    pub fn save(&self) -> Result<&Self, Error>{

        let mut client = Config::client()?;

        // Search for OID matches
        let mut pg_query = PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(self.oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));
        let rows = Self::select(
            None, 
            None, 
            None, 
            Some(pg_query)
        ).unwrap();

        if rows.len() == 0 {
            client.execute("INSERT INTO storage_locations (oid, storge_type, endpoint, username, password, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                &[&self.oid.clone(),
                &self.storge_type,
                &self.endpoint,
                &self.username,
                &self.password,
                &self.created_at,
                &self.updated_at]
            )?;        
            return Ok(self);
        
        } else {
            let ads = rows[0].clone();

            // Only save if newer than stored information
            if self.updated_at > ads.updated_at {
                client.execute("UPDATE storage_locations SET storge_type = $1, endpoint = $2, username = $3, password = $4, updated_at = $5 WHERE oid = $6;", 
                &[
                    &self.storge_type,
                    &self.endpoint,
                    &self.username,
                    &self.password,
                    &self.updated_at,
                    &ads.oid
                ])?;


             
            }

   
            return Ok(self);

        }
        
        

    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {
        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            storge_type: row.get("storge_type"), 
            endpoint: row.get("endpoint"), 
            username: row.get("username"), 
            password: row.get("password"), 
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at")
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("storage_locations"));
    }
}

pub struct FileMetadataPermissions {
    pub shared_with_humans: Vec<String>,
    pub public: bool,
    pub public_url: String
}

pub struct FileMetadata {
    pub file_name: String,
    pub mime_type: String,
    pub owner: String,
    pub permissions: FileMetadataPermissions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileStorage {
    pub id: i32,
    pub oid: String,
    pub file_name: String, // unique
    pub file_type: String,
    pub file_data: Option<Vec<u8>>,
    pub file_folder_tree: Option<Vec<String>>,
    pub storage_location_oid: String,
    pub created_at: i64,
    pub updated_at: i64
}
impl FileStorage {
    pub fn new() -> FileStorage {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        FileStorage { 
            id: 0,
            oid: oid,
            file_name: String::new(), 
            file_type: String::new(), 
            file_data: None, 
            file_folder_tree: None, 
            storage_location_oid: String::new(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        }
    }
    pub fn sql_table_name() -> String {
        return format!("file_storage")
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.file_storage (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            file_name varchar NULL,
            file_type varchar NULL,
            file_data BYTEA NULL,
            file_folder_tree varchar NULL,
            storage_location_oid varchar NULL,
            created_at BIGINT NULL,
            updated_at BIGINT NULL,
            CONSTRAINT file_storage_pkey PRIMARY KEY (id));"
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
            "ALTER TABLE public.file_storage ADD COLUMN created_at BIGINT NULL;",
            "ALTER TABLE public.file_storage ADD COLUMN updated_at BIGINT NULL;"
        ]
    }
    pub fn save(&self) -> Result<&Self, Error>{

        let mut client = Config::client()?;
        
        // Search for OID matches
        let mut pg_query = PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(self.oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));
        let rows = Self::select(
            None, 
            None, 
            None, 
            Some(pg_query.clone())
        )?;

        if rows.len() == 0 {
            client.execute("INSERT INTO file_storage (oid, file_name, file_type, storage_location_oid, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)",
                &[&self.oid.clone(),
                &self.file_name,
                &self.file_type,
                &self.storage_location_oid,
                &self.created_at,
                &self.updated_at]
            )?;        

        
        } else {
            let ads = rows[0].clone();

            // Only save if newer than stored information
            if self.updated_at > ads.updated_at {
                client.execute("UPDATE file_storage SET file_name = $1, file_type = $2, storage_location_oid = $3, updated_at = $4 WHERE oid = $5;", 
                &[
                    &self.file_name,
                    &self.file_type,
                    &self.storage_location_oid,
                    &self.updated_at,
                    &ads.oid
                ])?;
            }

        }

        let rows = Self::select(
            None, 
            None, 
            None, 
            Some(pg_query)
        )?;
        let ads = rows[0].clone();

        match self.file_folder_tree.clone(){
            Some(folder_tree) => {
                client.execute("UPDATE file_storage SET file_folder_tree = $1, updated_at = $2 WHERE oid = $3;", 
                &[
                    &folder_tree.join("/"),
                    &self.updated_at,
                    &ads.oid
                ])?;  
            },
            None => {}
        }

        match self.file_data.clone(){
            Some(file_data) => {
                client.execute("UPDATE file_storage SET file_data = $1, updated_at = $2 WHERE oid = $3;", 
                &[
                    &file_data,
                    &self.updated_at,
                    &ads.oid
                ])?;  
            },
            None => {}
        }

        return Ok(self);


    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    pub fn select_lite(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = Config::pg_select(Self::sql_table_name(), Some(format!("id, oid, file_name, file_type, file_folder_tree, storage_location_oid, created_at, updated_at")), limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {

        let mut file_folder_tree: Option<Vec<String>> = None;
        let sql_file_folder_tree: Option<String> = row.get("file_folder_tree");
        match sql_file_folder_tree {
            Some(ts) => {
                let split = ts.split('/');
                let vec = split.collect::<Vec<&str>>();
                let mut newvec: Vec<String> = Vec::new();
                for v in vec{
                    newvec.push(v.to_string());
                }
                file_folder_tree = Some(newvec);
            },
            None => {}
        }

        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            file_name: row.get("file_name"), 
            file_type: row.get("file_type"), 
            file_data: row.get("file_data"), 
            file_folder_tree: file_folder_tree, 
            storage_location_oid: row.get("storage_location_oid"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at")
        });
    }
    fn from_row_lite(row: &Row) -> Result<Self, Error> {

        let mut file_folder_tree: Option<Vec<String>> = None;
        let sql_file_folder_tree: Option<String> = row.get("file_folder_tree");
        match sql_file_folder_tree {
            Some(ts) => {
                let split = ts.split('/');
                let vec = split.collect::<Vec<&str>>();
                let mut newvec: Vec<String> = Vec::new();
                for v in vec{
                    newvec.push(v.to_string());
                }
                file_folder_tree = Some(newvec);
            },
            None => {}
        }


        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            file_name: row.get("file_name"), 
            file_type: row.get("file_type"), 
            file_data: None, 
            file_folder_tree: file_folder_tree, 
            storage_location_oid: row.get("storage_location_oid"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at")
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("file_storage"));
    }
    pub fn cache_all() -> Result<(), Error>{
        let files_without_data = FileStorage::select_lite(None, None, None, None)?;

        for file in files_without_data{

            if !Path::new(file.path_on_disk().as_str()).exists(){


                if file.storage_location_oid == format!("SQL"){
                    let mut pg_query = PostgresQueries::default();
                    pg_query.queries.push(crate::sam::memory::PGCol::String(file.oid.clone()));
                    pg_query.query_coulmns.push(format!("oid ="));
        
                    let files_with_data = FileStorage::select(None, None, None, Some(pg_query))?;
                    let ffile = files_with_data[0].clone();
                    ffile.cache()?;
                } else if file.storage_location_oid == format!("DROPBOX"){
                    crate::sam::services::dropbox::download_file("/Sam/test.png", file.path_on_disk().as_str());
                }
        
            }

        }

        return Ok(());
    }
    pub fn cache(&self) -> Result<(), Error>{
        std::fs::write(self.path_on_disk().clone(), self.file_data.clone().unwrap()).unwrap();
        return Ok(());
    }
    pub fn path_on_disk(&self) -> String{
        return format!("/opt/sam/files/{}", self.oid.clone());
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSessions {
    pub id: i32,
    pub oid: String,
    pub sid: String,
    pub human_oid: String,
    pub ip_address: String,
    pub authenticated: bool,
    pub timestamp: i64,
}
impl WebSessions {
    pub fn new(sid: String) -> WebSessions {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        WebSessions { 
            id: 0,
            oid: oid,
            sid: sid,
            human_oid: String::new(), 
            ip_address: String::new(),
            authenticated: false,
            timestamp: timestamp,
        }
    }
    pub fn sql_table_name() -> String {
        return format!("web_sessions")
    }
    pub fn migrations() -> Vec<&'static str> {
        vec![
           ""
        ]
    }
    pub fn sql_build_statement() -> &'static str {
        "CREATE TABLE public.web_sessions (
            id serial NOT NULL,
            oid varchar NOT NULL UNIQUE,
            sid varchar NOT NULL UNIQUE,
            human_oid varchar NULL,
            ip_address varchar NULL,
            authenticated bool NULL DEFAULT FALSE,
            timestamp BIGINT NULL,
            CONSTRAINT web_sessions_pkey PRIMARY KEY (id));"
    }
    pub fn save(&self) -> Result<&Self, Error>{
        
        let mut client = Config::client()?;

        // Search for OID matches
        let mut pg_query = PostgresQueries::default();
        pg_query.queries.push(crate::sam::memory::PGCol::String(self.oid.clone()));
        pg_query.query_coulmns.push(format!("oid ="));
        pg_query.queries.push(crate::sam::memory::PGCol::String(self.sid.clone()));
        pg_query.query_coulmns.push(format!(" OR sid ="));
        let rows = Self::select(
            None, 
            None, 
            None, 
            Some(pg_query)
        )?;

        if rows.len() == 0 {



            client.execute("INSERT INTO web_sessions (oid, sid, human_oid, ip_address, authenticated, timestamp) VALUES ($1, $2, $3, $4, $5, $6)",
                &[&self.oid.clone(),
                &self.sid,
                &self.human_oid, 
                &self.ip_address.to_string(), 
                &self.authenticated, 
                &self.timestamp]
            ).unwrap();

        
            return Ok(self);
        
        } 
        
    
        Ok(self)
    }
    pub fn select(limit: Option<usize>, offset: Option<usize>, order: Option<String>, query: Option<PostgresQueries>) -> Result<Vec<Self>, Error>{
        let mut parsed_rows: Vec<Self> = Vec::new();
        let jsons = crate::sam::memory::Config::pg_select(Self::sql_table_name(), None, limit, offset, order, query)?;

        for j in jsons{
            let object: Self = serde_json::from_str(&j).unwrap();
            parsed_rows.push(object);
        }
        

        Ok(parsed_rows)
    }
    fn from_row(row: &Row) -> Result<Self, Error> {


        return Ok(Self {
            id: row.get("id"),
            oid: row.get("oid"),
            sid: row.get("sid"),
            human_oid: row.get("human_oid"), 
            ip_address: row.get("ip_address"), 
            authenticated: row.get("authenticated"),
            timestamp: row.get("timestamp"),
        });
    }
    pub fn destroy(oid: String) -> Result<bool, Error>{
        return crate::sam::memory::Config::destroy_row(oid, format!("web_sessions"));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostgresServer {
	pub db_name: String,
    pub username: String,
    pub password: String,
	pub address: String
}
impl PostgresServer {
    pub fn new() -> PostgresServer {

        let db_name = env::var("PG_DBNAME").expect("$PG_DBNAME is not set");
        let username = env::var("PG_USER").expect("$PG_USER is not set");
        let password = env::var("PG_PASS").expect("$PG_PASS is not set");
        let address = env::var("PG_ADDRESS").expect("$PG_ADDRESS is not set");


        PostgresServer{
            db_name, 
            username, 
            password, 
            address
        }
    }
}

// Not tracked in SQL
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostgresQueries {
    pub queries: Vec<PGCol>, 
    pub query_coulmns: Vec<String>,
    pub append: Option<String>
}
impl Default for PostgresQueries {
    fn default () -> PostgresQueries {
        let queries: Vec<PGCol> = Vec::new();
        let query_coulmns: Vec<String> = Vec::new();
        PostgresQueries{queries, query_coulmns, append: None }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PGCol {
    String(String),
    Number(i32),
    Boolean(bool),
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeepVisionResult {
    pub id: String,
    pub whoio: Option<WhoioResult>,
    pub probability: f64,
    pub left: i64,
    pub bottom: i64,
    pub right: i64,
    pub top: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WhoioResult {
    pub id: String,
    pub directory: String,
    pub left: i64,
    pub bottom: i64,
    pub right: i64,
    pub top: i64
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ObservationType {
    UNKNOWN,
    SEEN,
    HEARD
}
impl fmt::Display for ObservationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::str::FromStr for ObservationType {
    type Err = ();
    fn from_str(input: &str) -> Result<ObservationType, Self::Err> {
        match input {
            "UNKNOWN"  => Ok(ObservationType::UNKNOWN),
            "SEEN"  => Ok(ObservationType::SEEN),
            "HEARD"  => Ok(ObservationType::HEARD),
            _      => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ObservationObjects {
    #[allow(non_camel_case_types)]
    QR_CODE,
    #[allow(non_camel_case_types)]
    PERSON,
    #[allow(non_camel_case_types)]
    BICYCLE,
    #[allow(non_camel_case_types)]
    CAR,
    #[allow(non_camel_case_types)]
    MOTORBIKE,
    #[allow(non_camel_case_types)]
    AEROPLANE,
    #[allow(non_camel_case_types)]
    BUS,
    #[allow(non_camel_case_types)]
    TRAIN,
    #[allow(non_camel_case_types)]
    TRUCK,
    #[allow(non_camel_case_types)]
    BOAT,
    #[allow(non_camel_case_types)]
    TRAFFIC_LIGHT,
    #[allow(non_camel_case_types)]
    FIRE_HYDRANT,
    #[allow(non_camel_case_types)]
    STOP_SIGN,
    #[allow(non_camel_case_types)]
    PARKING_METER,
    #[allow(non_camel_case_types)]
    BENCH,
    #[allow(non_camel_case_types)]
    BIRD,
    #[allow(non_camel_case_types)]
    CAT,
    #[allow(non_camel_case_types)]
    DOG,
    #[allow(non_camel_case_types)]
    HORSE,
    #[allow(non_camel_case_types)]
    SHEEP,
    #[allow(non_camel_case_types)]
    COW,
    #[allow(non_camel_case_types)]
    ELEPHANT,
    #[allow(non_camel_case_types)]
    BEAR,
    #[allow(non_camel_case_types)]
    ZEBRA,
    #[allow(non_camel_case_types)]
    GIRAFFE,
    #[allow(non_camel_case_types)]
    BACKPACK,
    #[allow(non_camel_case_types)]
    UMBRELLA,
    #[allow(non_camel_case_types)]
    HANDBAG,
    #[allow(non_camel_case_types)]
    TIE,
    #[allow(non_camel_case_types)]
    SUITCASE,
    #[allow(non_camel_case_types)]
    FRISBEE,
    #[allow(non_camel_case_types)]
    SKIS,
    #[allow(non_camel_case_types)]
    SNOWBOARD,
    #[allow(non_camel_case_types)]
    SPORTS_BALL,
    #[allow(non_camel_case_types)]
    KITE,
    #[allow(non_camel_case_types)]
    BASEBALL_BAT,
    #[allow(non_camel_case_types)]
    BASEBALL_GLOVE,
    #[allow(non_camel_case_types)]
    SKATEBOARD,
    #[allow(non_camel_case_types)]
    SURFBOARD,
    #[allow(non_camel_case_types)]
    TENNIS_RACKET,
    #[allow(non_camel_case_types)]
    BOTTLE,
    #[allow(non_camel_case_types)]
    WINE_GLASS,
    #[allow(non_camel_case_types)]
    CUP,
    #[allow(non_camel_case_types)]
    FORK,
    #[allow(non_camel_case_types)]
    KNIFE,
    #[allow(non_camel_case_types)]
    SPOON,
    #[allow(non_camel_case_types)]
    BOWL,
    #[allow(non_camel_case_types)]
    BANANA,
    #[allow(non_camel_case_types)]
    APPLE,
    #[allow(non_camel_case_types)]
    SANDWICH,
    #[allow(non_camel_case_types)]
    ORANGE,
    #[allow(non_camel_case_types)]
    BROCCOLI,
    #[allow(non_camel_case_types)]
    CARROT,
    #[allow(non_camel_case_types)]
    HOT_DOG,
    #[allow(non_camel_case_types)]
    PIZZA,
    #[allow(non_camel_case_types)]
    DONUT,
    #[allow(non_camel_case_types)]
    CAKE,
    #[allow(non_camel_case_types)]
    CHAIR,
    #[allow(non_camel_case_types)]
    SOFA,
    #[allow(non_camel_case_types)]
    POTTED_PLANT,
    #[allow(non_camel_case_types)]
    BED,
    #[allow(non_camel_case_types)]
    DINING_TABLE,
    #[allow(non_camel_case_types)]
    TOILET,
    #[allow(non_camel_case_types)]
    TV_MONITOR,
    #[allow(non_camel_case_types)]
    LAPTOP,
    #[allow(non_camel_case_types)]
    MOUSE,
    #[allow(non_camel_case_types)]
    REMOTE,
    #[allow(non_camel_case_types)]
    KEYBOARD,
    #[allow(non_camel_case_types)]
    CELL_PHONE,
    #[allow(non_camel_case_types)]
    MICROWAVE,
    #[allow(non_camel_case_types)]
    OVEN,
    #[allow(non_camel_case_types)]
    TOASTER,
    #[allow(non_camel_case_types)]
    SINK,
    #[allow(non_camel_case_types)]
    REFRIGERATOR,
    #[allow(non_camel_case_types)]
    BOOK,
    #[allow(non_camel_case_types)]
    CLOCK,
    #[allow(non_camel_case_types)]
    VASE,
    #[allow(non_camel_case_types)]
    SCISSORS,
    #[allow(non_camel_case_types)]
    TEDDY_BEAR,
    #[allow(non_camel_case_types)]
    HAIR_DRIER,
    #[allow(non_camel_case_types)]
    TOOTHBRUSH
}
impl fmt::Display for ObservationObjects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::str::FromStr for ObservationObjects {
    type Err = ();
    fn from_str(input: &str) -> Result<ObservationObjects, Self::Err> {
        match input {
            "QR_CODE"  => Ok(ObservationObjects::QR_CODE),
            "PERSON"  => Ok(ObservationObjects::PERSON),
            "BICYCLE"  => Ok(ObservationObjects::BICYCLE),
            "CAR"  => Ok(ObservationObjects::CAR),
            "MOTORBIKE"  => Ok(ObservationObjects::MOTORBIKE),
            "AEROPLANE"  => Ok(ObservationObjects::AEROPLANE),
            "BUS"  => Ok(ObservationObjects::BUS),
            "TRAIN"  => Ok(ObservationObjects::TRAIN),
            "TRUCK"  => Ok(ObservationObjects::TRUCK),
            "BOAT"  => Ok(ObservationObjects::BOAT),
            "TRAFFIC_LIGHT"  => Ok(ObservationObjects::TRAFFIC_LIGHT),
            "FIRE_HYDRANT"  => Ok(ObservationObjects::FIRE_HYDRANT),
            "STOP_SIGN"  => Ok(ObservationObjects::STOP_SIGN),
            "PARKING_METER"  => Ok(ObservationObjects::PARKING_METER),
            "BENCH"  => Ok(ObservationObjects::BENCH),
            "BIRD"  => Ok(ObservationObjects::BIRD),
            "CAT"  => Ok(ObservationObjects::CAT),
            "DOG"  => Ok(ObservationObjects::DOG),
            "HORSE"  => Ok(ObservationObjects::HORSE),
            "SHEEP"  => Ok(ObservationObjects::SHEEP),
            "COW"  => Ok(ObservationObjects::COW),
            "ELEPHANT"  => Ok(ObservationObjects::ELEPHANT),
            "BEAR"  => Ok(ObservationObjects::BEAR),
            "ZEBRA"  => Ok(ObservationObjects::ZEBRA),
            "GIRAFFE"  => Ok(ObservationObjects::GIRAFFE),
            "BACKPACK"  => Ok(ObservationObjects::BACKPACK),
            "UMBRELLA"  => Ok(ObservationObjects::UMBRELLA),
            "HANDBAG"  => Ok(ObservationObjects::HANDBAG),
            "TIE"  => Ok(ObservationObjects::TIE),
            "SUITCASE"  => Ok(ObservationObjects::SUITCASE),
            "FRISBEE"  => Ok(ObservationObjects::FRISBEE),
            "SKIS"  => Ok(ObservationObjects::SKIS),
            "SNOWBOARD"  => Ok(ObservationObjects::SNOWBOARD),
            "SPORTS_BALL"  => Ok(ObservationObjects::SPORTS_BALL),
            "KITE"  => Ok(ObservationObjects::KITE),
            "BASEBALL_BAT"  => Ok(ObservationObjects::BASEBALL_BAT),
            "SKATEBOARD"  => Ok(ObservationObjects::SKATEBOARD),
            "SURFBOARD"  => Ok(ObservationObjects::SURFBOARD),
            "TENNIS_RACKET"  => Ok(ObservationObjects::TENNIS_RACKET),
            "BOTTLE"  => Ok(ObservationObjects::BOTTLE),
            "WINE_GLASS"  => Ok(ObservationObjects::WINE_GLASS),
            "CUP"  => Ok(ObservationObjects::CUP),
            "FORK"  => Ok(ObservationObjects::FORK),
            "KNIFE"  => Ok(ObservationObjects::KNIFE),
            "SPOON"  => Ok(ObservationObjects::SPOON),
            "BOWL"  => Ok(ObservationObjects::BOWL),
            "BANANA"  => Ok(ObservationObjects::BANANA),
            "APPLE"  => Ok(ObservationObjects::APPLE),
            "SANDWICH"  => Ok(ObservationObjects::SANDWICH),
            "ORANGE"  => Ok(ObservationObjects::ORANGE),
            "BROCCOLI"  => Ok(ObservationObjects::BROCCOLI),
            "CARROT"  => Ok(ObservationObjects::CARROT),
            "HOT_DOG"  => Ok(ObservationObjects::HOT_DOG),
            "PIZZA"  => Ok(ObservationObjects::PIZZA),
            "DONUT"  => Ok(ObservationObjects::DONUT),
            "CAKE"  => Ok(ObservationObjects::CAKE),
            "CHAIR"  => Ok(ObservationObjects::CHAIR),
            "SOFA"  => Ok(ObservationObjects::SOFA),
            "POTTED_PLANT"  => Ok(ObservationObjects::POTTED_PLANT),
            "BED"  => Ok(ObservationObjects::BED),
            "DINING_TABLE"  => Ok(ObservationObjects::DINING_TABLE),
            "TOILET"  => Ok(ObservationObjects::TOILET),
            "TV_MONITOR"  => Ok(ObservationObjects::TV_MONITOR),
            "LAPTOP"  => Ok(ObservationObjects::LAPTOP),
            "MOUSE"  => Ok(ObservationObjects::MOUSE),
            "REMOTE"  => Ok(ObservationObjects::REMOTE),
            "KEYBOARD"  => Ok(ObservationObjects::KEYBOARD),
            "CELL_PHONE"  => Ok(ObservationObjects::CELL_PHONE),
            "MICROWAVE"  => Ok(ObservationObjects::MICROWAVE),
            "OVEN"  => Ok(ObservationObjects::OVEN),
            "SINK"  => Ok(ObservationObjects::SINK),
            "REFRIGERATOR"  => Ok(ObservationObjects::REFRIGERATOR),
            "BOOK"  => Ok(ObservationObjects::BOOK),
            "CLOCK"  => Ok(ObservationObjects::CLOCK),
            "VASE"  => Ok(ObservationObjects::VASE),
            "SCISSORS"  => Ok(ObservationObjects::SCISSORS),
            "TEDDY_BEAR"  => Ok(ObservationObjects::TEDDY_BEAR),
            "HAIR_DRIER"  => Ok(ObservationObjects::HAIR_DRIER),
            "TOOTHBRUSH"  => Ok(ObservationObjects::TOOTHBRUSH),
            _      => Err(()),
        }
    }
}