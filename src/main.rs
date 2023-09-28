// Domain layer

#[derive(Debug, Clone)]
pub struct User {
    id: i32,
    name: String,
}

impl User {
    pub fn new(id: i32, name: String) -> Self {
        User { id, name }
    }
}

pub trait UserRepository {
    fn find_user_by_id(&self, id: i32) -> Option<User>;
}

// Application layer

pub struct UserService {
    repository: Box<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Box<dyn UserRepository>) -> Self {
        UserService { repository }
    }

    pub fn get_user_by_id(&self, id: i32) -> Option<User> {
        self.repository.find_user_by_id(id)
    }
}

// Infrastructure layer

pub struct InMemoryUserRepository {
    users: Vec<User>,
}

// This is the "database"
impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository { users: vec![] }
    }

    // this is a query for inserting into the database
    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
}

impl UserRepository for InMemoryUserRepository {
    fn find_user_by_id(&self, id: i32) -> Option<User> {
        self.users.iter().find(|&user| user.id == id).cloned()
    }
}

fn main() {
    let mut repository = InMemoryUserRepository::new();
    let user = User::new(1, "John Doe".to_string());
    repository.add_user(user);

    let service = UserService::new(Box::new(repository));
    let user = service.get_user_by_id(1);

    match user {
        Some(user) => println!("User found: {:?}", user.name),
        None => println!("User not found"),
    }
}

