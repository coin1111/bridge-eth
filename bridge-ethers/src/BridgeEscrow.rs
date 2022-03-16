pub use bridgeescrow_mod::*;
#[allow(clippy::too_many_arguments)]
mod bridgeescrow_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "BridgeEscrow was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BRIDGEESCROW_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"olTokenAddr\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"executorAddr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"_to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"_data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"call\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"closeTransferAccountSender\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"receiver_address\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"amount\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"createTransferAccount\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"sender_this\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"sender_other\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"receiver_this\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"receiver_other\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"amount\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"createTransferAccountAux\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"receiver_address\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"amount\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"createTransferAccountThis\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"executor\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transferId\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"getLockedAccountInfo\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"sender_this\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes16\",\n            \"name\": \"sender_other\",\n            \"type\": \"bytes16\"\n          },\n          {\n            \"internalType\": \"address payable\",\n            \"name\": \"receiver_this\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes16\",\n            \"name\": \"receiver_other\",\n            \"type\": \"bytes16\"\n          },\n          {\n            \"internalType\": \"uint64\",\n            \"name\": \"balance\",\n            \"type\": \"uint64\"\n          },\n          {\n            \"internalType\": \"bytes16\",\n            \"name\": \"transfer_id\",\n            \"type\": \"bytes16\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locked_idx\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"is_closed\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct BridgeEscrow.AccountInfo\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getLockedLength\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"start\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"n\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getNextTransferId\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transferId\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"getUnlockedAccountInfo\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"sender_this\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes16\",\n            \"name\": \"sender_other\",\n            \"type\": \"bytes16\"\n          },\n          {\n            \"internalType\": \"address payable\",\n            \"name\": \"receiver_this\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes16\",\n            \"name\": \"receiver_other\",\n            \"type\": \"bytes16\"\n          },\n          {\n            \"internalType\": \"uint64\",\n            \"name\": \"balance\",\n            \"type\": \"uint64\"\n          },\n          {\n            \"internalType\": \"bytes16\",\n            \"name\": \"transfer_id\",\n            \"type\": \"bytes16\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locked_idx\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"is_closed\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct BridgeEscrow.AccountInfo\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"sender_address\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"receiver_address\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"balance\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"withdrawFromEscrow\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"sender_this\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"sender_other\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"receiver_this\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"balance\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"withdrawFromEscrowAux\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"sender_address\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"receiver_address\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"balance\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"withdrawFromEscrowThis\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct BridgeEscrow<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for BridgeEscrow<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for BridgeEscrow<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BridgeEscrow))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> BridgeEscrow<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), BRIDGEESCROW_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `call` (0x6dbf2fa0) function"]
        pub fn call(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([109, 191, 47, 160], (to, value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `closeTransferAccountSender` (0x73df63f4) function"]
        pub fn close_transfer_account_sender(
            &self,
            transfer_id: [u8; 16],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 223, 99, 244], transfer_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createTransferAccount` (0x4f411670) function"]
        pub fn create_transfer_account(
            &self,
            receiver_address: [u8; 16],
            amount: u64,
            transfer_id: [u8; 16],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 65, 22, 112], (receiver_address, amount, transfer_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createTransferAccountAux` (0xfac60217) function"]
        pub fn create_transfer_account_aux(
            &self,
            sender_this: ethers::core::types::Address,
            sender_other: [u8; 16],
            receiver_this: ethers::core::types::Address,
            receiver_other: [u8; 16],
            amount: u64,
            transfer_id: [u8; 16],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [250, 198, 2, 23],
                    (
                        sender_this,
                        sender_other,
                        receiver_this,
                        receiver_other,
                        amount,
                        transfer_id,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createTransferAccountThis` (0x3e60e5dd) function"]
        pub fn create_transfer_account_this(
            &self,
            receiver_address: ethers::core::types::Address,
            amount: u64,
            transfer_id: [u8; 16],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 96, 229, 221], (receiver_address, amount, transfer_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executor` (0xc34c08e5) function"]
        pub fn executor(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([195, 76, 8, 229], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLockedAccountInfo` (0x6ef902d8) function"]
        pub fn get_locked_account_info(
            &self,
            transfer_id: [u8; 16],
        ) -> ethers::contract::builders::ContractCall<M, AccountInfo> {
            self.0
                .method_hash([110, 249, 2, 216], transfer_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLockedLength` (0x1593d0f6) function"]
        pub fn get_locked_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([21, 147, 208, 246], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNextTransferId` (0x27ac1453) function"]
        pub fn get_next_transfer_id(
            &self,
            start: ethers::core::types::U256,
            n: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ([u8; 16], ethers::core::types::U256)>
        {
            self.0
                .method_hash([39, 172, 20, 83], (start, n))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUnlockedAccountInfo` (0xc829f8e7) function"]
        pub fn get_unlocked_account_info(
            &self,
            transfer_id: [u8; 16],
        ) -> ethers::contract::builders::ContractCall<M, AccountInfo> {
            self.0
                .method_hash([200, 41, 248, 231], transfer_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFromEscrow` (0xdb4d7e05) function"]
        pub fn withdraw_from_escrow(
            &self,
            sender_address: [u8; 16],
            receiver_address: ethers::core::types::Address,
            balance: u64,
            transfer_id: [u8; 16],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [219, 77, 126, 5],
                    (sender_address, receiver_address, balance, transfer_id),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFromEscrowAux` (0x703b7597) function"]
        pub fn withdraw_from_escrow_aux(
            &self,
            sender_this: ethers::core::types::Address,
            sender_other: [u8; 16],
            receiver_this: ethers::core::types::Address,
            balance: u64,
            transfer_id: [u8; 16],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [112, 59, 117, 151],
                    (
                        sender_this,
                        sender_other,
                        receiver_this,
                        balance,
                        transfer_id,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFromEscrowThis` (0x5c904ac8) function"]
        pub fn withdraw_from_escrow_this(
            &self,
            sender_address: ethers::core::types::Address,
            receiver_address: ethers::core::types::Address,
            balance: u64,
            transfer_id: [u8; 16],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [92, 144, 74, 200],
                    (sender_address, receiver_address, balance, transfer_id),
                )
                .expect("method not found (this should never happen)")
        }
    }
    #[doc = "Container type for all input parameters for the `call`function with signature `call(address,uint256,bytes)` and selector `[109, 191, 47, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "call", abi = "call(address,uint256,bytes)")]
    pub struct CallCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `closeTransferAccountSender`function with signature `closeTransferAccountSender(bytes16)` and selector `[115, 223, 99, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "closeTransferAccountSender",
        abi = "closeTransferAccountSender(bytes16)"
    )]
    pub struct CloseTransferAccountSenderCall {
        pub transfer_id: [u8; 16],
    }
    #[doc = "Container type for all input parameters for the `createTransferAccount`function with signature `createTransferAccount(bytes16,uint64,bytes16)` and selector `[79, 65, 22, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "createTransferAccount",
        abi = "createTransferAccount(bytes16,uint64,bytes16)"
    )]
    pub struct CreateTransferAccountCall {
        pub receiver_address: [u8; 16],
        pub amount: u64,
        pub transfer_id: [u8; 16],
    }
    #[doc = "Container type for all input parameters for the `createTransferAccountAux`function with signature `createTransferAccountAux(address,bytes16,address,bytes16,uint64,bytes16)` and selector `[250, 198, 2, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "createTransferAccountAux",
        abi = "createTransferAccountAux(address,bytes16,address,bytes16,uint64,bytes16)"
    )]
    pub struct CreateTransferAccountAuxCall {
        pub sender_this: ethers::core::types::Address,
        pub sender_other: [u8; 16],
        pub receiver_this: ethers::core::types::Address,
        pub receiver_other: [u8; 16],
        pub amount: u64,
        pub transfer_id: [u8; 16],
    }
    #[doc = "Container type for all input parameters for the `createTransferAccountThis`function with signature `createTransferAccountThis(address,uint64,bytes16)` and selector `[62, 96, 229, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "createTransferAccountThis",
        abi = "createTransferAccountThis(address,uint64,bytes16)"
    )]
    pub struct CreateTransferAccountThisCall {
        pub receiver_address: ethers::core::types::Address,
        pub amount: u64,
        pub transfer_id: [u8; 16],
    }
    #[doc = "Container type for all input parameters for the `executor`function with signature `executor()` and selector `[195, 76, 8, 229]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "executor", abi = "executor()")]
    pub struct ExecutorCall;
    #[doc = "Container type for all input parameters for the `getLockedAccountInfo`function with signature `getLockedAccountInfo(bytes16)` and selector `[110, 249, 2, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getLockedAccountInfo", abi = "getLockedAccountInfo(bytes16)")]
    pub struct GetLockedAccountInfoCall {
        pub transfer_id: [u8; 16],
    }
    #[doc = "Container type for all input parameters for the `getLockedLength`function with signature `getLockedLength()` and selector `[21, 147, 208, 246]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getLockedLength", abi = "getLockedLength()")]
    pub struct GetLockedLengthCall;
    #[doc = "Container type for all input parameters for the `getNextTransferId`function with signature `getNextTransferId(uint256,uint256)` and selector `[39, 172, 20, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNextTransferId", abi = "getNextTransferId(uint256,uint256)")]
    pub struct GetNextTransferIdCall {
        pub start: ethers::core::types::U256,
        pub n: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getUnlockedAccountInfo`function with signature `getUnlockedAccountInfo(bytes16)` and selector `[200, 41, 248, 231]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "getUnlockedAccountInfo",
        abi = "getUnlockedAccountInfo(bytes16)"
    )]
    pub struct GetUnlockedAccountInfoCall {
        pub transfer_id: [u8; 16],
    }
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `withdrawFromEscrow`function with signature `withdrawFromEscrow(bytes16,address,uint64,bytes16)` and selector `[219, 77, 126, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "withdrawFromEscrow",
        abi = "withdrawFromEscrow(bytes16,address,uint64,bytes16)"
    )]
    pub struct WithdrawFromEscrowCall {
        pub sender_address: [u8; 16],
        pub receiver_address: ethers::core::types::Address,
        pub balance: u64,
        pub transfer_id: [u8; 16],
    }
    #[doc = "Container type for all input parameters for the `withdrawFromEscrowAux`function with signature `withdrawFromEscrowAux(address,bytes16,address,uint64,bytes16)` and selector `[112, 59, 117, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "withdrawFromEscrowAux",
        abi = "withdrawFromEscrowAux(address,bytes16,address,uint64,bytes16)"
    )]
    pub struct WithdrawFromEscrowAuxCall {
        pub sender_this: ethers::core::types::Address,
        pub sender_other: [u8; 16],
        pub receiver_this: ethers::core::types::Address,
        pub balance: u64,
        pub transfer_id: [u8; 16],
    }
    #[doc = "Container type for all input parameters for the `withdrawFromEscrowThis`function with signature `withdrawFromEscrowThis(address,address,uint64,bytes16)` and selector `[92, 144, 74, 200]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "withdrawFromEscrowThis",
        abi = "withdrawFromEscrowThis(address,address,uint64,bytes16)"
    )]
    pub struct WithdrawFromEscrowThisCall {
        pub sender_address: ethers::core::types::Address,
        pub receiver_address: ethers::core::types::Address,
        pub balance: u64,
        pub transfer_id: [u8; 16],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BridgeEscrowCalls {
        Call(CallCall),
        CloseTransferAccountSender(CloseTransferAccountSenderCall),
        CreateTransferAccount(CreateTransferAccountCall),
        CreateTransferAccountAux(CreateTransferAccountAuxCall),
        CreateTransferAccountThis(CreateTransferAccountThisCall),
        Executor(ExecutorCall),
        GetLockedAccountInfo(GetLockedAccountInfoCall),
        GetLockedLength(GetLockedLengthCall),
        GetNextTransferId(GetNextTransferIdCall),
        GetUnlockedAccountInfo(GetUnlockedAccountInfoCall),
        Owner(OwnerCall),
        WithdrawFromEscrow(WithdrawFromEscrowCall),
        WithdrawFromEscrowAux(WithdrawFromEscrowAuxCall),
        WithdrawFromEscrowThis(WithdrawFromEscrowThisCall),
    }
    impl ethers::core::abi::AbiDecode for BridgeEscrowCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <CallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(BridgeEscrowCalls::Call(decoded));
            }
            if let Ok(decoded) =
                <CloseTransferAccountSenderCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeEscrowCalls::CloseTransferAccountSender(decoded));
            }
            if let Ok(decoded) =
                <CreateTransferAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeEscrowCalls::CreateTransferAccount(decoded));
            }
            if let Ok(decoded) =
                <CreateTransferAccountAuxCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeEscrowCalls::CreateTransferAccountAux(decoded));
            }
            if let Ok(decoded) =
                <CreateTransferAccountThisCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeEscrowCalls::CreateTransferAccountThis(decoded));
            }
            if let Ok(decoded) =
                <ExecutorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeEscrowCalls::Executor(decoded));
            }
            if let Ok(decoded) =
                <GetLockedAccountInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeEscrowCalls::GetLockedAccountInfo(decoded));
            }
            if let Ok(decoded) =
                <GetLockedLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeEscrowCalls::GetLockedLength(decoded));
            }
            if let Ok(decoded) =
                <GetNextTransferIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeEscrowCalls::GetNextTransferId(decoded));
            }
            if let Ok(decoded) =
                <GetUnlockedAccountInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeEscrowCalls::GetUnlockedAccountInfo(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeEscrowCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFromEscrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeEscrowCalls::WithdrawFromEscrow(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFromEscrowAuxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeEscrowCalls::WithdrawFromEscrowAux(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFromEscrowThisCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeEscrowCalls::WithdrawFromEscrowThis(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BridgeEscrowCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BridgeEscrowCalls::Call(element) => element.encode(),
                BridgeEscrowCalls::CloseTransferAccountSender(element) => element.encode(),
                BridgeEscrowCalls::CreateTransferAccount(element) => element.encode(),
                BridgeEscrowCalls::CreateTransferAccountAux(element) => element.encode(),
                BridgeEscrowCalls::CreateTransferAccountThis(element) => element.encode(),
                BridgeEscrowCalls::Executor(element) => element.encode(),
                BridgeEscrowCalls::GetLockedAccountInfo(element) => element.encode(),
                BridgeEscrowCalls::GetLockedLength(element) => element.encode(),
                BridgeEscrowCalls::GetNextTransferId(element) => element.encode(),
                BridgeEscrowCalls::GetUnlockedAccountInfo(element) => element.encode(),
                BridgeEscrowCalls::Owner(element) => element.encode(),
                BridgeEscrowCalls::WithdrawFromEscrow(element) => element.encode(),
                BridgeEscrowCalls::WithdrawFromEscrowAux(element) => element.encode(),
                BridgeEscrowCalls::WithdrawFromEscrowThis(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BridgeEscrowCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BridgeEscrowCalls::Call(element) => element.fmt(f),
                BridgeEscrowCalls::CloseTransferAccountSender(element) => element.fmt(f),
                BridgeEscrowCalls::CreateTransferAccount(element) => element.fmt(f),
                BridgeEscrowCalls::CreateTransferAccountAux(element) => element.fmt(f),
                BridgeEscrowCalls::CreateTransferAccountThis(element) => element.fmt(f),
                BridgeEscrowCalls::Executor(element) => element.fmt(f),
                BridgeEscrowCalls::GetLockedAccountInfo(element) => element.fmt(f),
                BridgeEscrowCalls::GetLockedLength(element) => element.fmt(f),
                BridgeEscrowCalls::GetNextTransferId(element) => element.fmt(f),
                BridgeEscrowCalls::GetUnlockedAccountInfo(element) => element.fmt(f),
                BridgeEscrowCalls::Owner(element) => element.fmt(f),
                BridgeEscrowCalls::WithdrawFromEscrow(element) => element.fmt(f),
                BridgeEscrowCalls::WithdrawFromEscrowAux(element) => element.fmt(f),
                BridgeEscrowCalls::WithdrawFromEscrowThis(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CallCall> for BridgeEscrowCalls {
        fn from(var: CallCall) -> Self {
            BridgeEscrowCalls::Call(var)
        }
    }
    impl ::std::convert::From<CloseTransferAccountSenderCall> for BridgeEscrowCalls {
        fn from(var: CloseTransferAccountSenderCall) -> Self {
            BridgeEscrowCalls::CloseTransferAccountSender(var)
        }
    }
    impl ::std::convert::From<CreateTransferAccountCall> for BridgeEscrowCalls {
        fn from(var: CreateTransferAccountCall) -> Self {
            BridgeEscrowCalls::CreateTransferAccount(var)
        }
    }
    impl ::std::convert::From<CreateTransferAccountAuxCall> for BridgeEscrowCalls {
        fn from(var: CreateTransferAccountAuxCall) -> Self {
            BridgeEscrowCalls::CreateTransferAccountAux(var)
        }
    }
    impl ::std::convert::From<CreateTransferAccountThisCall> for BridgeEscrowCalls {
        fn from(var: CreateTransferAccountThisCall) -> Self {
            BridgeEscrowCalls::CreateTransferAccountThis(var)
        }
    }
    impl ::std::convert::From<ExecutorCall> for BridgeEscrowCalls {
        fn from(var: ExecutorCall) -> Self {
            BridgeEscrowCalls::Executor(var)
        }
    }
    impl ::std::convert::From<GetLockedAccountInfoCall> for BridgeEscrowCalls {
        fn from(var: GetLockedAccountInfoCall) -> Self {
            BridgeEscrowCalls::GetLockedAccountInfo(var)
        }
    }
    impl ::std::convert::From<GetLockedLengthCall> for BridgeEscrowCalls {
        fn from(var: GetLockedLengthCall) -> Self {
            BridgeEscrowCalls::GetLockedLength(var)
        }
    }
    impl ::std::convert::From<GetNextTransferIdCall> for BridgeEscrowCalls {
        fn from(var: GetNextTransferIdCall) -> Self {
            BridgeEscrowCalls::GetNextTransferId(var)
        }
    }
    impl ::std::convert::From<GetUnlockedAccountInfoCall> for BridgeEscrowCalls {
        fn from(var: GetUnlockedAccountInfoCall) -> Self {
            BridgeEscrowCalls::GetUnlockedAccountInfo(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for BridgeEscrowCalls {
        fn from(var: OwnerCall) -> Self {
            BridgeEscrowCalls::Owner(var)
        }
    }
    impl ::std::convert::From<WithdrawFromEscrowCall> for BridgeEscrowCalls {
        fn from(var: WithdrawFromEscrowCall) -> Self {
            BridgeEscrowCalls::WithdrawFromEscrow(var)
        }
    }
    impl ::std::convert::From<WithdrawFromEscrowAuxCall> for BridgeEscrowCalls {
        fn from(var: WithdrawFromEscrowAuxCall) -> Self {
            BridgeEscrowCalls::WithdrawFromEscrowAux(var)
        }
    }
    impl ::std::convert::From<WithdrawFromEscrowThisCall> for BridgeEscrowCalls {
        fn from(var: WithdrawFromEscrowThisCall) -> Self {
            BridgeEscrowCalls::WithdrawFromEscrowThis(var)
        }
    }
    #[doc = "`AccountInfo(address,bytes16,address,bytes16,uint64,bytes16,uint256,bool)`"]
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthAbiType)]
    pub struct AccountInfo {
        pub sender_this: ethers::core::types::Address,
        pub sender_other: [u8; 16],
        pub receiver_this: ethers::core::types::Address,
        pub receiver_other: [u8; 16],
        pub balance: u64,
        pub transfer_id: [u8; 16],
        pub locked_idx: ethers::core::types::U256,
        pub is_closed: bool,
    }
}
