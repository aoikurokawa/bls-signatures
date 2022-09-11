import type { AnchorTypes } from "@saberhq/anchor-contrib";

import type { VotingIDL } from "../idls/voting";

export * from "../idls/voting";

export type VotingType = AnchorTypes<
  VotingIDL,
  {
    pollCount: PollCountData;
    pollData: PollData;
    voteData: VoteData;
  },
  {}
>;

type Accounts = VotingType["Accounts"];
export type PollCountData = Accounts["PollCount"];
export type PollData = Accounts["Poll"];
export type VoteData = Accounts["Vote"];

export type VotingProgram = VotingType["Program"];
