/// Application routes for navigation between pages
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    /// Bitcoin price monitoring page
    Bitcoin,
    /// Application settings page
    Settings,
    /// About page
    About,
}

impl Default for Route {
    fn default() -> Self {
        Route::Bitcoin
    }
}

impl Route {
    /// Get the display name for the route
    pub fn display_name(&self) -> &'static str {
        match self {
            Route::Bitcoin => "Bitcoin Prices",
            Route::Settings => "Settings", 
            Route::About => "About",
        }
    }

    /// Get all available routes
    pub fn all() -> Vec<Route> {
        vec![Route::Bitcoin, Route::Settings, Route::About]
    }
}
