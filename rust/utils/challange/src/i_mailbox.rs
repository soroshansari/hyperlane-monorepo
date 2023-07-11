pub use i_mailbox::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_mailbox {
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
    #[doc = "IMailbox was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint32\",\n        \"name\": \"destination\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"recipient\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes\",\n        \"name\": \"message\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"Dispatch\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"DispatchId\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint32\",\n        \"name\": \"origin\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"sender\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"Process\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"ProcessId\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"count\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"\",\n        \"type\": \"uint32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"defaultIsm\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IInterchainSecurityModule\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"messageId\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"delivered\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"_destinationDomain\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_recipientAddress\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"_messageBody\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"dispatch\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"latestCheckpoint\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"\",\n        \"type\": \"uint32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"localDomain\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"\",\n        \"type\": \"uint32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"_metadata\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"_message\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"process\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_recipient\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"recipientIsm\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IInterchainSecurityModule\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"root\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IMAILBOX_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IMailbox<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IMailbox<M> {
        fn clone(&self) -> Self {
            IMailbox(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IMailbox<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IMailbox<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IMailbox))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IMailbox<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IMAILBOX_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `count` (0x06661abd) function"]
        pub fn count(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([6, 102, 26, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `defaultIsm` (0x6e5f516e) function"]
        pub fn default_ism(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([110, 95, 81, 110], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delivered` (0xe495f1d4) function"]
        pub fn delivered(
            &self,
            message_id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([228, 149, 241, 212], message_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dispatch` (0xfa31de01) function"]
        pub fn dispatch(
            &self,
            destination_domain: u32,
            recipient_address: [u8; 32],
            message_body: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [250, 49, 222, 1],
                    (destination_domain, recipient_address, message_body),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestCheckpoint` (0x907c0f92) function"]
        pub fn latest_checkpoint(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ([u8; 32], u32)> {
            self.0
                .method_hash([144, 124, 15, 146], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `localDomain` (0x8d3638f4) function"]
        pub fn local_domain(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([141, 54, 56, 244], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `process` (0x7c39d130) function"]
        pub fn process(
            &self,
            metadata: ethers::core::types::Bytes,
            message: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 57, 209, 48], (metadata, message))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `recipientIsm` (0xe70f48ac) function"]
        pub fn recipient_ism(
            &self,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([231, 15, 72, 172], recipient)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `root` (0xebf0c717) function"]
        pub fn root(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([235, 240, 199, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Dispatch` event"]
        pub fn dispatch_filter(&self) -> ethers::contract::builders::Event<M, DispatchFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DispatchId` event"]
        pub fn dispatch_id_filter(&self) -> ethers::contract::builders::Event<M, DispatchIdFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Process` event"]
        pub fn process_filter(&self) -> ethers::contract::builders::Event<M, ProcessFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProcessId` event"]
        pub fn process_id_filter(&self) -> ethers::contract::builders::Event<M, ProcessIdFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IMailboxEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IMailbox<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Dispatch", abi = "Dispatch(address,uint32,bytes32,bytes)")]
    pub struct DispatchFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub destination: u32,
        #[ethevent(indexed)]
        pub recipient: [u8; 32],
        pub message: ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "DispatchId", abi = "DispatchId(bytes32)")]
    pub struct DispatchIdFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Process", abi = "Process(uint32,bytes32,address)")]
    pub struct ProcessFilter {
        #[ethevent(indexed)]
        pub origin: u32,
        #[ethevent(indexed)]
        pub sender: [u8; 32],
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ProcessId", abi = "ProcessId(bytes32)")]
    pub struct ProcessIdFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IMailboxEvents {
        DispatchFilter(DispatchFilter),
        DispatchIdFilter(DispatchIdFilter),
        ProcessFilter(ProcessFilter),
        ProcessIdFilter(ProcessIdFilter),
    }
    impl ethers::contract::EthLogDecode for IMailboxEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DispatchFilter::decode_log(log) {
                return Ok(IMailboxEvents::DispatchFilter(decoded));
            }
            if let Ok(decoded) = DispatchIdFilter::decode_log(log) {
                return Ok(IMailboxEvents::DispatchIdFilter(decoded));
            }
            if let Ok(decoded) = ProcessFilter::decode_log(log) {
                return Ok(IMailboxEvents::ProcessFilter(decoded));
            }
            if let Ok(decoded) = ProcessIdFilter::decode_log(log) {
                return Ok(IMailboxEvents::ProcessIdFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IMailboxEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMailboxEvents::DispatchFilter(element) => element.fmt(f),
                IMailboxEvents::DispatchIdFilter(element) => element.fmt(f),
                IMailboxEvents::ProcessFilter(element) => element.fmt(f),
                IMailboxEvents::ProcessIdFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `count` function with signature `count()` and selector `[6, 102, 26, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "count", abi = "count()")]
    pub struct CountCall;
    #[doc = "Container type for all input parameters for the `defaultIsm` function with signature `defaultIsm()` and selector `[110, 95, 81, 110]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "defaultIsm", abi = "defaultIsm()")]
    pub struct DefaultIsmCall;
    #[doc = "Container type for all input parameters for the `delivered` function with signature `delivered(bytes32)` and selector `[228, 149, 241, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "delivered", abi = "delivered(bytes32)")]
    pub struct DeliveredCall {
        pub message_id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `dispatch` function with signature `dispatch(uint32,bytes32,bytes)` and selector `[250, 49, 222, 1]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "dispatch", abi = "dispatch(uint32,bytes32,bytes)")]
    pub struct DispatchCall {
        pub destination_domain: u32,
        pub recipient_address: [u8; 32],
        pub message_body: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `latestCheckpoint` function with signature `latestCheckpoint()` and selector `[144, 124, 15, 146]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "latestCheckpoint", abi = "latestCheckpoint()")]
    pub struct LatestCheckpointCall;
    #[doc = "Container type for all input parameters for the `localDomain` function with signature `localDomain()` and selector `[141, 54, 56, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "localDomain", abi = "localDomain()")]
    pub struct LocalDomainCall;
    #[doc = "Container type for all input parameters for the `process` function with signature `process(bytes,bytes)` and selector `[124, 57, 209, 48]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "process", abi = "process(bytes,bytes)")]
    pub struct ProcessCall {
        pub metadata: ethers::core::types::Bytes,
        pub message: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `recipientIsm` function with signature `recipientIsm(address)` and selector `[231, 15, 72, 172]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "recipientIsm", abi = "recipientIsm(address)")]
    pub struct RecipientIsmCall {
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `root` function with signature `root()` and selector `[235, 240, 199, 23]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "root", abi = "root()")]
    pub struct RootCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IMailboxCalls {
        Count(CountCall),
        DefaultIsm(DefaultIsmCall),
        Delivered(DeliveredCall),
        Dispatch(DispatchCall),
        LatestCheckpoint(LatestCheckpointCall),
        LocalDomain(LocalDomainCall),
        Process(ProcessCall),
        RecipientIsm(RecipientIsmCall),
        Root(RootCall),
    }
    impl ethers::core::abi::AbiDecode for IMailboxCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <CountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMailboxCalls::Count(decoded));
            }
            if let Ok(decoded) =
                <DefaultIsmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMailboxCalls::DefaultIsm(decoded));
            }
            if let Ok(decoded) =
                <DeliveredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMailboxCalls::Delivered(decoded));
            }
            if let Ok(decoded) =
                <DispatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMailboxCalls::Dispatch(decoded));
            }
            if let Ok(decoded) =
                <LatestCheckpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMailboxCalls::LatestCheckpoint(decoded));
            }
            if let Ok(decoded) =
                <LocalDomainCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMailboxCalls::LocalDomain(decoded));
            }
            if let Ok(decoded) =
                <ProcessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMailboxCalls::Process(decoded));
            }
            if let Ok(decoded) =
                <RecipientIsmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMailboxCalls::RecipientIsm(decoded));
            }
            if let Ok(decoded) = <RootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IMailboxCalls::Root(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IMailboxCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IMailboxCalls::Count(element) => element.encode(),
                IMailboxCalls::DefaultIsm(element) => element.encode(),
                IMailboxCalls::Delivered(element) => element.encode(),
                IMailboxCalls::Dispatch(element) => element.encode(),
                IMailboxCalls::LatestCheckpoint(element) => element.encode(),
                IMailboxCalls::LocalDomain(element) => element.encode(),
                IMailboxCalls::Process(element) => element.encode(),
                IMailboxCalls::RecipientIsm(element) => element.encode(),
                IMailboxCalls::Root(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IMailboxCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMailboxCalls::Count(element) => element.fmt(f),
                IMailboxCalls::DefaultIsm(element) => element.fmt(f),
                IMailboxCalls::Delivered(element) => element.fmt(f),
                IMailboxCalls::Dispatch(element) => element.fmt(f),
                IMailboxCalls::LatestCheckpoint(element) => element.fmt(f),
                IMailboxCalls::LocalDomain(element) => element.fmt(f),
                IMailboxCalls::Process(element) => element.fmt(f),
                IMailboxCalls::RecipientIsm(element) => element.fmt(f),
                IMailboxCalls::Root(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CountCall> for IMailboxCalls {
        fn from(var: CountCall) -> Self {
            IMailboxCalls::Count(var)
        }
    }
    impl ::std::convert::From<DefaultIsmCall> for IMailboxCalls {
        fn from(var: DefaultIsmCall) -> Self {
            IMailboxCalls::DefaultIsm(var)
        }
    }
    impl ::std::convert::From<DeliveredCall> for IMailboxCalls {
        fn from(var: DeliveredCall) -> Self {
            IMailboxCalls::Delivered(var)
        }
    }
    impl ::std::convert::From<DispatchCall> for IMailboxCalls {
        fn from(var: DispatchCall) -> Self {
            IMailboxCalls::Dispatch(var)
        }
    }
    impl ::std::convert::From<LatestCheckpointCall> for IMailboxCalls {
        fn from(var: LatestCheckpointCall) -> Self {
            IMailboxCalls::LatestCheckpoint(var)
        }
    }
    impl ::std::convert::From<LocalDomainCall> for IMailboxCalls {
        fn from(var: LocalDomainCall) -> Self {
            IMailboxCalls::LocalDomain(var)
        }
    }
    impl ::std::convert::From<ProcessCall> for IMailboxCalls {
        fn from(var: ProcessCall) -> Self {
            IMailboxCalls::Process(var)
        }
    }
    impl ::std::convert::From<RecipientIsmCall> for IMailboxCalls {
        fn from(var: RecipientIsmCall) -> Self {
            IMailboxCalls::RecipientIsm(var)
        }
    }
    impl ::std::convert::From<RootCall> for IMailboxCalls {
        fn from(var: RootCall) -> Self {
            IMailboxCalls::Root(var)
        }
    }
    #[doc = "Container type for all return fields from the `count` function with signature `count()` and selector `[6, 102, 26, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CountReturn(pub u32);
    #[doc = "Container type for all return fields from the `defaultIsm` function with signature `defaultIsm()` and selector `[110, 95, 81, 110]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DefaultIsmReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `delivered` function with signature `delivered(bytes32)` and selector `[228, 149, 241, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DeliveredReturn(pub bool);
    #[doc = "Container type for all return fields from the `dispatch` function with signature `dispatch(uint32,bytes32,bytes)` and selector `[250, 49, 222, 1]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DispatchReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `latestCheckpoint` function with signature `latestCheckpoint()` and selector `[144, 124, 15, 146]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LatestCheckpointReturn(pub [u8; 32], pub u32);
    #[doc = "Container type for all return fields from the `localDomain` function with signature `localDomain()` and selector `[141, 54, 56, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LocalDomainReturn(pub u32);
    #[doc = "Container type for all return fields from the `recipientIsm` function with signature `recipientIsm(address)` and selector `[231, 15, 72, 172]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RecipientIsmReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `root` function with signature `root()` and selector `[235, 240, 199, 23]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RootReturn(pub [u8; 32]);
}
