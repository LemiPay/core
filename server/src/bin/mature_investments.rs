use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rand::Rng;
use server::infrastructure::db::{
    config::DbConfig,
    models::investment::{InvestmentStatusModel, NewInvestmentValueSnapshotModel},
    pool::create_pool,
    schema,
};
use uuid::Uuid;

fn risk_range(risk_level: &str) -> f64 {
    match risk_level {
        "low" => 0.01,
        "medium" => 0.05,
        "high" => 0.10,
        _ => 0.0,
    }
}

fn main() {
    dotenvy::dotenv().ok();

    let args: Vec<String> = std::env::args().collect();
    let now = if let Some(pos) = args.iter().position(|a| a == "--now") {
        args.get(pos + 1)
            .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S").ok())
            .expect("Usage: mature_investments [--now YYYY-MM-DDTHH:MM:SS]")
    } else {
        chrono::Utc::now().naive_utc()
    };

    let db_config = DbConfig::from_env();
    let pool = create_pool(&db_config.database_url);
    let mut conn = pool.get().expect("Failed to get DB connection");

    let active: Vec<(Uuid, Uuid, BigDecimal, NaiveDateTime, NaiveDateTime)> =
        schema::investment::table
            .filter(schema::investment::status.eq(InvestmentStatusModel::Active))
            .inner_join(
                schema::investment_proposal::table
                    .on(schema::investment::proposal_id
                        .eq(schema::investment_proposal::proposal_id)),
            )
            .select((
                schema::investment::id,
                schema::investment_proposal::strategy_id,
                schema::investment::amount,
                schema::investment::started_at,
                schema::investment::matures_at,
            ))
            .load(&mut conn)
            .expect("Failed to query active investments");

    if active.is_empty() {
        println!("No active investments found.");
        return;
    }

    println!(
        "Processing {} active investment(s) at {}",
        active.len(),
        now
    );

    let mut rng = rand::rngs::OsRng;
    let mut matured_count = 0;
    let mut updated_count = 0;

    for (investment_id, strategy_id, amount, started_at, matures_at) in &active {
        let (return_pct, risk_level, duration_days): (BigDecimal, String, i32) =
            schema::investment_strategy::table
                .filter(schema::investment_strategy::id.eq(strategy_id))
                .select((
                    schema::investment_strategy::expected_return_percentage,
                    schema::investment_strategy::risk_level,
                    schema::investment_strategy::duration_days,
                ))
                .first(&mut conn)
                .expect("Strategy not found");

        let elapsed_days = (now - *started_at).num_days();
        if elapsed_days < 0 {
            continue;
        }

        let days = BigDecimal::from_i64(elapsed_days).unwrap();
        let duration = BigDecimal::from_i32(duration_days).unwrap();
        let hundred = BigDecimal::from(100);

        let current_value: BigDecimal =
            amount.clone() * (BigDecimal::from(1) + &return_pct / &hundred * &days / &duration);

        diesel::update(schema::investment::table.filter(schema::investment::id.eq(investment_id)))
            .set((
                schema::investment::current_value.eq(&current_value),
                schema::investment::updated_at.eq(now),
            ))
            .execute(&mut conn)
            .expect("Failed to update current_value");

        diesel::insert_into(schema::investment_value_snapshot::table)
            .values(&NewInvestmentValueSnapshotModel {
                investment_id: *investment_id,
                value: current_value.clone(),
                snapshot_date: now,
            })
            .execute(&mut conn)
            .expect("Failed to insert snapshot");

        updated_count += 1;

        if *matures_at <= now {
            let range = risk_range(&risk_level);
            let variation: f64 = rng.gen_range(-range..=range);

            let return_portion = &current_value - amount;
            let varied_return = if return_portion.is_zero() {
                BigDecimal::zero()
            } else {
                return_portion * BigDecimal::from_f64(1.0 + variation).unwrap()
            };

            let final_value = amount + &varied_return;

            diesel::update(
                schema::investment::table.filter(schema::investment::id.eq(investment_id)),
            )
            .set((
                schema::investment::status.eq(InvestmentStatusModel::Matured),
                schema::investment::actual_return.eq(&varied_return),
                schema::investment::current_value.eq(&final_value),
                schema::investment::updated_at.eq(now),
            ))
            .execute(&mut conn)
            .expect("Failed to mature investment");

            matured_count += 1;

            println!(
                "  Matured {}: amount={}, current_value={}, actual_return={}, risk={}, variation={:.2}%",
                investment_id,
                amount,
                final_value,
                varied_return,
                risk_level,
                variation * 100.0,
            );
        } else {
            println!(
                "  Updated {}: amount={}, current_value={}, elapsed_days={}",
                investment_id, amount, current_value, elapsed_days,
            );
        }
    }

    println!(
        "Done! {} updated, {} matured.",
        updated_count, matured_count
    );
}
