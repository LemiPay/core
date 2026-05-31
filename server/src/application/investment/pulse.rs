use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use chrono::Utc;
use diesel::prelude::*;
use rand::Rng;
use uuid::Uuid;

use crate::infrastructure::db::{
    models::investment::{InvestmentStatusModel, NewInvestmentValueSnapshotModel},
    pool::DbPool,
    schema,
};

pub struct PulseResult {
    pub updated: usize,
    pub matured: usize,
}

pub fn process_pulse(pool: &DbPool) -> Result<PulseResult, String> {
    let mut conn = pool
        .get()
        .map_err(|e| format!("DB connection error: {}", e))?;
    let mut rng = rand::rngs::OsRng;
    let now = Utc::now().naive_utc();

    let active = schema::investment::table
        .filter(schema::investment::status.eq(InvestmentStatusModel::Active))
        .inner_join(
            schema::investment_proposal::table
                .on(schema::investment::proposal_id.eq(schema::investment_proposal::proposal_id)),
        )
        .inner_join(
            schema::investment_strategy::table
                .on(schema::investment_proposal::strategy_id.eq(schema::investment_strategy::id)),
        )
        .select((
            schema::investment::id,
            schema::investment::amount,
            schema::investment_strategy::expected_return_percentage,
            schema::investment_strategy::risk_level,
            schema::investment_strategy::duration_days,
        ))
        .load::<(Uuid, BigDecimal, BigDecimal, String, i32)>(&mut conn)
        .map_err(|e| format!("Failed to query active investments: {}", e))?;

    if active.is_empty() {
        return Ok(PulseResult {
            updated: 0,
            matured: 0,
        });
    }

    let mut updated = 0;
    let mut matured = 0;

    for (investment_id, amount, return_pct, risk_level, duration_days) in &active {
        let snapshot_count: i64 = schema::investment_value_snapshot::table
            .filter(schema::investment_value_snapshot::investment_id.eq(investment_id))
            .count()
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to count snapshots for {}: {}", investment_id, e))?;

        let accrued_days = snapshot_count + 1;
        let is_last_day = accrued_days >= *duration_days as i64;

        let days = BigDecimal::from_i64(accrued_days).unwrap();
        let duration = BigDecimal::from_i32(*duration_days).unwrap();
        let hundred = BigDecimal::from(100);

        let linear_value =
            amount.clone() * (BigDecimal::from(1) + return_pct / &hundred * &days / &duration);

        let noise_pct = daily_noise_range(risk_level);
        let noise_factor =
            BigDecimal::from_f64(1.0 + rng.gen_range(-noise_pct..=noise_pct)).unwrap();
        let current_value = linear_value * noise_factor;

        if is_last_day {
            let variation_pct = risk_variation_range(risk_level);
            let variation: f64 = rng.gen_range(-variation_pct..=variation_pct);

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
            .map_err(|e| format!("Failed to mature investment {}: {}", investment_id, e))?;

            matured += 1;
        } else {
            diesel::update(
                schema::investment::table.filter(schema::investment::id.eq(investment_id)),
            )
            .set((
                schema::investment::current_value.eq(&current_value),
                schema::investment::updated_at.eq(now),
            ))
            .execute(&mut conn)
            .map_err(|e| {
                format!(
                    "Failed to update current_value for {}: {}",
                    investment_id, e
                )
            })?;
        }

        diesel::insert_into(schema::investment_value_snapshot::table)
            .values(&NewInvestmentValueSnapshotModel {
                investment_id: *investment_id,
                value: current_value.clone(),
                snapshot_date: now,
            })
            .execute(&mut conn)
            .map_err(|e| format!("Failed to insert snapshot for {}: {}", investment_id, e))?;

        updated += 1;
    }

    Ok(PulseResult { updated, matured })
}

fn daily_noise_range(risk_level: &str) -> f64 {
    match risk_level {
        "low" => 0.005,
        "medium" => 0.01,
        "high" => 0.02,
        _ => 0.0,
    }
}

fn risk_variation_range(risk_level: &str) -> f64 {
    match risk_level {
        "low" => 0.01,
        "medium" => 0.05,
        "high" => 0.10,
        _ => 0.0,
    }
}
