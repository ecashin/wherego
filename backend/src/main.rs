use std::{convert::Infallible, net::Ipv4Addr};

use anyhow::{Context, Result};
use clap::Parser;
use deadpool_postgres::{
    tokio_postgres::{Config, NoTls},
    Manager, Pool,
};
use log::info;
use warp::{Filter, Reply};

use wherego::{Destination, Score};

const CREATE_DB_ASSETS: [&str; 2] = [include_str!("destinations.sql"), include_str!("scores.sql")];

#[derive(Debug, Parser)]
struct Cli {
    postgres_user: String,
    postgres_password: String,
    #[arg(long)]
    listen_ip: Option<Ipv4Addr>,
    #[arg(long)]
    reinitialize_database: bool,
}

async fn set_up_database(pool: Pool) -> Result<()> {
    let client = pool.get().await.context("getting client from DB pool")?;
    for sql in CREATE_DB_ASSETS {
        for line in sql.lines() {
            client.execute(line, &[]).await?;
        }
    }
    Ok(())
}

async fn get_scores(pool: Pool) -> std::result::Result<impl Reply, Infallible> {
    let client = pool
        .get()
        .await
        .context("getting DB client from pool")
        .unwrap();
    let sql = "
        select username, dest_id, score from wherego_scores
    ";
    let stmt = client.prepare(sql).await.unwrap();
    let scores = client
        .query(&stmt, &[])
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            info!("row: {row:?}");
            let username = row.get::<_, String>(0);
            let dest_id = row.get::<_, i64>(1);
            let score = row.get::<_, i64>(2);
            Score {
                username,
                dest_id,
                score,
            }
        })
        .collect::<Vec<_>>();
    Ok(warp::reply::json(&scores))
}

async fn post_score(pool: Pool, score: Score) -> std::result::Result<impl Reply, Infallible> {
    println!("received score update: {score:?}");
    let client = pool
        .get()
        .await
        .context("getting DB client from pool")
        .unwrap();
    let sql = "
        insert into wherego_scores (username, dest_id, score)
        values
        ($1, $2, $3)
        on conflict (username, dest_id)
        do update set score = excluded.score
    ";
    client
        .execute(sql, &[&score.username, &score.dest_id, &score.score])
        .await
        .context("executing statement")
        .unwrap();

    Ok(warp::reply::json(&score))
}

async fn post_destination(
    pool: Pool,
    dest: Destination,
) -> std::result::Result<impl Reply, Infallible> {
    println!("received new destination: {dest:?}");
    let client = pool
        .get()
        .await
        .context("getting DB client from pool")
        .unwrap();
    if dest.id < 0 {
        let sql = "
            insert into wherego_destinations (name, description)
            values
            ($1, $2)
        ";
        client
            .execute(sql, &[&dest.name, &dest.description])
            .await
            .context("executing statement")
            .unwrap();
    } else {
        let sql = "
            update wherego_destinations set name = $1, description = $2
            where id = $3
        ";
        client
            .execute(sql, &[&dest.name, &dest.description, &dest.id])
            .await
            .context("executing statement")
            .unwrap();
    }

    Ok(warp::reply::json(&dest))
}

async fn get_destinations(pool: Pool) -> std::result::Result<impl Reply, Infallible> {
    let client = pool
        .get()
        .await
        .context("getting DB client from pool")
        .unwrap();
    let sql = "
        select name, description, id from wherego_destinations
    ";
    let stmt = client.prepare(sql).await.unwrap();
    let destinations = client
        .query(&stmt, &[])
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            info!("row: {row:?}");
            let id = row.get::<_, i64>(2);
            let name = row.get::<_, String>(0);
            let description = row.get::<_, String>(1);
            Destination {
                name,
                description,
                id,
            }
        })
        .collect::<Vec<_>>();
    Ok(warp::reply::json(&destinations))
}

async fn api_routes(pool: Pool) -> warp::filters::BoxedFilter<(impl Reply,)> {
    let get_destinations = {
        let pool = pool.clone();
        warp::get()
            .and(warp::path!("api" / "destinations"))
            .and(warp::any().map(move || pool.clone()))
            .and_then(get_destinations)
    };
    let get_scores = {
        let pool = pool.clone();
        warp::get()
            .and(warp::path!("api" / "scores"))
            .and(warp::any().map(move || pool.clone()))
            .and_then(get_scores)
    };
    let post_score = {
        let pool = pool.clone();
        warp::post()
            .and(warp::path!("api" / "scores"))
            .and(warp::any().map(move || pool.clone()))
            .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
            .and_then(post_score)
    };
    let post_destination = {
        let pool = pool.clone();
        warp::post()
            .and(warp::path!("api" / "destinations"))
            .and(warp::any().map(move || pool.clone()))
            .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
            .and_then(post_destination)
    };
    get_destinations
        .or(get_scores)
        .or(post_score)
        .or(post_destination)
        .boxed()
}

#[tokio::main]
async fn main() -> Result<()> {
    simple_logger::init_with_env().context("setting up logging")?;
    let cli = Cli::parse();
    let mut cfg = Config::new();
    cfg.host("127.0.0.1");
    cfg.user(&cli.postgres_user);
    cfg.password(cli.postgres_password);
    cfg.dbname(&cli.postgres_user);
    let mgr = Manager::new(cfg, NoTls);
    let pool = Pool::builder(mgr)
        .max_size(8)
        .build()
        .context("creating DB pool")?;
    if cli.reinitialize_database {
        set_up_database(pool.clone())
            .await
            .context("initializing DB")?;
    }

    let routes = api_routes(pool.clone())
        .await
        .or(warp::get().and(warp::fs::dir("../frontend/dist")));
    let listen_ip = if let Some(ip) = cli.listen_ip {
        ip
    } else {
        Ipv4Addr::new(127, 0, 0, 1)
    };
    warp::serve(routes).run((listen_ip, 3030)).await;

    Ok(())
}
