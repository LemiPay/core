use bigdecimal::BigDecimal;

use crate::application::balances::dto::GroupBalancesDetails;
use crate::application::expense::dto::ExpenseDetails;
use crate::application::governance::dto::{
    FundRoundProposalDetails, NewMemberProposalDetails, WithdrawProposalDetails,
};
use crate::application::group::dto::{GroupFromUserDetails, GroupMemberDetails};
use crate::application::investment::dto::InvestmentDetails;
use crate::application::settlements::get_settlements::dto::SettlementDetails;

const INDENT: &str = "  ";

pub fn format_user_groups(
    groups: &[GroupFromUserDetails],
    balances: &[(&GroupBalancesDetails, &str)],
) -> String {
    let mut out = String::from("USER GROUPS:\n");

    for group in groups {
        let own_balance = balances
            .iter()
            .find(|(_, id)| *id == group.group_id.to_string())
            .and_then(|(b, _)| {
                b.balances
                    .iter()
                    .find(|u| u.user_id == group.user_id)
                    .map(|u| &u.balance)
            });

        let balance_str = match own_balance {
            Some(b) if *b > BigDecimal::from(0) => {
                format!(" - You are owed {}", b)
            }
            Some(b) if *b < BigDecimal::from(0) => {
                format!(" - You owe {}", b.abs())
            }
            _ => " - Settled".to_string(),
        };

        out.push_str(&format!(
            "- \"{}\" [{}] ({}){} \n",
            group.group_name,
            fmt(&group.status),
            fmt(&group.role),
            balance_str
        ));
    }

    out
}

pub fn format_group_balances(balances: &GroupBalancesDetails, user_id: &uuid::Uuid) -> String {
    let mut out = String::new();

    out.push_str(&format!("Group Balance: {}\n", balances.group_balance));

    let own_balance = balances
        .balances
        .iter()
        .find(|u| u.user_id == *user_id)
        .map(|u| &u.balance);

    if let Some(b) = own_balance {
        if *b > BigDecimal::from(0) {
            out.push_str(&format!("Your Balance: +{} (creditor)\n", b));
        } else if *b < BigDecimal::from(0) {
            out.push_str(&format!("Your Balance: {} (debtor)\n", b));
        } else {
            out.push_str("Your Balance: 0 (settled)\n");
        }
    }

    out.push_str("Member Balances:\n");
    for member in &balances.balances {
        let sign = if member.balance > BigDecimal::from(0) {
            "+"
        } else {
            ""
        };
        let label = if member.user_id == *user_id {
            "you"
        } else {
            &member.user_name
        };
        out.push_str(&format!(
            "{}{}: {}{}\n",
            INDENT, label, sign, member.balance
        ));
    }

    out
}

pub fn format_members(members: &[GroupMemberDetails]) -> String {
    let mut out = String::from("Members:\n");
    for m in members {
        out.push_str(&format!(
            "{}- {} ({}) [{}]\n",
            INDENT,
            m.name,
            fmt(&m.status),
            fmt(&m.role)
        ));
    }
    out
}

pub fn format_expenses(expenses: &[ExpenseDetails]) -> String {
    if expenses.is_empty() {
        return String::new();
    }

    let mut out = String::from("Recent Expenses:\n");
    for expense in expenses.iter().rev().take(10) {
        let desc = expense.description.as_deref().unwrap_or("no description");
        out.push_str(&format!(
            "{}- {} paid {} for \"{}\"\n",
            INDENT, expense.user_id, expense.amount, desc
        ));
    }
    out
}

pub fn format_withdraw_proposals(proposals: &[WithdrawProposalDetails]) -> String {
    if proposals.is_empty() {
        return String::new();
    }

    let mut out = String::from("Withdraw Proposals:\n");
    for p in proposals {
        out.push_str(&format!(
            "{}- {} {} (status: {})\n",
            INDENT,
            p.amount,
            fmt(&p.kind),
            fmt(&p.proposal.status)
        ));
    }
    out
}

