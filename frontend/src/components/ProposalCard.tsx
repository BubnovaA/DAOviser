/* eslint-disable @typescript-eslint/no-explicit-any */
import React, { useState } from "react";
import { Proposal } from "../api";

type ProposalCardProps = {
  proposal: Proposal;
  onSelect: () => void;
};

const ProposalCard: React.FC<ProposalCardProps> = ({ proposal, onSelect }) => {
  const [expanded, setExpanded] = useState(false);
  const toggleExpanded = () => setExpanded((prev) => !prev);

  const formatDate = (timestamp?: number) => {
    if (!timestamp) return "-";
    return new Date(timestamp * 1000).toLocaleString();
  };

  return (
    <div
      onClick={onSelect}
      style={{
        border: "1px solid #ddd",
        borderRadius: "8px",
        padding: "1rem",
        marginBottom: "1rem",
        boxShadow: "0 2px 4px rgba(0,0,0,0.05)",
        cursor: "pointer",
      }}
    >
      <h2>{proposal.title}</h2>
      <div style={{ fontSize: "0.9rem", color: "#666" }}>
        <span>
          <strong>Start:</strong> {formatDate(proposal.start)}
        </span>{" "}
        -{" "}
        <span>
          <strong>End:</strong> {formatDate(proposal.end)}
        </span>
      </div>
      <div style={{ marginTop: "1rem" }}>
        <p
          style={{
            maxHeight: expanded ? "none" : "100px",
            overflow: "hidden",
            transition: "max-height 0.3s ease",
            whiteSpace: "pre-wrap",
          }}
        >
          {proposal.body || "No description available."}
        </p>
        {proposal.body && proposal.body.length > 300 && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              toggleExpanded();
            }}
            style={{ fontSize: "0.8rem", marginTop: "0.5rem" }}
          >
            {expanded ? "Show less" : "Read more"}
          </button>
        )}
      </div>
      <div style={{ marginTop: "1rem" }}>
        <strong>Choices:</strong>
        <ul>
          {Array.isArray(proposal.choices)
            ? proposal.choices.map((choice: any, index: number) => (
                <li key={index}>{choice}</li>
              ))
            : proposal.choices && <li>{proposal.choices}</li>}
        </ul>
      </div>
    </div>
  );
};

export default ProposalCard;
