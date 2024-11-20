//! A service is any business logic that can be called from the API. That being said, endpoints should do minimal logic and instead call a service to do the heavy lifting,
//! this allows us to call the functions of more complex logic from within the API.

pub(crate) mod health;
pub(crate) mod token;
