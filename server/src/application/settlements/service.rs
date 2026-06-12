use crate::application::settlements::get_settlements::GetSettlementsUseCase;

pub struct SettlementsService {
    pub get_settlements: GetSettlementsUseCase,
}
