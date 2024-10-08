pub use amount_derivation_errors::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
  clippy::enum_variant_names,
  clippy::too_many_arguments,
  clippy::upper_case_acronyms,
  clippy::type_complexity,
  dead_code,
  non_camel_case_types,
)]
pub mod amount_derivation_errors {
  #[rustfmt::skip]
  const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"InexactFraction\",\"outputs\":[]}]";
  ///The parsed JSON ABI of the contract.
  pub static AMOUNTDERIVATIONERRORS_ABI: ::ethers::contract::Lazy<
    ::ethers::core::abi::Abi,
  > = ::ethers::contract::Lazy::new(|| {
    ::ethers::core::utils::__serde_json::from_str(__ABI)
      .expect("ABI is always valid")
  });
  pub struct AmountDerivationErrors<M>(::ethers::contract::Contract<M>);
  impl<M> ::core::clone::Clone for AmountDerivationErrors<M> {
    fn clone(&self) -> Self {
      Self(::core::clone::Clone::clone(&self.0))
    }
  }
  impl<M> ::core::ops::Deref for AmountDerivationErrors<M> {
    type Target = ::ethers::contract::Contract<M>;
    fn deref(&self) -> &Self::Target {
      &self.0
    }
  }
  impl<M> ::core::ops::DerefMut for AmountDerivationErrors<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
    }
  }
  impl<M> ::core::fmt::Debug for AmountDerivationErrors<M> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
      f.debug_tuple(stringify!(AmountDerivationErrors))
        .field(&self.address())
        .finish()
    }
  }
  impl<M: ::ethers::providers::Middleware> AmountDerivationErrors<M> {
    /// Creates a new contract instance with the specified `ethers` client at
    /// `address`. The contract derefs to a `ethers::Contract` object.
    pub fn new<T: Into<::ethers::core::types::Address>>(
      address: T,
      client: ::std::sync::Arc<M>,
    ) -> Self {
      Self(
        ::ethers::contract::Contract::new(
          address.into(),
          AMOUNTDERIVATIONERRORS_ABI.clone(),
          client,
        ),
      )
    }
  }
  impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
  for AmountDerivationErrors<M> {
    fn from(contract: ::ethers::contract::Contract<M>) -> Self {
      Self::new(contract.address(), contract.client())
    }
  }
  ///Custom Error type `InexactFraction` with signature `InexactFraction()` and selector `0xc63cf089`
  #[derive(
    Clone,
    ::ethers::contract::EthError,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
  )]
  #[etherror(name = "InexactFraction", abi = "InexactFraction()")]
  pub struct InexactFraction;
}
