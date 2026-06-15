// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "blockchain"))]
    pub struct Blockchain;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "expense_status"))]
    pub struct ExpenseStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "group_member_status"))]
    pub struct GroupMemberStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "group_role"))]
    pub struct GroupRole;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "group_status"))]
    pub struct GroupStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "investment_status"))]
    pub struct InvestmentStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "proposal_status"))]
    pub struct ProposalStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transaction_type"))]
    pub struct TransactionType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "vote_type"))]
    pub struct VoteType;
}

diesel::table! {
    blockchain_event (id) {
        id -> Uuid,
        event_type -> Text,
        sender -> Text,
        wallet_address -> Text,
        token_address -> Text,
        currency_id -> Uuid,
        gross_amount -> Numeric,
        fee_amount -> Numeric,
        net_amount -> Numeric,
        tx_hash -> Text,
        block_number -> Int8,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    blockchain_sync_state (sync_key) {
        sync_key -> Text,
        last_processed_block -> Int8,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Blockchain;

    currency (currency_id) {
        currency_id -> Uuid,
        name -> Text,
        ticker -> Text,
        blockchain -> Blockchain,
        token_address -> Text,
        token_currency_id -> Nullable<Text>,
        decimals -> Int2,
        is_active -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ExpenseStatus;

    expense (expense_id) {
        expense_id -> Uuid,
        user_id -> Uuid,
        currency_id -> Uuid,
        group_id -> Uuid,
        description -> Nullable<Text>,
        amount -> Numeric,
        status -> ExpenseStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    expense_participant (expense_id, user_id) {
        expense_id -> Uuid,
        user_id -> Uuid,
        amount -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    fund_round_contribution (fund_round_proposal_id, user_id) {
        fund_round_proposal_id -> Uuid,
        user_id -> Uuid,
        amount -> Numeric,
        transaction_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    fund_round_proposal (proposal_id) {
        proposal_id -> Uuid,
        target_amount -> Numeric,
        currency_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::GroupStatus;

    group (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        status -> GroupStatus,
        created_at -> Date,
        updated_at -> Date,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::GroupRole;

    group_permission (group_id, role, action) {
        group_id -> Uuid,
        role -> GroupRole,
        #[max_length = 50]
        action -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    group_wallet (id) {
        id -> Uuid,
        address -> Text,
        group_id -> Uuid,
        currency_id -> Uuid,
        balance -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::InvestmentStatus;

    investment (id) {
        id -> Uuid,
        proposal_id -> Uuid,
        amount -> Numeric,
        current_value -> Numeric,
        actual_return -> Nullable<Numeric>,
        status -> InvestmentStatus,
        started_at -> Timestamp,
        matures_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    investment_member (id) {
        id -> Uuid,
        investment_id -> Uuid,
        user_id -> Uuid,
        balance_at_investment -> Numeric,
        participation_pct -> Numeric,
        invested_amount -> Numeric,
        returned_amount -> Nullable<Numeric>,
        withdrawn_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    investment_proposal (proposal_id) {
        proposal_id -> Uuid,
        amount -> Numeric,
        strategy_id -> Uuid,
        currency_id -> Uuid,
    }
}

diesel::table! {
    investment_strategy (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        risk_level -> Text,
        expected_return_percentage -> Numeric,
        duration_days -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    investment_value_snapshot (id) {
        id -> Uuid,
        investment_id -> Uuid,
        value -> Numeric,
        snapshot_date -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    new_member_proposal (proposal_id) {
        proposal_id -> Uuid,
        new_member_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProposalStatus;

    proposal (id) {
        id -> Uuid,
        group_id -> Uuid,
        created_by -> Uuid,
        status -> ProposalStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TransactionType;

    transaction (id) {
        id -> Uuid,
        tx_hash -> Nullable<Text>,
        amount -> Numeric,
        user_id -> Uuid,
        group_id -> Uuid,
        currency_id -> Uuid,
        address -> Text,
        description -> Nullable<Text>,
        tx_type -> TransactionType,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    transaction_participant (transaction_id, user_id) {
        transaction_id -> Uuid,
        user_id -> Uuid,
        amount -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        email -> Text,
        password -> Nullable<Text>,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::GroupRole;
    use super::sql_types::GroupMemberStatus;

    user_in_group (user_id, group_id) {
        user_id -> Uuid,
        group_id -> Uuid,
        role -> GroupRole,
        status -> GroupMemberStatus,
        joined_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_wallet (id) {
        id -> Uuid,
        address -> Text,
        user_id -> Uuid,
        currency_id -> Uuid,
        balance -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::VoteType;

    vote (proposal_id, user_id) {
        proposal_id -> Uuid,
        user_id -> Uuid,
        value -> VoteType,
        created_at -> Timestamp,
    }
}

diesel::table! {
    withdraw_proposal (proposal_id) {
        proposal_id -> Uuid,
        amount -> Numeric,
        currency_id -> Uuid,
    }
}

diesel::joinable!(blockchain_event -> currency (currency_id));
diesel::joinable!(expense -> currency (currency_id));
diesel::joinable!(expense -> group (group_id));
diesel::joinable!(expense -> user (user_id));
diesel::joinable!(expense_participant -> expense (expense_id));
diesel::joinable!(expense_participant -> user (user_id));
diesel::joinable!(fund_round_contribution -> fund_round_proposal (fund_round_proposal_id));
diesel::joinable!(fund_round_contribution -> transaction (transaction_id));
diesel::joinable!(fund_round_contribution -> user (user_id));
diesel::joinable!(fund_round_proposal -> currency (currency_id));
diesel::joinable!(fund_round_proposal -> proposal (proposal_id));
diesel::joinable!(group_permission -> group (group_id));
diesel::joinable!(group_wallet -> currency (currency_id));
diesel::joinable!(group_wallet -> group (group_id));
diesel::joinable!(investment -> investment_proposal (proposal_id));
diesel::joinable!(investment_member -> investment (investment_id));
diesel::joinable!(investment_member -> user (user_id));
diesel::joinable!(investment_proposal -> currency (currency_id));
diesel::joinable!(investment_proposal -> investment_strategy (strategy_id));
diesel::joinable!(investment_proposal -> proposal (proposal_id));
diesel::joinable!(investment_value_snapshot -> investment (investment_id));
diesel::joinable!(new_member_proposal -> proposal (proposal_id));
diesel::joinable!(new_member_proposal -> user (new_member_id));
diesel::joinable!(proposal -> group (group_id));
diesel::joinable!(proposal -> user (created_by));
diesel::joinable!(transaction -> currency (currency_id));
diesel::joinable!(transaction -> group (group_id));
diesel::joinable!(transaction -> user (user_id));
diesel::joinable!(transaction_participant -> transaction (transaction_id));
diesel::joinable!(transaction_participant -> user (user_id));
diesel::joinable!(user_in_group -> group (group_id));
diesel::joinable!(user_in_group -> user (user_id));
diesel::joinable!(user_wallet -> currency (currency_id));
diesel::joinable!(user_wallet -> user (user_id));
diesel::joinable!(vote -> proposal (proposal_id));
diesel::joinable!(vote -> user (user_id));
diesel::joinable!(withdraw_proposal -> currency (currency_id));
diesel::joinable!(withdraw_proposal -> proposal (proposal_id));

diesel::allow_tables_to_appear_in_same_query!(
    blockchain_event,
    blockchain_sync_state,
    currency,
    expense,
    expense_participant,
    fund_round_contribution,
    fund_round_proposal,
    group,
    group_permission,
    group_wallet,
    investment,
    investment_member,
    investment_proposal,
    investment_strategy,
    investment_value_snapshot,
    new_member_proposal,
    proposal,
    transaction,
    transaction_participant,
    user,
    user_in_group,
    user_wallet,
    vote,
    withdraw_proposal,
);
