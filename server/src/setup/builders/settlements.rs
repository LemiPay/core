use crate::application::balances::BalancesService;
use crate::application::settlements::get_settlements::GetSettlementsUseCase;
use crate::application::settlements::service::SettlementsService;

pub fn build_settlements_service(balances_service: BalancesService) -> SettlementsService {
    SettlementsService {
        get_settlements: GetSettlementsUseCase { balances_service },
    }
}
