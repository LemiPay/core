use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;
use thiserror::Error;

use crate::application::treasury::list_user_transactions::ListUserTransactionsError;
use crate::{
    application::{
        auth::error::AuthError,
        balances::BalancesError,
        expense::ExpenseError,
        governance::GovernanceError,
        group::{
            create_group::CreateGroupError, delete_group::DeleteGroupError,
            get_group::GetGroupError, get_group_members::GetGroupMembersError,
            leave_group::LeaveGroupError, list_user_groups::ListUserGroupsError,
            make_group_admin::MakeGroupAdminError, update_group::UpdateGroupError,
        },
        treasury::{
            create_group_wallet::CreateGroupWalletError, create_user_wallet::CreateUserWalletError,
            faucet_fund_wallet::FaucetFundWalletError,
            faucet_withdraw_wallet::FaucetWithdrawWalletError, fund_group::FundGroupError,
            get_group_transaction::GetGroupTransactionError,
            get_user_wallet_by_address_and_ticker::GetUserWalletError,
            list_group_transactions::ListGroupTransactionsError,
            list_group_wallets::ListGroupWalletsError, list_user_wallets::ListUserWalletsError,
            transfer_funds::TransferFundsError,
        },
    },
    infrastructure::db::error::DbError,
};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Db(#[from] DbError),

    #[error("Not found")]
    NotFound,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal,

    #[error("Password hashing error")]
    PasswordHash(String),

    #[error("Invalid credentials")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden(String),

    #[error("Core operation failed")]
    Core,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Db(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::PasswordHash(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::Core => (StatusCode::CONFLICT, self.to_string()),
        };

        let body = Json(ErrorResponse { message });

        (status, body).into_response()
    }
}

// Mappings

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::InvalidEmail => AppError::BadRequest("Invalid email".into()),
            AuthError::InvalidName => AppError::BadRequest("Invalid name".into()),
            AuthError::EmailAlreadyExists => AppError::BadRequest("Email already exists".into()),
            AuthError::InvalidCredentials => AppError::Unauthorized,
            AuthError::InternalError => AppError::Internal,
        }
    }
}

impl From<CreateGroupError> for AppError {
    fn from(err: CreateGroupError) -> Self {
        match err {
            CreateGroupError::InvalidName => AppError::BadRequest("Invalid group name".into()),
            CreateGroupError::InvalidDescription => {
                AppError::BadRequest("Invalid group description".into())
            }
            CreateGroupError::InternalError => AppError::Internal,
        }
    }
}

impl From<GetGroupError> for AppError {
    fn from(err: GetGroupError) -> Self {
        match err {
            GetGroupError::InternalError => AppError::Internal,
        }
    }
}

impl From<LeaveGroupError> for AppError {
    fn from(err: LeaveGroupError) -> Self {
        match err {
            LeaveGroupError::NotFound => AppError::NotFound,
            LeaveGroupError::NotMember => {
                AppError::Forbidden("User is not a member of this group".into())
            }
            LeaveGroupError::LastAdminCannotLeave => {
                AppError::BadRequest("El grupo tiene que tener al menos un administrador".into())
            }
            LeaveGroupError::InternalError => AppError::Internal,
        }
    }
}

impl From<ListUserGroupsError> for AppError {
    fn from(err: ListUserGroupsError) -> Self {
        match err {
            ListUserGroupsError::InternalError => AppError::Internal,
        }
    }
}

impl From<MakeGroupAdminError> for AppError {
    fn from(err: MakeGroupAdminError) -> Self {
        match err {
            MakeGroupAdminError::Forbidden => AppError::Forbidden("Forbidden".into()),
            MakeGroupAdminError::NotFound => AppError::NotFound,
            MakeGroupAdminError::BadRequest(message) => AppError::BadRequest(message),
            MakeGroupAdminError::Internal => AppError::Internal,
        }
    }
}

impl From<UpdateGroupError> for AppError {
    fn from(err: UpdateGroupError) -> Self {
        match err {
            UpdateGroupError::Forbidden => {
                AppError::Forbidden("Solo el administrador puede actualizar el grupo.".into())
            }
            UpdateGroupError::NotFound => AppError::NotFound,
            UpdateGroupError::BadRequest(message) => AppError::BadRequest(message),
            UpdateGroupError::Internal => AppError::Internal,
        }
    }
}

impl From<DeleteGroupError> for AppError {
    fn from(err: DeleteGroupError) -> Self {
        match err {
            DeleteGroupError::Forbidden => {
                AppError::Forbidden("Solo el administrador puede borrar el grupo".into())
            }
            DeleteGroupError::NotFound => AppError::NotFound,
            DeleteGroupError::Internal => AppError::Internal,
        }
    }
}

