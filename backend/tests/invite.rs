use diary_dot_computer_backend::services::invite;
use uuid::Uuid;

#[test]
fn generates_an_invite() {
  let code = invite::generate_invite(None);
  assert!(code.is_ok());
}

#[test]
fn generates_unique_invites() {
  let code1 = invite::generate_invite(None).unwrap();
  let code2 = invite::generate_invite(None).unwrap();
  assert_ne!(code1.code, code2.code);
}

#[test]
fn generates_an_invite_with_code() {
  let value = Uuid::new_v4().to_string();
  let code = invite::generate_invite(Some(&value)).unwrap();
  assert_eq!(code.code, value);
}

#[test]
fn generates_unique_invites_with_code() {
  let value = Uuid::new_v4().to_string();
  let code1 = invite::generate_invite(Some(&value)).unwrap();
  let code2 = invite::generate_invite(Some(&value)).unwrap();
  assert_ne!(code1.code, code2.code);
}
