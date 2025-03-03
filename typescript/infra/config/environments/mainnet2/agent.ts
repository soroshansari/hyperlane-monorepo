import {
  AgentConnectionType,
  chainMetadata,
  hyperlaneEnvironments,
  objMap,
} from '@hyperlane-xyz/sdk';

import {
  GasPaymentEnforcementPolicyType,
  RootAgentConfig,
  routerMatchingList,
} from '../../../src/config';
import { GasPaymentEnforcementConfig } from '../../../src/config/agent/relayer';
import { ALL_KEY_ROLES, Role } from '../../../src/roles';
import { Contexts } from '../../contexts';

import { chainNames, environment } from './chains';
import { helloWorld } from './helloworld';
import { validators } from './validators';

const releaseCandidateHelloworldMatchingList = routerMatchingList(
  helloWorld[Contexts.ReleaseCandidate].addresses,
);

const interchainQueryRouters = objMap(
  hyperlaneEnvironments.mainnet,
  (_, addresses) => {
    return {
      router: addresses.interchainQueryRouter,
    };
  },
);

const interchainQueriesMatchingList = routerMatchingList(
  interchainQueryRouters,
);

const repo = 'gcr.io/abacus-labs-dev/hyperlane-agent';

const contextBase = {
  namespace: environment,
  runEnv: environment,
  contextChainNames: chainNames,
  environmentChainNames: chainNames,
  aws: {
    region: 'us-east-1',
  },
} as const;

const gasPaymentEnforcement: GasPaymentEnforcementConfig[] = [
  {
    type: GasPaymentEnforcementPolicyType.None,
    // To continue relaying interchain query callbacks, we whitelist
    // all messages between interchain query routers.
    // This whitelist will become more strict with
    // https://github.com/hyperlane-xyz/hyperlane-monorepo/issues/1605
    matchingList: interchainQueriesMatchingList,
  },
  {
    type: GasPaymentEnforcementPolicyType.OnChainFeeQuoting,
  },
];

const hyperlane: RootAgentConfig = {
  ...contextBase,
  context: Contexts.Hyperlane,
  rolesWithKeys: ALL_KEY_ROLES,
  relayer: {
    connectionType: AgentConnectionType.HttpFallback,
    docker: {
      repo,
      tag: '2deb9b8-20230602-205342',
    },
    blacklist: [
      ...releaseCandidateHelloworldMatchingList,
      {
        originDomain: 137,
        recipientAddress: '0xBC3cFeca7Df5A45d61BC60E7898E63670e1654aE',
      },
    ],
    gasPaymentEnforcement,
  },
  validators: {
    docker: {
      repo,
      tag: '497db63-20230614-174455',
    },
    connectionType: AgentConnectionType.HttpQuorum,
    chains: validators,
  },
  scraper: {
    connectionType: AgentConnectionType.HttpFallback,
    docker: {
      repo,
      tag: 'aaddba7-20230620-154941',
    },
  },
};

const releaseCandidate: RootAgentConfig = {
  ...contextBase,
  context: Contexts.ReleaseCandidate,
  rolesWithKeys: [Role.Relayer, Role.Kathy],
  relayer: {
    connectionType: AgentConnectionType.HttpFallback,
    docker: {
      repo,
      tag: '2deb9b8-20230602-205342',
    },
    whitelist: releaseCandidateHelloworldMatchingList,
    gasPaymentEnforcement,
    transactionGasLimit: 750000,
    // Skipping arbitrum because the gas price estimates are inclusive of L1
    // fees which leads to wildly off predictions.
    skipTransactionGasLimitFor: [chainMetadata.arbitrum.chainId],
  },
};

export const agents = {
  [Contexts.Hyperlane]: hyperlane,
  [Contexts.ReleaseCandidate]: releaseCandidate,
};
