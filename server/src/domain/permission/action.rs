use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    // ── Group ──
    UpdateGroup,
    DeleteGroup,
    EnterDebtResolution,
    InviteMember,

    // ── Governance ──
    CancelProposal,
    CreateFundRound,
    CancelFundRound,

    // ── Expense ──
    ManageAnyExpense,

    // ── Investment ──
    CreateInvestment,
}

impl Action {
    pub const ALL: &[Action] = &[
        Action::UpdateGroup,
        Action::DeleteGroup,
        Action::EnterDebtResolution,
        Action::InviteMember,
        Action::CancelProposal,
        Action::CreateFundRound,
        Action::CancelFundRound,
        Action::ManageAnyExpense,
        Action::CreateInvestment,
    ];

    pub fn name(&self) -> &str {
        match self {
            Action::UpdateGroup => "update_group",
            Action::DeleteGroup => "delete_group",
            Action::EnterDebtResolution => "enter_debt_resolution",
            Action::InviteMember => "invite_member",
            Action::CancelProposal => "cancel_proposal",
            Action::CreateFundRound => "create_fund_round",
            Action::CancelFundRound => "cancel_fund_round",
            Action::ManageAnyExpense => "manage_any_expense",
            Action::CreateInvestment => "create_investment",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Action::UpdateGroup => "Actualizar nombre y descripción del grupo",
            Action::DeleteGroup => "Eliminar o finalizar el grupo",
            Action::EnterDebtResolution => "Iniciar resolución de deudas",
            Action::InviteMember => "Invitar nuevos miembros al grupo",
            Action::CancelProposal => "Cancelar propuestas pendientes",
            Action::CreateFundRound => "Crear rondas de fondeo",
            Action::CancelFundRound => "Cancelar rondas de fondeo",
            Action::ManageAnyExpense => "Editar o eliminar cualquier gasto del grupo",
            Action::CreateInvestment => "Crear propuestas de inversión",
        }
    }

    pub fn category(&self) -> ActionCategory {
        match self {
            Action::UpdateGroup
            | Action::DeleteGroup
            | Action::InviteMember
            | Action::EnterDebtResolution => ActionCategory::Group,

            Action::CancelProposal | Action::CreateFundRound | Action::CancelFundRound => {
                ActionCategory::Governance
            }

            Action::ManageAnyExpense => ActionCategory::Expense,

            Action::CreateInvestment => ActionCategory::Investment,
        }
    }

    pub fn from_name(s: &str) -> Option<Action> {
        match s {
            "update_group" => Some(Action::UpdateGroup),
            "delete_group" => Some(Action::DeleteGroup),
            "enter_debt_resolution" => Some(Action::EnterDebtResolution),
            "invite_member" => Some(Action::InviteMember),
            "cancel_proposal" => Some(Action::CancelProposal),
            "create_fund_round" => Some(Action::CreateFundRound),
            "cancel_fund_round" => Some(Action::CancelFundRound),
            "manage_any_expense" => Some(Action::ManageAnyExpense),
            "create_investment" => Some(Action::CreateInvestment),
            _ => None,
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionCategory {
    Group,
    Governance,
    Expense,
    Investment,
}

impl ActionCategory {
    pub fn name(&self) -> &str {
        match self {
            ActionCategory::Group => "Grupo",
            ActionCategory::Governance => "Gobernanza",
            ActionCategory::Expense => "Gastos",
            ActionCategory::Investment => "Inversiones",
        }
    }
}
