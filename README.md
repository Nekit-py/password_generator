# Simple password generator

- The length of the password is accepted from the user (the maximum length is 255 characters of the int type), as well as the complexity of the password.
- The complexity of the password means the inclusion of certain arrays of the char type:
a simple password ("easy", "simple", "low", "short") includes numbers and letters in lowercase;
- The password of medium complexity ("medium", "default", "middle", "standart") includes characters from a simple password + uppercase letters;
- A complex password ("hard", "strong", "expert", "big") includes a medium password +. special characters;
- If the password length and/or its complexity are set incorrectly, a password of average complexity with a length of 8 characters is created.
- A password of each complexity is guaranteed to include at least 1 character from the arrays of characters listed above.