use std::sync::Arc;

use bcrypt::{hash, verify};
use lib_core::model::base::BasicDbOps;

use crate::auth::dto::{LoginRequest, RegisterRequest};
use crate::auth::error::{AuthError, Result};
use crate::auth::jwt::create_jwt;
use crate::users::model::{NewUser, User};
use lib_core::model::ModelManager;

#[derive(Clone)]
pub struct AuthService {
	pub mm: Arc<ModelManager>,
	pub jwt_secret: String,
}

impl AuthService {
	pub fn register(&self, input: RegisterRequest) -> Result<(String, String)> {
		let mut db = self.mm.db();

		// Check if username already exists
		let existing = User::find_by_username(&mut db, &input.username)
			.map_err(|e| AuthError::DbError(e.to_string()))?;

		if existing.is_some() {
			return Err(AuthError::UserExists);
		}

		// Hash password
		let password_hash = hash(&input.password, 10)
			.map_err(|e| AuthError::HashError(e.to_string()))?;

		let new_user = NewUser {
			username: &input.username,
			password_hash: &password_hash,
		};

		let user = User::create(&mut db, new_user)
			.map_err(|e| AuthError::DbError(e.to_string()))?;

		// Generate token
		let token = create_jwt(user.id, self.jwt_secret.as_bytes())
			.map_err(|e| AuthError::JwtError(e.to_string()))?;

		Ok((token, user.id.to_string()))
	}

	pub fn login(&self, input: LoginRequest) -> Result<(String, String)> {
		// Find user by username
		let mut db = self.mm.db();

		let user = User::find_by_username(&mut db, &input.username)
			.map_err(|e| AuthError::DbError(e.to_string()))?
			.ok_or(AuthError::UserNotFound)?;

		// Verify password
		let verified = verify(&input.password, &user.password_hash)
			.map_err(|e| AuthError::VerifyError(e.to_string()))?;

		if !verified {
			return Err(AuthError::InvalidToken);
		}

		// Generate token
		let token = create_jwt(user.id, self.jwt_secret.as_bytes())
			.map_err(|e| AuthError::JwtError(e.to_string()))?;

		Ok((token, user.id.to_string()))
	}
}
