use seed::{*};

use crate::constants;
use crate::pages;

struct_urls!();

impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn dashboard(self) -> pages::dashboard::Urls<'a> {
        pages::dashboard::Urls::new(self.base_url().add_hash_path_part(constants::DASHBOARD))
    }
    pub fn team(self) -> pages::team::Urls<'a> {
        pages::team::Urls::new(self.base_url().add_hash_path_part(constants::TEAM))
    }
    pub fn projects(self) -> pages::projects::Urls<'a> {
        pages::projects::Urls::new(self.base_url().add_hash_path_part(constants::PROJECTS))
    }
    pub fn calendar(self) -> pages::calendar::Urls<'a> {
        pages::calendar::Urls::new(self.base_url().add_hash_path_part(constants::CALENDAR))
    }
    pub fn reports(self) -> pages::reports::Urls<'a> {
        pages::reports::Urls::new(self.base_url().add_hash_path_part(constants::REPORTS))
    }
    pub fn authentication(self) -> pages::authentication::urls::Urls<'a> {
        pages::authentication::urls::Urls::new(
            self.base_url()
                .add_hash_path_part(constants::AUTHENTICATION),
        )
    }
}
