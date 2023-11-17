// pub trait IAppState: Send + Sync + 'static {}

// #[nject::provider]
pub struct Provider<'a> {
//     #[provide(dyn IUserService)]
    pub user_service: &'a dyn IUserService,

    // #[provide(dyn ITenantService)]
    pub tenant_service: &'a dyn ITenantService,
}

// impl<'a: 'static> IAppState for Provider<'a> {}



pub trait IUserService: Send + Sync + 'static {
    fn create(&self);
}

pub trait ITenantService: Send + Sync + 'static {
    fn get(&self);
}

