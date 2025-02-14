// src/components/ProposalsTable.tsx

import React from "react";
import { Proposal } from "../api";

type ProposalsTableProps = {
  proposals: Proposal[];
  onSelectProposal: (id: string) => void;
};

const ProposalsTable: React.FC<ProposalsTableProps> = ({ proposals, onSelectProposal }) => {
  return (
    <table style={{ width: "100%", borderCollapse: "collapse" }}>
      <thead>
        <tr style={{ textAlign: "left", borderBottom: "1px solid #ccc" }}>
          <th>ID</th>
          <th>Title</th>
          <th>Author</th>
          <th>Start</th>
          <th>State</th>
        </tr>
      </thead>
      <tbody>
        {proposals.map((proposal) => (
          <tr
            key={proposal.id}
            onClick={() => onSelectProposal(proposal.id)}
            style={{ cursor: "pointer", borderBottom: "1px solid #eee" }}
          >
            <td>{proposal.id}</td>
            <td>{proposal.title || "No title"}</td>
            <td>{proposal.author || "-"}</td>
            <td>
              {proposal.start
                ? new Date(proposal.start * 1000).toLocaleString()
                : "-"}
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default ProposalsTable;
