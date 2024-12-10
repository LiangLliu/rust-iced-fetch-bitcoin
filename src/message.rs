use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum BitcoinMessage {
    Refetch,
    CurrentPrice((f64, HashMap<String, f64>)),
}