impl From<GetGroupMembersError> for AppError {
    fn from(err: GetGroupMembersError) -> Self {
        match err {
            GetGroupMembersError::Forbidden => AppError::Forbidden("Forbidden".into()),
            GetGroupMembersError::Internal => AppError::Internal,
        }
    }
}

// ====== Treasury ======

impl From<CreateUserWalletError> for AppError {
    fn from(err: CreateUserWalletError) -> Self {
        match err {
            CreateUserWalletError::InvalidAddress => {
                AppError::BadRequest("Dirección inválida".into())
            }
            CreateUserWalletError::CurrencyNotFound => {
                AppError::BadRequest("Esa moneda no existe".into())
            }
            CreateUserWalletError::AddressTakenByOtherUser => {
                AppError::Forbidden("Esa dirección ya está tomada, elegí otra".into())
            }
            CreateUserWalletError::AddressAlreadyHasCurrency => {
                AppError::Forbidden("Esa dirección ya está registrada para esa moneda".into())
            }
            CreateUserWalletError::Internal => AppError::Internal,
        }
    }
}

impl From<FaucetFundWalletError> for AppError {
    fn from(err: FaucetFundWalletError) -> Self {
        match err {
            FaucetFundWalletError::InvalidAmount => {
                AppError::BadRequest("La cantidad tiene que ser mayor a 0".into())
            }
            FaucetFundWalletError::NotFound => AppError::NotFound,
            FaucetFundWalletError::NotOwner => {
                AppError::Forbidden("No podes fondear una wallet que no es tuya".into())
            }
            FaucetFundWalletError::Internal => AppError::Internal,
        }
    }
}

impl From<FaucetWithdrawWalletError> for AppError {
    fn from(err: FaucetWithdrawWalletError) -> Self {
        match err {
            FaucetWithdrawWalletError::InvalidAmount => {
                AppError::BadRequest("La cantidad tiene que ser mayor a 0".into())
            }
            FaucetWithdrawWalletError::NotFound => AppError::NotFound,
            FaucetWithdrawWalletError::NotOwner => {
                AppError::Forbidden("No podes retirar de una wallet que no es tuya".into())
            }
            FaucetWithdrawWalletError::InsufficientFunds => {
                AppError::BadRequest("Fondos insuficientes".into())
            }
            FaucetWithdrawWalletError::Internal => AppError::Internal,
        }
    }
}

impl From<TransferFundsError> for AppError {
    fn from(err: TransferFundsError) -> Self {
        match err {
            TransferFundsError::InvalidAmount => {
                AppError::BadRequest("La cantidad tiene que ser mayor a 0".into())
            }
            TransferFundsError::SenderWalletNotFound => {
                AppError::BadRequest("Esa wallet no existe".into())
            }
            TransferFundsError::NotOwner => {
                AppError::Forbidden("No podes retirar de una wallet que no es tuya".into())
            }
            TransferFundsError::InsufficientFunds => {
                AppError::BadRequest("Fondos insuficientes".into())
            }
            TransferFundsError::SameWalletTransfer => {
                AppError::BadRequest("No podés transferir a la misma dirección".into())
            }
            TransferFundsError::ReceiverNotFound => {
                AppError::BadRequest("Esa dirección no existe o no soporta esta moneda".into())
            }
            TransferFundsError::Internal => AppError::Internal,
        }
    }
}

impl From<ListUserWalletsError> for AppError {
    fn from(err: ListUserWalletsError) -> Self {
        match err {
            ListUserWalletsError::Internal => AppError::Internal,
        }
    }
}

impl From<GetUserWalletError> for AppError {
    fn from(err: GetUserWalletError) -> Self {
        match err {
            GetUserWalletError::CurrencyNotFound => {
                AppError::BadRequest("Esa moneda no existe".into())
            }
            GetUserWalletError::NotFound => AppError::NotFound,
            GetUserWalletError::NotOwner => AppError::Forbidden("Esta wallet no es tuya".into()),
            GetUserWalletError::Internal => AppError::Internal,
        }
    }
}

impl From<CreateGroupWalletError> for AppError {
    fn from(err: CreateGroupWalletError) -> Self {
        match err {
            CreateGroupWalletError::InvalidAddress => {
                AppError::BadRequest("Dirección inválida".into())
            }
            CreateGroupWalletError::CurrencyNotFound => {
                AppError::BadRequest("Esa moneda no existe".into())
            }
            CreateGroupWalletError::GroupAlreadyHasCurrency => {
                AppError::BadRequest("El grupo ya tiene una wallet para esa moneda".into())
            }
            CreateGroupWalletError::AddressAlreadyTaken => {
                AppError::BadRequest("Esa dirección ya está registrada para esa moneda".into())
            }
            CreateGroupWalletError::Internal => AppError::Internal,
        }
    }
}

impl From<ListGroupWalletsError> for AppError {
    fn from(err: ListGroupWalletsError) -> Self {
        match err {
            ListGroupWalletsError::Internal => AppError::Internal,
        }
    }
}

