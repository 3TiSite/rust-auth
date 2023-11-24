t3::api!();

urlmod!();
mod _mod;
mod db;
mod r#macro;
use crate::db::sign_in::{sign_in, SignIn};
#[allow(non_snake_case)]
pub mod K;
mod i18n;
mod lua;
use anyhow::Result;
use client::Client;
use intbin::u64_bin;
use r::{fred::interfaces::HashesInterface, KV};
use t3::{ok, ConnectInfo, HeaderMap};

use crate::db::bantld;

pub const SIGN_UP: u8 = 0; // 注册
pub const SIGN_IN: u8 = 1; // 登录

pub async fn sign_in_lang_name(client: &Client, id: u64, ip: Box<[u8]>) -> Result<(u8, String)> {
  let client_uid = client.uid().await?;
  let id_bin = &u64_bin(id)[..];
  let p = KV.pipeline();
  p.hget(K::LANG, id_bin).await?;
  let (lang, name) = if if let Some(uid) = client_uid {
    id != uid
  } else {
    true
  } {
    p.hget(K::NAME, id_bin).await?;
    client.sign_in(&p, id_bin, ip).await?;
    let li: (_, _, ()) = p.all().await?;
    (li.0, li.1)
  } else {
    p.hget(K::NAME, id_bin).await?;
    p.all().await?
  };
  Ok((db::lang::get(lang), name))
}

pub async fn post(
  ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
  client: Client,
  header: HeaderMap,
  json: String,
) -> t3::msg!() {
  captcha::verify(&header).await?;
  let (action, account, password): (u8, String, String) = sonic_rs::from_str(&json)?;

  let account = xmail::norm(account);
  let host = &t3::origin_tld(&header)?;

  match sign_in(host, &account, &password).await? {
    SignIn::Ok(id) => {
      let ip = t3::ip_bin(&header, &addr);
      let (lang, name) = sign_in_lang_name(&client, id, ip).await?;
      ok!(api::User {
        id,
        name,
        lang: lang as _
      })
    }
    SignIn::PasswdError => {
      if action == SIGN_UP {
        throw!(header, account, ACCOUNT_EXIST)
      }
      throw!(header, password, PASSWORD_ERROR)
    }
    SignIn::AccountNotExist => {
      if action == SIGN_UP {
        if let Some(p) = account.find('@') {
          if account.len() > p {
            let tld = xtld::host_tld(&account[p + 1..]);
            if bantld::is(tld).await? {
              throw!(header, account, BAN_MAIL)
            }
          }
        }
        return ok!(db::mail::host_send(i18n::SIGN_UP, &header, host, account, password).await?);
      }
      throw!(header, account, ACCOUNT_NOT_EXIST)
    }
  }
}