pub fn format_new_member_proposals(proposals: &[NewMemberProposalDetails]) -> String {
    if proposals.is_empty() {
        return String::new();
    }

    let mut out = String::from("New Member Proposals:\n");
    for p in proposals {
        out.push_str(&format!(
            "{}- New member {} (status: {})\n",
            INDENT,
            p.new_member_id,
            fmt(&p.proposal.status)
        ));
    }
    out
}

pub fn format_fund_rounds(rounds: &[FundRoundProposalDetails]) -> String {
    if rounds.is_empty() {
        return String::new();
    }

    let mut out = String::from("Fund Rounds:\n");
    for r in rounds {
        out.push_str(&format!(
            "{}- Target: {} (status: {})\n",
            INDENT,
            r.target_amount,
            fmt(&r.proposal.status)
        ));
    }
    out
}

pub fn format_investments(investments: &[InvestmentDetails]) -> String {
    if investments.is_empty() {
        return String::new();
    }

    let mut out = String::from("Investments:\n");
    for inv in investments {
        out.push_str(&format!(
            "{}- {}: {} invested -> current value: {} (status: {}, risk: {})\n",
            INDENT,
            inv.strategy_name,
            inv.amount,
            inv.current_value,
            fmt(&inv.status),
            inv.risk_level
        ));
    }
    out
}

pub fn format_settlements(settlements: &[SettlementDetails]) -> String {
    if settlements.is_empty() {
        return String::new();
    }

    let mut out = String::from("Pending Settlements:\n");
    for s in settlements {
        let from = s
            .from_name
            .as_ref()
            .map(|n| n.to_string())
            .unwrap_or_else(|| "someone".to_string());
        let to = s
            .to_name
            .as_ref()
            .map(|n| n.to_string())
            .unwrap_or_else(|| "someone".to_string());
        out.push_str(&format!("{}- {} owes {} {}\n", INDENT, from, to, s.amount));
    }
    out
}

