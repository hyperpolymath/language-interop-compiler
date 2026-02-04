-- SPDX-License-Identifier: PMPL-1.0-or-later
-- Example: User type that will be translated to ReScript and Rust

module Examples.UserType

-- Abstract type definition (language-agnostic)
public export
record User where
  constructor MkUser
  id : Int  -- Will map to: ReScript int, Rust i64
  name : String  -- Will map to: ReScript string, Rust String
  email : String  -- Will map to: ReScript string, Rust String
  active : Bool  -- Will map to: ReScript bool, Rust bool

-- Validation function (will be translated to both languages)
public export
validateUser : User -> Either String User
validateUser u =
  if name u == ""
    then Left "Name is required"
    else if email u == ""
      then Left "Email is required"
      else Right u

-- Usage example
public export
exampleUser : User
exampleUser = MkUser 1 "Alice" "alice@example.com" True
