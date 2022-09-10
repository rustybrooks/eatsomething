use std::convert::Infallible;

use crate::pool::OurConn;

/*
 async signup({ username, email, password, password2 }: { username: string; email: string; password: string; password2: string }) {
   const errors: { [id: string]: string } = {};
   if (username.length < 4) {
     errors.username = 'Username must be at least 4 characters';
   }

   const re = /^[a-z,A-Z,0-9,\-,_]+$/;
   if (!username.match(re)) {
     errors.username = 'Username must be composed of only letters, numbers, _ and -';
   }

   if (password !== password2) {
     errors.password2 = 'Passwords do not match';
   }

   if (password.length < 8) {
     errors.password = 'Password must be at least 8 characters';
   }

   if (!email) {
     errors.email = 'Email required';
   }

   if (Object.keys(errors).length) {
     throw new HttpBadRequest(errors);
   }

   try {
     await queries.addUser({ username, password, email });
     return queries.generateToken(username);
   } catch (e) {
     throw new HttpBadRequest({ username: 'Failed to create user' });
   }
 }
*/

pub async fn user_signup(_pool: OurConn) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("hi"))
}