fn fmt<T: std::fmt::Debug>(value: &T) -> String {
    format!("{:?}", value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use chrono::NaiveDateTime;
    use uuid::Uuid;

    use crate::application::balances::dto::UserBalanceDetails;
    use crate::application::governance::dto::ProposalDetails;
    use crate::domain::expense::ExpenseStatus;
    use crate::domain::governance::{ProposalKind, ProposalStatus};
    use crate::domain::investment::InvestmentStatus;
    use crate::domain::user::{UserId, UserName};
    use crate::infrastructure::db::models::group::{
        GroupMemberStatusModel, GroupRoleModel, GroupStatusModel,
    };

    fn id(n: u8) -> Uuid {
        Uuid::from_bytes([n; 16])
    }

    fn dt() -> NaiveDateTime {
        NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        )
    }

    // -------------------------------------------------------
    // format_user_groups
    // -------------------------------------------------------

    #[test]
    fn test_format_user_groups_empty() {
        assert_eq!(format_user_groups(&[], &[]), "USER GROUPS:\n");
    }

    #[test]
    fn test_format_user_groups_debtor() {
        let uid = id(1);
        let gid = id(2);
        let groups = vec![GroupFromUserDetails {
            user_id: uid,
            group_id: gid,
            role: GroupRoleModel::Member,
            group_name: "Viaje".into(),
            group_description: "".into(),
            status: GroupStatusModel::Active,
        }];
        let balance = GroupBalancesDetails {
            group_balance: BigDecimal::from(500),
            balances: vec![UserBalanceDetails {
                user_id: uid,
                user_name: "you".into(),
                balance: BigDecimal::from(-150),
            }],
        };
        let out = format_user_groups(&groups, &[(&balance, &gid.to_string())]);
        assert!(out.contains("Viaje"));
        assert!(out.contains("owe 150"));
    }

    #[test]
    fn test_format_user_groups_creditor() {
        let uid = id(1);
        let gid = id(2);
        let groups = vec![GroupFromUserDetails {
            user_id: uid,
            group_id: gid,
            role: GroupRoleModel::Admin,
            group_name: "Casa".into(),
            group_description: "".into(),
            status: GroupStatusModel::Active,
        }];
        let balance = GroupBalancesDetails {
            group_balance: BigDecimal::from(300),
            balances: vec![UserBalanceDetails {
                user_id: uid,
                user_name: "you".into(),
                balance: BigDecimal::from(200),
            }],
        };
        let out = format_user_groups(&groups, &[(&balance, &gid.to_string())]);
        assert!(out.contains("Casa"));
        assert!(out.contains("owed 200"));
    }

    #[test]
    fn test_format_user_groups_settled() {
        let uid = id(1);
        let gid = id(2);
        let groups = vec![GroupFromUserDetails {
            user_id: uid,
            group_id: gid,
            role: GroupRoleModel::Member,
            group_name: "Libros".into(),
            group_description: "".into(),
            status: GroupStatusModel::DebtResolution,
        }];
        let balance = GroupBalancesDetails {
            group_balance: BigDecimal::from(0),
            balances: vec![UserBalanceDetails {
                user_id: uid,
                user_name: "you".into(),
                balance: BigDecimal::from(0),
            }],
        };
        let out = format_user_groups(&groups, &[(&balance, &gid.to_string())]);
        assert!(out.contains("Settled"));
    }

    #[test]
    fn test_format_user_groups_multiple() {
        let uid = id(1);
        let gid1 = id(10);
        let gid2 = id(20);
        let groups = vec![
            GroupFromUserDetails {
                user_id: uid,
                group_id: gid1,
                role: GroupRoleModel::Member,
                group_name: "A".into(),
                group_description: "".into(),
                status: GroupStatusModel::Active,
            },
            GroupFromUserDetails {
                user_id: uid,
                group_id: gid2,
                role: GroupRoleModel::Admin,
                group_name: "B".into(),
                group_description: "".into(),
                status: GroupStatusModel::Ended,
            },
        ];
        let bal1 = GroupBalancesDetails {
            group_balance: BigDecimal::from(100),
            balances: vec![UserBalanceDetails {
                user_id: uid,
                user_name: "you".into(),
                balance: BigDecimal::from(-10),
            }],
        };
        let bal2 = GroupBalancesDetails {
            group_balance: BigDecimal::from(0),
            balances: vec![UserBalanceDetails {
                user_id: uid,
                user_name: "you".into(),
                balance: BigDecimal::from(0),
            }],
        };
        let out = format_user_groups(
            &groups,
            &[(&bal1, &gid1.to_string()), (&bal2, &gid2.to_string())],
        );
        assert!(out.contains("\"A\""));
        assert!(out.contains("\"B\""));
        assert!(out.contains("owe 10"));
        assert!(out.contains("Settled"));
    }

    // -------------------------------------------------------
    // format_group_balances
    // -------------------------------------------------------

    #[test]
    fn test_format_group_balances_creditor() {
        let uid = id(1);
        let balances = GroupBalancesDetails {
            group_balance: BigDecimal::from(1000),
            balances: vec![
                UserBalanceDetails {
                    user_id: uid,
                    user_name: "you".into(),
                    balance: BigDecimal::from(300),
                },
                UserBalanceDetails {
                    user_id: id(2),
                    user_name: "Alice".into(),
                    balance: BigDecimal::from(-100),
                },
            ],
        };
        let out = format_group_balances(&balances, &uid);
        assert!(out.contains("Group Balance: 1000"));
        assert!(out.contains("+300"));
        assert!(out.contains("creditor"));
        assert!(out.contains("Alice: -100"));
        assert!(out.contains("you: +300"));
    }

    #[test]
    fn test_format_group_balances_debtor() {
        let uid = id(1);
        let balances = GroupBalancesDetails {
            group_balance: BigDecimal::from(500),
            balances: vec![UserBalanceDetails {
                user_id: uid,
                user_name: "you".into(),
                balance: BigDecimal::from(-200),
            }],
        };
        let out = format_group_balances(&balances, &uid);
        assert!(out.contains("-200"));
        assert!(out.contains("debtor"));
    }

    #[test]
    fn test_format_group_balances_zero() {
        let uid = id(1);
        let balances = GroupBalancesDetails {
            group_balance: BigDecimal::from(0),
            balances: vec![UserBalanceDetails {
                user_id: uid,
                user_name: "you".into(),
                balance: BigDecimal::from(0),
            }],
        };
        let out = format_group_balances(&balances, &uid);
        assert!(out.contains("0 (settled)"));
    }

    // -------------------------------------------------------
    // format_members
    // -------------------------------------------------------

    #[test]
    fn test_format_members_empty() {
        let out = format_members(&[]);
        assert_eq!(out, "Members:\n");
    }

    #[test]
    fn test_format_members_with_data() {
        let uid = id(1);
        let members = vec![
            GroupMemberDetails {
                user_id: uid,
                group_id: id(99),
                name: "Alice".into(),
                email: "alice@test.com".into(),
                status: GroupMemberStatusModel::Active,
                role: GroupRoleModel::Admin,
            },
            GroupMemberDetails {
                user_id: id(2),
                group_id: id(99),
                name: "Bob".into(),
                email: "bob@test.com".into(),
                status: GroupMemberStatusModel::Active,
                role: GroupRoleModel::Member,
            },
        ];
        let out = format_members(&members);
        assert!(out.contains("Alice"));
        assert!(out.contains("Bob"));
        assert!(out.contains("Admin"));
        assert!(out.contains("Member"));
    }

    // -------------------------------------------------------
    // format_expenses
    // -------------------------------------------------------

    #[test]
    fn test_format_expenses_empty() {
        assert_eq!(format_expenses(&[]), "");
    }

    #[test]
    fn test_format_expenses_with_data() {
        let expense = ExpenseDetails {
            expense_id: id(1),
            user_id: id(10),
            currency_id: id(99),
            group_id: id(99),
            description: Some("pizza".into()),
            amount: BigDecimal::from(60),
            status: ExpenseStatus::Created,
            created_at: dt(),
            updated_at: dt(),
        };
        let out = format_expenses(&[expense]);
        assert!(out.contains("Recent Expenses:"));
        assert!(out.contains("60"));
        assert!(out.contains("pizza"));
    }

    #[test]
    fn test_format_expenses_no_description() {
        let expense = ExpenseDetails {
            expense_id: id(1),
            user_id: id(10),
            currency_id: id(99),
            group_id: id(99),
            description: None,
            amount: BigDecimal::from(30),
            status: ExpenseStatus::Created,
            created_at: dt(),
            updated_at: dt(),
        };
        let out = format_expenses(&[expense]);
        assert!(out.contains("no description"));
    }

    #[test]
    fn test_format_expenses_takes_last_10() {
        let expenses: Vec<_> = (0..15)
            .map(|i| ExpenseDetails {
                expense_id: id(i),
                user_id: id(10),
                currency_id: id(99),
                group_id: id(99),
                description: Some(format!("e{}", i)),
                amount: BigDecimal::from(i * 10),
                status: ExpenseStatus::Created,
                created_at: dt(),
                updated_at: dt(),
            })
            .collect();
        let out = format_expenses(&expenses);
        assert!(out.contains("e14"));
        assert!(!out.contains("e0"));
    }

    // -------------------------------------------------------
    // format_withdraw_proposals
    // -------------------------------------------------------

    #[test]
    fn test_format_withdraw_proposals_empty() {
        assert_eq!(format_withdraw_proposals(&[]), "");
    }

    #[test]
    fn test_format_withdraw_proposals_pending() {
        let p = WithdrawProposalDetails {
            proposal: ProposalDetails {
                id: id(1),
                group_id: id(99),
                created_by: id(10),
                status: ProposalStatus::Pending,
                created_at: dt(),
                updated_at: dt(),
            },
            amount: BigDecimal::from(200),
            currency_id: id(50),
            kind: ProposalKind::Withdraw,
        };
        let out = format_withdraw_proposals(&[p]);
        assert!(out.contains("200"));
        assert!(out.contains("Pending"));
    }

    // -------------------------------------------------------
    // format_new_member_proposals
    // -------------------------------------------------------

    #[test]
    fn test_format_new_member_proposals_empty() {
        assert_eq!(format_new_member_proposals(&[]), "");
    }

    #[test]
    fn test_format_new_member_proposals_pending() {
        let p = NewMemberProposalDetails {
            proposal: ProposalDetails {
                id: id(1),
                group_id: id(99),
                created_by: id(10),
                status: ProposalStatus::Pending,
                created_at: dt(),
                updated_at: dt(),
            },
            new_member_id: id(42),
            kind: ProposalKind::NewMember,
        };
        let out = format_new_member_proposals(&[p]);
        assert!(out.contains("New member"));
        assert!(out.contains(&id(42).to_string()));
        assert!(out.contains("Pending"));
    }

    // -------------------------------------------------------
    // format_fund_rounds
    // -------------------------------------------------------

    #[test]
    fn test_format_fund_rounds_empty() {
        assert_eq!(format_fund_rounds(&[]), "");
    }

    #[test]
    fn test_format_fund_rounds_pending() {
        let r = FundRoundProposalDetails {
            proposal: ProposalDetails {
                id: id(1),
                group_id: id(99),
                created_by: id(10),
                status: ProposalStatus::Approved,
                created_at: dt(),
                updated_at: dt(),
            },
            target_amount: BigDecimal::from(1000),
            currency_id: id(50),
            kind: ProposalKind::FundRound,
        };
        let out = format_fund_rounds(&[r]);
        assert!(out.contains("1000"));
        assert!(out.contains("Approved"));
    }

    // -------------------------------------------------------
    // format_investments
    // -------------------------------------------------------

    #[test]
    fn test_format_investments_empty() {
        assert_eq!(format_investments(&[]), "");
    }

    #[test]
    fn test_format_investments_active() {
        let inv = InvestmentDetails {
            id: id(1),
            group_id: id(99),
            proposal_id: id(10),
            strategy_id: id(20),
            currency_id: id(50),
            amount: BigDecimal::from(500),
            current_value: BigDecimal::from(550),
            actual_return: Some(BigDecimal::from(50)),
            status: InvestmentStatus::Active,
            started_at: dt(),
            matures_at: dt(),
            created_at: dt(),
            updated_at: dt(),
            strategy_name: "Conservative".into(),
            risk_level: "low".into(),
            expected_return_percentage: BigDecimal::from(10),
        };
        let out = format_investments(&[inv]);
        assert!(out.contains("Conservative"));
        assert!(out.contains("500"));
        assert!(out.contains("550"));
        assert!(out.contains("Active"));
        assert!(out.contains("low"));
    }

    // -------------------------------------------------------
    // format_settlements
    // -------------------------------------------------------

    #[test]
    fn test_format_settlements_empty() {
        assert_eq!(format_settlements(&[]), "");
    }

    #[test]
    fn test_format_settlements_with_names() {
        let s = SettlementDetails {
            from: UserId(id(1)),
            to: UserId(id(2)),
            amount: BigDecimal::from(100),
            from_name: Some(UserName("Alice".into())),
            to_name: Some(UserName("Bob".into())),
        };
        let out = format_settlements(&[s]);
        assert!(out.contains("Alice"));
        assert!(out.contains("Bob"));
        assert!(out.contains("100"));
        assert!(out.contains("owes"));
    }

    #[test]
    fn test_format_settlements_without_names() {
        let s = SettlementDetails {
            from: UserId(id(1)),
            to: UserId(id(2)),
            amount: BigDecimal::from(50),
            from_name: None,
            to_name: None,
        };
        let out = format_settlements(&[s]);
        assert!(out.contains("someone"));
    }
}
