import React from "react";
import { Proposal } from "../api";
import ProposalCard from "./ProposalCard";

type ProposalsListProps = {
  proposals: Proposal[];
  onSelectProposal: (id: string) => void;
};

const ProposalsList: React.FC<ProposalsListProps> = ({ proposals, onSelectProposal }) => {
  if (proposals.length === 0) {
    return <p>No proposals found.</p>;
  }

  return (
    <div>
      {proposals.map((proposal) => (
        <ProposalCard
          key={proposal.id}
          proposal={proposal}
          onSelect={() => onSelectProposal(proposal.id)}
        />
      ))}
    </div>
  );
};

export default ProposalsList;
