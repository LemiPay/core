use crate::application::settlements::get_settlements::GetSettlementsUseCase;
use crate::application::settlements::pay_settlement::PaySettlementUseCase;

pub struct SettlementsService {
    pub get_settlements: GetSettlementsUseCase,
    pub pay_settlement: PaySettlementUseCase,
}