impl From<FundGroupError> for AppError {
    fn from(err: FundGroupError) -> Self {
        match err {
            FundGroupError::InvalidAmount => {
                AppError::BadRequest("El monto debe ser mayor a 0".into())
            }
            FundGroupError::UserWalletNotFound => {
                AppError::BadRequest("El usuario no tiene una wallet para esta moneda".into())
            }
            FundGroupError::GroupWalletNotFound => {
                AppError::BadRequest("El grupo no tiene una wallet para esta moneda".into())
            }
            FundGroupError::InsufficientFunds => AppError::BadRequest("Saldo insuficiente".into()),
            FundGroupError::Internal => AppError::Internal,
        }
    }
}

impl From<ListGroupTransactionsError> for AppError {
    fn from(err: ListGroupTransactionsError) -> Self {
        match err {
            ListGroupTransactionsError::Internal => AppError::Internal,
        }
    }
}

impl From<ListUserTransactionsError> for AppError {
    fn from(err: ListUserTransactionsError) -> Self {
        match err {
            ListUserTransactionsError::Internal => AppError::Internal,
        }
    }
}

impl From<GetGroupTransactionError> for AppError {
    fn from(err: GetGroupTransactionError) -> Self {
        match err {
            GetGroupTransactionError::NotFound => AppError::NotFound,
            GetGroupTransactionError::Internal => AppError::Internal,
        }
    }
}

impl From<GovernanceError> for AppError {
    fn from(err: GovernanceError) -> Self {
        match err {
            GovernanceError::NotFound => AppError::NotFound,
            GovernanceError::Internal => AppError::Internal,
            GovernanceError::InvalidAmount => {
                AppError::BadRequest("El monto debe ser mayor a 0".into())
            }
            GovernanceError::InvalidEmail => AppError::BadRequest("Email inválido".into()),
            GovernanceError::MissingProposalTarget => {
                AppError::BadRequest("Se debe enviar user_id o user_email".into())
            }
            GovernanceError::UserNotFound => AppError::BadRequest("Usuario no encontrado".into()),
            GovernanceError::UserAlreadyMember => {
                AppError::BadRequest("El usuario ya pertenece al grupo".into())
            }
            GovernanceError::NotProposalDestination => {
                AppError::Forbidden("No podés responder una propuesta que no es tuya".into())
            }
            GovernanceError::NotProposalCreator => {
                AppError::Forbidden("Solo el creador puede ejecutar/cancelar la propuesta".into())
            }
            GovernanceError::ProposalAlreadyResolved => {
                AppError::BadRequest("La propuesta ya fue resuelta".into())
            }
            GovernanceError::GroupMismatch => {
                AppError::BadRequest("La propuesta no pertenece al grupo".into())
            }
            GovernanceError::FundRoundNotActive => {
                AppError::BadRequest("La ronda de fondeo no está activa".into())
            }
            GovernanceError::ContributionExceedsTarget => AppError::BadRequest(
                "El aporte excede el objetivo restante de la ronda de fondeo".into(),
            ),
            GovernanceError::InvalidStatusTransition => {
                AppError::BadRequest("Transición de estado inválida".into())
            }
            GovernanceError::SenderWalletNotFound => {
                AppError::BadRequest("La wallet del usuario no existe para esa moneda".into())
            }
        }
    }
}

impl From<ExpenseError> for AppError {
    fn from(err: ExpenseError) -> Self {
        match err {
            ExpenseError::NotFound => AppError::NotFound,
            ExpenseError::Internal => AppError::Internal,
            ExpenseError::InvalidAmount => {
                AppError::BadRequest("El monto debe ser mayor a 0".into())
            }
            ExpenseError::EmptyParticipants => {
                AppError::BadRequest("El gasto debe tener al menos un participante".into())
            }
            ExpenseError::DuplicatedParticipant => {
                AppError::BadRequest("Hay un participante duplicado en el gasto".into())
            }
            ExpenseError::TooManyParticipants => {
                AppError::BadRequest("Hay demasiados participantes".into())
            }
            ExpenseError::NoFieldsToUpdate => {
                AppError::BadRequest("No hay campos para actualizar".into())
            }
            ExpenseError::AlreadyDeleted => {
                AppError::BadRequest("El gasto ya fue eliminado".into())
            }
            ExpenseError::NotOwner => {
                AppError::Forbidden("Solo el creador o el admin pueden editar".into())
            }
            ExpenseError::GroupMismatch => AppError::NotFound,
        }
    }
}

impl From<BalancesError> for AppError {
    fn from(err: BalancesError) -> Self {
        match err {
            BalancesError::Internal => AppError::Internal,
            BalancesError::UserNotFound => AppError::Core,
            BalancesError::InsufficientFunds => AppError::Core,
        }
    }
}
