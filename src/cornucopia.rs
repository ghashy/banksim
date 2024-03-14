// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod bank_queries
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct InsertAccountParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,> { pub username : T1,pub card_number : T2,pub password_hash : T3,}#[derive( Debug)] pub struct CreateTransactionParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> { pub sender_card : T1,pub recipient_card : T2,pub amount : i64,}#[derive( Debug)] pub struct InsertTokenParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> { pub card_number : T1,pub token : T2,}pub struct I64Query < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> i64,
    mapper : fn(i64) -> T,
} impl < 'a, C, T : 'a, const N : usize > I64Query < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(i64) -> R) -> I64Query
    < 'a, C, R, N >
    {
        I64Query
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetEmissionAccount
{ pub id : i32,pub created_at : time::OffsetDateTime,pub username : String,pub card_number : String,pub password_hash : String,pub is_existing : bool,}pub struct GetEmissionAccountBorrowed < 'a >
{ pub id : i32,pub created_at : time::OffsetDateTime,pub username : &'a str,pub card_number : &'a str,pub password_hash : &'a str,pub is_existing : bool,} impl < 'a > From < GetEmissionAccountBorrowed <
'a >> for GetEmissionAccount
{
    fn
    from(GetEmissionAccountBorrowed { id,created_at,username,card_number,password_hash,is_existing,} : GetEmissionAccountBorrowed < 'a >)
    -> Self { Self { id,created_at,username: username.into(),card_number: card_number.into(),password_hash: password_hash.into(),is_existing,} }
}pub struct GetEmissionAccountQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetEmissionAccountBorrowed,
    mapper : fn(GetEmissionAccountBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetEmissionAccountQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetEmissionAccountBorrowed) -> R) -> GetEmissionAccountQuery
    < 'a, C, R, N >
    {
        GetEmissionAccountQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetStoreAccount
{ pub id : i32,pub created_at : time::OffsetDateTime,pub username : String,pub card_number : String,pub password_hash : String,pub is_existing : bool,}pub struct GetStoreAccountBorrowed < 'a >
{ pub id : i32,pub created_at : time::OffsetDateTime,pub username : &'a str,pub card_number : &'a str,pub password_hash : &'a str,pub is_existing : bool,} impl < 'a > From < GetStoreAccountBorrowed <
'a >> for GetStoreAccount
{
    fn
    from(GetStoreAccountBorrowed { id,created_at,username,card_number,password_hash,is_existing,} : GetStoreAccountBorrowed < 'a >)
    -> Self { Self { id,created_at,username: username.into(),card_number: card_number.into(),password_hash: password_hash.into(),is_existing,} }
}pub struct GetStoreAccountQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetStoreAccountBorrowed,
    mapper : fn(GetStoreAccountBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetStoreAccountQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetStoreAccountBorrowed) -> R) -> GetStoreAccountQuery
    < 'a, C, R, N >
    {
        GetStoreAccountQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub struct BoolQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> bool,
    mapper : fn(bool) -> T,
} impl < 'a, C, T : 'a, const N : usize > BoolQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(bool) -> R) -> BoolQuery
    < 'a, C, R, N >
    {
        BoolQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetAccount
{ pub username : String,pub card_number : String,pub is_existing : bool,pub password_hash : String,}pub struct GetAccountBorrowed < 'a >
{ pub username : &'a str,pub card_number : &'a str,pub is_existing : bool,pub password_hash : &'a str,} impl < 'a > From < GetAccountBorrowed <
'a >> for GetAccount
{
    fn
    from(GetAccountBorrowed { username,card_number,is_existing,password_hash,} : GetAccountBorrowed < 'a >)
    -> Self { Self { username: username.into(),card_number: card_number.into(),is_existing,password_hash: password_hash.into(),} }
}pub struct GetAccountQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetAccountBorrowed,
    mapper : fn(GetAccountBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetAccountQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetAccountBorrowed) -> R) -> GetAccountQuery
    < 'a, C, R, N >
    {
        GetAccountQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetAccountByToken
{ pub username : String,pub card_number : String,pub is_existing : bool,}pub struct GetAccountByTokenBorrowed < 'a >
{ pub username : &'a str,pub card_number : &'a str,pub is_existing : bool,} impl < 'a > From < GetAccountByTokenBorrowed <
'a >> for GetAccountByToken
{
    fn
    from(GetAccountByTokenBorrowed { username,card_number,is_existing,} : GetAccountByTokenBorrowed < 'a >)
    -> Self { Self { username: username.into(),card_number: card_number.into(),is_existing,} }
}pub struct GetAccountByTokenQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetAccountByTokenBorrowed,
    mapper : fn(GetAccountByTokenBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetAccountByTokenQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetAccountByTokenBorrowed) -> R) -> GetAccountByTokenQuery
    < 'a, C, R, N >
    {
        GetAccountByTokenQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub struct RustdecimalDecimalQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> rust_decimal::Decimal,
    mapper : fn(rust_decimal::Decimal) -> T,
} impl < 'a, C, T : 'a, const N : usize > RustdecimalDecimalQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(rust_decimal::Decimal) -> R) -> RustdecimalDecimalQuery
    < 'a, C, R, N >
    {
        RustdecimalDecimalQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct ListAccountTransactions
{ pub amount : i64,pub created_at : time::OffsetDateTime,pub sender_username : String,pub sender_card_number : String,pub sender_is_existing : bool,pub recipient_username : String,pub recipient_card_number : String,pub recipient_is_existing : bool,}pub struct ListAccountTransactionsBorrowed < 'a >
{ pub amount : i64,pub created_at : time::OffsetDateTime,pub sender_username : &'a str,pub sender_card_number : &'a str,pub sender_is_existing : bool,pub recipient_username : &'a str,pub recipient_card_number : &'a str,pub recipient_is_existing : bool,} impl < 'a > From < ListAccountTransactionsBorrowed <
'a >> for ListAccountTransactions
{
    fn
    from(ListAccountTransactionsBorrowed { amount,created_at,sender_username,sender_card_number,sender_is_existing,recipient_username,recipient_card_number,recipient_is_existing,} : ListAccountTransactionsBorrowed < 'a >)
    -> Self { Self { amount,created_at,sender_username: sender_username.into(),sender_card_number: sender_card_number.into(),sender_is_existing,recipient_username: recipient_username.into(),recipient_card_number: recipient_card_number.into(),recipient_is_existing,} }
}pub struct ListAccountTransactionsQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> ListAccountTransactionsBorrowed,
    mapper : fn(ListAccountTransactionsBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > ListAccountTransactionsQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(ListAccountTransactionsBorrowed) -> R) -> ListAccountTransactionsQuery
    < 'a, C, R, N >
    {
        ListAccountTransactionsQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct GetAccounts
{ pub username : String,pub card_number : String,pub is_existing : bool,pub balance : rust_decimal::Decimal,pub tokens : Vec<Option<String>>,}pub struct GetAccountsBorrowed < 'a >
{ pub username : &'a str,pub card_number : &'a str,pub is_existing : bool,pub balance : rust_decimal::Decimal,pub tokens : cornucopia_async::ArrayIterator<'a, Option<&'a str>>,} impl < 'a > From < GetAccountsBorrowed <
'a >> for GetAccounts
{
    fn
    from(GetAccountsBorrowed { username,card_number,is_existing,balance,tokens,} : GetAccountsBorrowed < 'a >)
    -> Self { Self { username: username.into(),card_number: card_number.into(),is_existing,balance,tokens: tokens.map(|v| v.map(|v| v.into())).collect(),} }
}pub struct GetAccountsQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetAccountsBorrowed,
    mapper : fn(GetAccountsBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetAccountsQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetAccountsBorrowed) -> R) -> GetAccountsQuery
    < 'a, C, R, N >
    {
        GetAccountsQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn accounts_count() -> AccountsCountStmt
{ AccountsCountStmt(cornucopia_async :: private :: Stmt :: new("SELECT COUNT(*)
FROM accounts")) } pub
struct AccountsCountStmt(cornucopia_async :: private :: Stmt) ; impl
AccountsCountStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> I64Query < 'a, C,
i64, 0 >
{
    I64Query
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }pub fn get_emission_account() -> GetEmissionAccountStmt
{ GetEmissionAccountStmt(cornucopia_async :: private :: Stmt :: new("SELECT *
FROM accounts
WHERE accounts.id = 1")) } pub
struct GetEmissionAccountStmt(cornucopia_async :: private :: Stmt) ; impl
GetEmissionAccountStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> GetEmissionAccountQuery < 'a, C,
GetEmissionAccount, 0 >
{
    GetEmissionAccountQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { GetEmissionAccountBorrowed { id : row.get(0),created_at : row.get(1),username : row.get(2),card_number : row.get(3),password_hash : row.get(4),is_existing : row.get(5),} }, mapper : | it | { <GetEmissionAccount>::from(it) },
    }
} }pub fn get_store_account() -> GetStoreAccountStmt
{ GetStoreAccountStmt(cornucopia_async :: private :: Stmt :: new("SELECT *
FROM accounts
WHERE accounts.id = 2")) } pub
struct GetStoreAccountStmt(cornucopia_async :: private :: Stmt) ; impl
GetStoreAccountStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> GetStoreAccountQuery < 'a, C,
GetStoreAccount, 0 >
{
    GetStoreAccountQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { GetStoreAccountBorrowed { id : row.get(0),created_at : row.get(1),username : row.get(2),card_number : row.get(3),password_hash : row.get(4),is_existing : row.get(5),} }, mapper : | it | { <GetStoreAccount>::from(it) },
    }
} }pub fn insert_account() -> InsertAccountStmt
{ InsertAccountStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO accounts(username, card_number, password_hash)
VALUES ($1, $2, $3)")) } pub
struct InsertAccountStmt(cornucopia_async :: private :: Stmt) ; impl
InsertAccountStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
username : & 'a T1,card_number : & 'a T2,password_hash : & 'a T3,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [username,card_number,password_hash,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, InsertAccountParams < T1,T2,T3,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for InsertAccountStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertAccountParams < T1,T2,T3,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.username,& params.card_number,& params.password_hash,) ) }
}pub fn is_account_exists() -> IsAccountExistsStmt
{ IsAccountExistsStmt(cornucopia_async :: private :: Stmt :: new("SELECT is_existing FROM accounts
WHERE card_number = $1")) } pub
struct IsAccountExistsStmt(cornucopia_async :: private :: Stmt) ; impl
IsAccountExistsStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
card_number : & 'a T1,) -> BoolQuery < 'a, C,
bool, 1 >
{
    BoolQuery
    {
        client, params : [card_number,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }pub fn mark_account_as_deleted() -> MarkAccountAsDeletedStmt
{ MarkAccountAsDeletedStmt(cornucopia_async :: private :: Stmt :: new("UPDATE accounts
SET is_existing = FALSE
WHERE card_number = $1")) } pub
struct MarkAccountAsDeletedStmt(cornucopia_async :: private :: Stmt) ; impl
MarkAccountAsDeletedStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
card_number : & 'a T1,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [card_number,]) .await
} }pub fn get_account() -> GetAccountStmt
{ GetAccountStmt(cornucopia_async :: private :: Stmt :: new("SELECT 
    accounts.username,
    accounts.card_number,
    accounts.is_existing,
    accounts.password_hash
FROM accounts
WHERE card_number = $1")) } pub
struct GetAccountStmt(cornucopia_async :: private :: Stmt) ; impl
GetAccountStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
card_number : & 'a T1,) -> GetAccountQuery < 'a, C,
GetAccount, 1 >
{
    GetAccountQuery
    {
        client, params : [card_number,], stmt : & mut self.0, extractor :
        | row | { GetAccountBorrowed { username : row.get(0),card_number : row.get(1),is_existing : row.get(2),password_hash : row.get(3),} }, mapper : | it | { <GetAccount>::from(it) },
    }
} }pub fn get_account_by_token() -> GetAccountByTokenStmt
{ GetAccountByTokenStmt(cornucopia_async :: private :: Stmt :: new("SELECT 
    a.username,
    a.card_number,
    a.is_existing
FROM tokens
LEFT JOIN accounts a ON tokens.account = a.id
WHERE  tokens.token = $1")) } pub
struct GetAccountByTokenStmt(cornucopia_async :: private :: Stmt) ; impl
GetAccountByTokenStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
token : & 'a T1,) -> GetAccountByTokenQuery < 'a, C,
GetAccountByToken, 1 >
{
    GetAccountByTokenQuery
    {
        client, params : [token,], stmt : & mut self.0, extractor :
        | row | { GetAccountByTokenBorrowed { username : row.get(0),card_number : row.get(1),is_existing : row.get(2),} }, mapper : | it | { <GetAccountByToken>::from(it) },
    }
} }pub fn get_account_balance() -> GetAccountBalanceStmt
{ GetAccountBalanceStmt(cornucopia_async :: private :: Stmt :: new("SELECT COALESCE(SUM(recv.amount), 0) - COALESCE(SUM(spnd.amount), 0) AS balance
FROM accounts a
LEFT JOIN transactions recv ON a.id = recv.recipient
LEFT JOIN transactions spnd ON a.id = spnd.sender
WHERE a.card_number = $1")) } pub
struct GetAccountBalanceStmt(cornucopia_async :: private :: Stmt) ; impl
GetAccountBalanceStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
card_number : & 'a T1,) -> RustdecimalDecimalQuery < 'a, C,
rust_decimal::Decimal, 1 >
{
    RustdecimalDecimalQuery
    {
        client, params : [card_number,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }pub fn list_account_transactions() -> ListAccountTransactionsStmt
{ ListAccountTransactionsStmt(cornucopia_async :: private :: Stmt :: new("SELECT 
    t.amount,
    t.created_at,
    sender_account.username AS sender_username,
    sender_account.card_number AS sender_card_number,
    sender_account.is_existing AS sender_is_existing,
    recipient_account.username AS recipient_username,
    recipient_account.card_number AS recipient_card_number,
    recipient_account.is_existing AS recipient_is_existing
FROM transactions t
LEFT JOIN accounts sender_account ON t.sender = sender_account.id
LEFT JOIN accounts recipient_account ON t.recipient = recipient_account.id
WHERE sender_account.card_number = $1 OR recipient_account.card_number = $1")) } pub
struct ListAccountTransactionsStmt(cornucopia_async :: private :: Stmt) ; impl
ListAccountTransactionsStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
card_number : & 'a T1,) -> ListAccountTransactionsQuery < 'a, C,
ListAccountTransactions, 1 >
{
    ListAccountTransactionsQuery
    {
        client, params : [card_number,], stmt : & mut self.0, extractor :
        | row | { ListAccountTransactionsBorrowed { amount : row.get(0),created_at : row.get(1),sender_username : row.get(2),sender_card_number : row.get(3),sender_is_existing : row.get(4),recipient_username : row.get(5),recipient_card_number : row.get(6),recipient_is_existing : row.get(7),} }, mapper : | it | { <ListAccountTransactions>::from(it) },
    }
} }pub fn get_accounts() -> GetAccountsStmt
{ GetAccountsStmt(cornucopia_async :: private :: Stmt :: new("WITH received_amount AS (
    SELECT recipient, COALESCE(SUM(amount), 0) AS received_total
    FROM transactions
    GROUP BY recipient
),
spent_amount AS (
    SELECT sender, COALESCE(SUM(amount), 0) AS spent_total
    FROM transactions
    GROUP BY sender
)
SELECT
    a.username,
    a.card_number,
    a.is_existing,
    COALESCE(ra.received_total, 0) - COALESCE(sa.spent_total, 0) AS balance,
    ARRAY_AGG(t.token) AS tokens
FROM accounts a
LEFT JOIN received_amount ra ON a.id = ra.recipient
LEFT JOIN spent_amount sa ON a.id = sa.sender
LEFT JOIN tokens t ON a.id = t.account
GROUP BY a.username, a.card_number, a.is_existing, ra.received_total, sa.spent_total")) } pub
struct GetAccountsStmt(cornucopia_async :: private :: Stmt) ; impl
GetAccountsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> GetAccountsQuery < 'a, C,
GetAccounts, 0 >
{
    GetAccountsQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { GetAccountsBorrowed { username : row.get(0),card_number : row.get(1),is_existing : row.get(2),balance : row.get(3),tokens : row.get(4),} }, mapper : | it | { <GetAccounts>::from(it) },
    }
} }pub fn create_transaction() -> CreateTransactionStmt
{ CreateTransactionStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO transactions(sender, recipient, amount)
VALUES (
    (
        SELECT id FROM accounts WHERE card_number = $1 
    ),
    (
        SELECT id FROM accounts WHERE card_number = $2
    ),
     $3
)")) } pub
struct CreateTransactionStmt(cornucopia_async :: private :: Stmt) ; impl
CreateTransactionStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
sender_card : & 'a T1,recipient_card : & 'a T2,amount : & 'a i64,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [sender_card,recipient_card,amount,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, CreateTransactionParams < T1,T2,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for CreateTransactionStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    CreateTransactionParams < T1,T2,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.sender_card,& params.recipient_card,& params.amount,) ) }
}pub fn insert_token() -> InsertTokenStmt
{ InsertTokenStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO tokens(account, token)
VALUES (
    (
        SELECT id FROM accounts WHERE card_number = $1
    ),
    $2
)")) } pub
struct InsertTokenStmt(cornucopia_async :: private :: Stmt) ; impl
InsertTokenStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
card_number : & 'a T1,token : & 'a T2,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [card_number,token,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, InsertTokenParams < T1,T2,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for InsertTokenStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertTokenParams < T1,T2,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.card_number,& params.token,) ) }
}}}
