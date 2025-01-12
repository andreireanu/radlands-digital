#[derive(Debug)]
pub enum CardType {
    Base,
    People,
    Event,
}

#[derive(Debug)]
pub enum Effect {
    Destroy,
    Damage,
    Injure,
    Restore,
    Draw,
    GainPunk,
    ExtraWater,
    Raid,
}

pub struct Card {
    pub card_type: CardType,
    pub name: String,
    pub description: String,
    pub front: String,
    pub back: String,
    pub junk_effect: Option<Effect>,
}
